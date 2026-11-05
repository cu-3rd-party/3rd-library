use std::collections::HashMap;

use async_trait::async_trait;
use library_domain::common::{Page, PageRequest};
use library_domain::error::RepositoryResult;
use library_domain::materials::MaterialFile;
use library_domain::submissions::{StatusCounters, Submission, SubmissionId, SubmissionRepository, SubmissionStatus};
use library_domain::users::UserId;
use sqlx::PgConnection;
use uuid::Uuid;

use super::db_error;
use crate::persistence::Db;
use crate::persistence::rows::{FILE_COLUMNS, LinkedFileRow, SUBMISSION_COLUMNS, SubmissionRow};

pub struct SqlxSubmissionRepository<'a> {
    db: Db<'a>,
}

impl<'a> SqlxSubmissionRepository<'a> {
    pub fn new(db: Db<'a>) -> Self {
        Self { db }
    }

    /// Страница заявок: `$1` — автор (или null), `$2` — статус (или null).
    async fn list(
        &self,
        author: Option<Uuid>,
        status: Option<SubmissionStatus>,
        newest_first: bool,
        page: PageRequest,
    ) -> RepositoryResult<Page<Submission>> {
        const FILTER: &str = "($1::uuid is null or s.user_id = $1) and ($2::text is null or s.status = $2)";
        let order = if newest_first { "desc" } else { "asc" };
        let status = status.map(SubmissionStatus::as_str);
        let mut conn = self.db.conn().await.map_err(db_error)?;

        let total: i64 = sqlx::query_scalar(&format!("select count(*)::int8 from submission s where {FILTER}"))
            .bind(author)
            .bind(status)
            .fetch_one(&mut *conn)
            .await
            .map_err(db_error)?;

        let rows: Vec<SubmissionRow> = sqlx::query_as(&format!(
            "select {SUBMISSION_COLUMNS} from submission s where {FILTER} \
             order by s.created_at {order}, s.submission_id limit $3 offset $4"
        ))
        .bind(author)
        .bind(status)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(&mut *conn)
        .await
        .map_err(db_error)?;

        let ids: Vec<Uuid> = rows.iter().map(|row| row.submission_id).collect();
        let mut files = files_of(&mut conn, &ids).await?;
        let items = rows
            .into_iter()
            .map(|row| {
                let files = files.remove(&row.submission_id).unwrap_or_default();
                row.into_domain(files)
            })
            .collect::<RepositoryResult<_>>()?;
        Ok(Page { items, total })
    }
}

/// Файлы заявок `ids` одним запросом, по заявкам.
async fn files_of(conn: &mut PgConnection, ids: &[Uuid]) -> RepositoryResult<HashMap<Uuid, Vec<MaterialFile>>> {
    let rows: Vec<LinkedFileRow> = sqlx::query_as(&format!(
        "select sfr.submission_id, {FILE_COLUMNS} from material_file mf \
         inner join submission_file_rel sfr on mf.file_id = sfr.file_id \
         where sfr.submission_id = any($1) order by mf.created_at, mf.file_id"
    ))
    .bind(ids)
    .fetch_all(conn)
    .await
    .map_err(db_error)?;

    let mut files: HashMap<Uuid, Vec<MaterialFile>> = HashMap::new();
    for row in rows {
        files.entry(row.submission_id).or_default().push(row.file.into());
    }
    Ok(files)
}

/// Новые файлы заявки и связи с ними; связи с файлами, которых в заявке больше нет, удаляются.
async fn sync_files(conn: &mut PgConnection, submission: &Submission) -> RepositoryResult<()> {
    let submission_id = submission.id().into_inner();
    for file in submission.files() {
        sqlx::query(
            r#"insert into material_file (file_id, user_id, name, size_bytes, extension, mime_type, storage_key, created_at)
               values ($1, $2, $3, $4, $5, $6, $7, $8)
               on conflict (file_id) do nothing"#,
        )
        .bind(file.id.into_inner())
        .bind(file.owner.into_inner())
        .bind(&file.name)
        .bind(file.size_bytes)
        .bind(&file.extension)
        .bind(&file.mime_type)
        .bind(&file.storage_key)
        .bind(file.created)
        .execute(&mut *conn)
        .await
        .map_err(db_error)?;
    }

    let file_ids: Vec<Uuid> = submission.files().iter().map(|file| file.id.into_inner()).collect();
    sqlx::query("delete from submission_file_rel where submission_id = $1 and not (file_id = any($2))")
        .bind(submission_id)
        .bind(&file_ids)
        .execute(&mut *conn)
        .await
        .map_err(db_error)?;
    sqlx::query(
        "insert into submission_file_rel (submission_id, file_id) select $1, unnest($2::uuid[]) \
         on conflict do nothing",
    )
    .bind(submission_id)
    .bind(&file_ids)
    .execute(&mut *conn)
    .await
    .map_err(db_error)?;
    Ok(())
}

#[async_trait]
impl SubmissionRepository for SqlxSubmissionRepository<'_> {
    async fn find_by_id(&self, id: SubmissionId) -> RepositoryResult<Option<Submission>> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let row: Option<SubmissionRow> = sqlx::query_as(&format!(
            "select {SUBMISSION_COLUMNS} from submission s where s.submission_id = $1"
        ))
        .bind(id.into_inner())
        .fetch_optional(&mut *conn)
        .await
        .map_err(db_error)?;
        let Some(row) = row else {
            return Ok(None);
        };

        let files = files_of(&mut conn, &[row.submission_id])
            .await?
            .remove(&row.submission_id)
            .unwrap_or_default();
        row.into_domain(files).map(Some)
    }

    async fn list_by_author(
        &self,
        author: UserId,
        status: Option<SubmissionStatus>,
        page: PageRequest,
    ) -> RepositoryResult<Page<Submission>> {
        self.list(Some(author.into_inner()), status, true, page).await
    }

    async fn list_for_moderation(
        &self,
        status: Option<SubmissionStatus>,
        page: PageRequest,
    ) -> RepositoryResult<Page<Submission>> {
        self.list(None, status, false, page).await
    }

    async fn count_by_status(&self) -> RepositoryResult<StatusCounters> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let (all, draft, pending_review, rejected, approved): (i64, i64, i64, i64, i64) = sqlx::query_as(
            r#"select count(*)::int8,
                      count(*) filter (where status = 'draft')::int8,
                      count(*) filter (where status = 'pending_review')::int8,
                      count(*) filter (where status = 'rejected')::int8,
                      count(*) filter (where status = 'approved')::int8
               from submission"#,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(db_error)?;
        Ok(StatusCounters {
            all,
            draft,
            pending_review,
            rejected,
            approved,
        })
    }

    async fn insert(&self, submission: &Submission) -> RepositoryResult<()> {
        let info = submission.info();
        let mut conn = self.db.begin().await.map_err(db_error)?;

        sqlx::query(
            r#"insert into submission (submission_id, user_id, material_id, title, description, courses, subjects,
                   type, difficulty, status, moderator_id, moderator_comment, published_at, created_at, updated_at,
                   submitted_at, reviewed_at)
               values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"#,
        )
        .bind(submission.id().into_inner())
        .bind(submission.author().into_inner())
        .bind(submission.material_id().map(|id| id.into_inner()))
        .bind(&info.title)
        .bind(&info.description)
        .bind(&info.courses)
        .bind(&info.subjects)
        .bind(&info.kind)
        .bind(&info.difficulty)
        .bind(submission.status().as_str())
        .bind(submission.moderator_id().map(|id| id.into_inner()))
        .bind(submission.moderator_comment())
        .bind(submission.published_at())
        .bind(submission.audit().created())
        .bind(submission.audit().last_modified())
        .bind(submission.submitted_at())
        .bind(submission.reviewed_at())
        .execute(&mut *conn)
        .await
        .map_err(db_error)?;

        sync_files(&mut conn, submission).await?;
        conn.commit().await.map_err(db_error)
    }

    async fn update(&self, submission: &Submission) -> RepositoryResult<bool> {
        let info = submission.info();
        let mut conn = self.db.begin().await.map_err(db_error)?;

        let result = sqlx::query(
            r#"update submission
               set material_id = $2, title = $3, description = $4, courses = $5, subjects = $6, type = $7,
                   difficulty = $8, status = $9, moderator_id = $10, moderator_comment = $11, published_at = $12,
                   updated_at = $13, submitted_at = $14, reviewed_at = $15
               where submission_id = $1"#,
        )
        .bind(submission.id().into_inner())
        .bind(submission.material_id().map(|id| id.into_inner()))
        .bind(&info.title)
        .bind(&info.description)
        .bind(&info.courses)
        .bind(&info.subjects)
        .bind(&info.kind)
        .bind(&info.difficulty)
        .bind(submission.status().as_str())
        .bind(submission.moderator_id().map(|id| id.into_inner()))
        .bind(submission.moderator_comment())
        .bind(submission.published_at())
        .bind(submission.audit().last_modified())
        .bind(submission.submitted_at())
        .bind(submission.reviewed_at())
        .execute(&mut *conn)
        .await
        .map_err(db_error)?;
        if result.rows_affected() == 0 {
            return Ok(false);
        }

        sync_files(&mut conn, submission).await?;
        conn.commit().await.map_err(db_error)?;
        Ok(true)
    }
}
