use async_trait::async_trait;
use chrono::{DateTime, TimeDelta, Utc};
use library_application::auth::Session;
use library_application::ports::{ExternalError, ExternalResult, SessionStore};
use sqlx::PgPool;
use uuid::Uuid;

use crate::metrics::observe_db_query;

/// Сессии в таблице `session`; просроченные не находятся и удаляются при следующем сохранении.
#[derive(Clone)]
pub struct PgSessionStore {
    db: PgPool,
}

impl PgSessionStore {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SessionStore for PgSessionStore {
    async fn save(&self, session: &Session, ttl: TimeDelta) -> ExternalResult<()> {
        observe_db_query();
        sqlx::query(
            r#"insert into session (session_id, user_id, verified, expires_at) values ($1, $2, $3, $4)
               on conflict (session_id) do update set verified = excluded.verified, expires_at = excluded.expires_at"#,
        )
        .bind(session.session_id)
        .bind(session.user_id)
        .bind(session.verified)
        .bind(Utc::now() + ttl)
        .execute(&self.db)
        .await
        .map_err(ExternalError::new)?;

        observe_db_query();
        sqlx::query("delete from session where expires_at < now()")
            .execute(&self.db)
            .await
            .map_err(ExternalError::new)?;
        Ok(())
    }

    async fn find(&self, session_id: Uuid) -> ExternalResult<Option<Session>> {
        observe_db_query();
        let row: Option<(Uuid, bool, DateTime<Utc>)> =
            sqlx::query_as("select user_id, verified, expires_at from session where session_id = $1")
                .bind(session_id)
                .fetch_optional(&self.db)
                .await
                .map_err(ExternalError::new)?;
        Ok(row
            .filter(|(_, _, expires_at)| *expires_at > Utc::now())
            .map(|(user_id, verified, _)| Session {
                user_id,
                session_id,
                verified,
            }))
    }

    async fn delete(&self, session_id: Uuid) -> ExternalResult<()> {
        observe_db_query();
        sqlx::query("delete from session where session_id = $1")
            .bind(session_id)
            .execute(&self.db)
            .await
            .map_err(ExternalError::new)?;
        Ok(())
    }
}
