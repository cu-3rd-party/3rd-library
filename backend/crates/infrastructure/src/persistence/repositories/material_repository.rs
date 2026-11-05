use async_trait::async_trait;
use library_domain::common::{Page, PageRequest};
use library_domain::error::RepositoryResult;
use library_domain::materials::{Material, MaterialFile, MaterialId, MaterialRepository, MaterialSummary};
use library_domain::users::UserId;
use uuid::Uuid;

use super::{contains_pattern, db_error};
use crate::persistence::Db;
use crate::persistence::rows::{FILE_COLUMNS, FileRow, MATERIAL_SUMMARY_SELECT, MaterialSummaryRow};

pub struct SqlxMaterialRepository<'a> {
    db: Db<'a>,
}

impl<'a> SqlxMaterialRepository<'a> {
    pub fn new(db: Db<'a>) -> Self {
        Self { db }
    }

    /// Опубликованные материалы, новые сверху: `$1` — автор (или null), `$2` — шаблон названия (или null).
    async fn list(
        &self,
        author: Option<Uuid>,
        title_pattern: Option<String>,
        page: PageRequest,
    ) -> RepositoryResult<Page<MaterialSummary>> {
        const FILTER: &str = "m.published_at is not null \
             and ($1::uuid is null or m.user_id = $1) and ($2::text is null or m.title ilike $2)";
        let mut conn = self.db.conn().await.map_err(db_error)?;

        let total: i64 = sqlx::query_scalar(&format!("select count(*)::int8 from material m where {FILTER}"))
            .bind(author)
            .bind(&title_pattern)
            .fetch_one(&mut *conn)
            .await
            .map_err(db_error)?;

        let rows: Vec<MaterialSummaryRow> = sqlx::query_as(&format!(
            "{MATERIAL_SUMMARY_SELECT} where {FILTER} order by m.published_at desc, m.material_id limit $3 offset $4"
        ))
        .bind(author)
        .bind(&title_pattern)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(&mut *conn)
        .await
        .map_err(db_error)?;

        Ok(Page {
            items: rows.into_iter().map(MaterialSummary::from).collect(),
            total,
        })
    }
}

#[async_trait]
impl MaterialRepository for SqlxMaterialRepository<'_> {
    async fn list_published(
        &self,
        search: Option<String>,
        page: PageRequest,
    ) -> RepositoryResult<Page<MaterialSummary>> {
        self.list(None, contains_pattern(search), page).await
    }

    async fn list_published_by_author(
        &self,
        author: UserId,
        page: PageRequest,
    ) -> RepositoryResult<Page<MaterialSummary>> {
        self.list(Some(author.into_inner()), None, page).await
    }

    async fn find_by_id(&self, id: MaterialId) -> RepositoryResult<Option<MaterialSummary>> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let row: Option<MaterialSummaryRow> =
            sqlx::query_as(&format!("{MATERIAL_SUMMARY_SELECT} where m.material_id = $1"))
                .bind(id.into_inner())
                .fetch_optional(&mut *conn)
                .await
                .map_err(db_error)?;
        Ok(row.map(MaterialSummary::from))
    }

    async fn files(&self, id: MaterialId) -> RepositoryResult<Vec<MaterialFile>> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let rows: Vec<FileRow> = sqlx::query_as(&format!(
            "select {FILE_COLUMNS} from material_file mf \
             inner join material_file_rel mfr on mf.file_id = mfr.file_id \
             where mfr.material_id = $1 order by mf.created_at, mf.file_id"
        ))
        .bind(id.into_inner())
        .fetch_all(&mut *conn)
        .await
        .map_err(db_error)?;
        Ok(rows.into_iter().map(MaterialFile::from).collect())
    }

    async fn insert(&self, material: &Material) -> RepositoryResult<()> {
        let info = material.info();
        let mut conn = self.db.begin().await.map_err(db_error)?;

        sqlx::query(
            r#"insert into material (material_id, user_id, title, description, courses, subjects, type, difficulty,
                   published_at, created_at, updated_at)
               values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
        )
        .bind(material.id().into_inner())
        .bind(material.author().into_inner())
        .bind(&info.title)
        .bind(&info.description)
        .bind(&info.courses)
        .bind(&info.subjects)
        .bind(&info.kind)
        .bind(&info.difficulty)
        .bind(material.published_at())
        .bind(material.audit().created())
        .bind(material.audit().last_modified())
        .execute(&mut *conn)
        .await
        .map_err(db_error)?;

        let file_ids: Vec<Uuid> = material.file_ids().iter().map(|id| id.into_inner()).collect();
        sqlx::query("insert into material_file_rel (material_id, file_id) select $1, unnest($2::uuid[])")
            .bind(material.id().into_inner())
            .bind(&file_ids)
            .execute(&mut *conn)
            .await
            .map_err(db_error)?;

        conn.commit().await.map_err(db_error)
    }
}
