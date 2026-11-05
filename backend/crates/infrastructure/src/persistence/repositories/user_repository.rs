use async_trait::async_trait;
use library_domain::common::{Page, PageRequest};
use library_domain::error::RepositoryResult;
use library_domain::users::{AuthorSummary, User, UserId, UserRepository};

use super::{contains_pattern, db_error};
use crate::metrics::record_user_created;
use crate::persistence::Db;
use crate::persistence::rows::{AuthorRow, USER_COLUMNS, UserRow};

pub struct SqlxUserRepository<'a> {
    db: Db<'a>,
}

impl<'a> SqlxUserRepository<'a> {
    pub fn new(db: Db<'a>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository<'_> {
    async fn find_by_id(&self, id: UserId) -> RepositoryResult<Option<User>> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let row: Option<UserRow> = sqlx::query_as(&format!("select {USER_COLUMNS} from web_user where user_id = $1"))
            .bind(id.into_inner())
            .fetch_optional(&mut *conn)
            .await
            .map_err(db_error)?;
        Ok(row.map(User::from))
    }

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let row: Option<UserRow> = sqlx::query_as(&format!("select {USER_COLUMNS} from web_user where email = $1"))
            .bind(email)
            .fetch_optional(&mut *conn)
            .await
            .map_err(db_error)?;
        Ok(row.map(User::from))
    }

    async fn insert(&self, user: &User) -> RepositoryResult<bool> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let result = sqlx::query(
            r#"insert into web_user (user_id, email, password_hash, name, bio, roles, is_email_verified,
                   verification_code, verification_code_issued_at, verification_code_expires_at, created_at, updated_at)
               values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
               on conflict (email) do nothing"#,
        )
        .bind(user.id().into_inner())
        .bind(user.email().as_str())
        .bind(user.password_hash().as_str())
        .bind(user.name())
        .bind(user.bio())
        .bind(user.roles())
        .bind(user.is_email_verified())
        .bind(user.verification_code().map(|code| code.as_str()))
        .bind(user.code_issued_at())
        .bind(user.code_expires_at())
        .bind(user.audit().created())
        .bind(user.audit().last_modified())
        .execute(&mut *conn)
        .await
        .map_err(db_error)?;

        let inserted = result.rows_affected() > 0;
        if inserted {
            record_user_created();
        }
        Ok(inserted)
    }

    async fn update(&self, user: &User) -> RepositoryResult<bool> {
        let mut conn = self.db.conn().await.map_err(db_error)?;
        let result = sqlx::query(
            r#"update web_user
               set name = $2, bio = $3, roles = $4, is_email_verified = $5, verification_code = $6,
                   verification_code_issued_at = $7, verification_code_expires_at = $8, updated_at = $9
               where user_id = $1"#,
        )
        .bind(user.id().into_inner())
        .bind(user.name())
        .bind(user.bio())
        .bind(user.roles())
        .bind(user.is_email_verified())
        .bind(user.verification_code().map(|code| code.as_str()))
        .bind(user.code_issued_at())
        .bind(user.code_expires_at())
        .bind(user.audit().last_modified())
        .execute(&mut *conn)
        .await
        .map_err(db_error)?;
        Ok(result.rows_affected() > 0)
    }

    async fn list_authors(&self, search: Option<String>, page: PageRequest) -> RepositoryResult<Page<AuthorSummary>> {
        let pattern = contains_pattern(search);
        let mut conn = self.db.conn().await.map_err(db_error)?;

        let total: i64 = sqlx::query_scalar(
            r#"select count(distinct u.user_id)::int8
               from web_user u inner join material m on m.user_id = u.user_id
               where m.published_at is not null and ($1::text is null or u.name ilike $1)"#,
        )
        .bind(&pattern)
        .fetch_one(&mut *conn)
        .await
        .map_err(db_error)?;

        let rows: Vec<AuthorRow> = sqlx::query_as(
            r#"select u.user_id, u.name, u.bio, u.is_email_verified, count(m.material_id)::int8 as materials_count
               from web_user u inner join material m on m.user_id = u.user_id
               where m.published_at is not null and ($1::text is null or u.name ilike $1)
               group by u.user_id, u.name, u.bio, u.is_email_verified
               order by u.name, u.user_id
               limit $2 offset $3"#,
        )
        .bind(&pattern)
        .bind(page.limit())
        .bind(page.offset())
        .fetch_all(&mut *conn)
        .await
        .map_err(db_error)?;

        Ok(Page {
            items: rows.into_iter().map(AuthorSummary::from).collect(),
            total,
        })
    }
}
