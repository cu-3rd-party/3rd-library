//! Инфраструктурный слой: всё, что знает про PostgreSQL (sqlx), S3, SMTP, JWT и argon2.
//! Наружу отдаёт доменные трейты и порты приложения (`Arc<dyn …>`).

mod config;
pub mod mail;
pub mod metrics;
pub mod persistence;
pub mod rate_limit;
pub mod security;
pub mod sessions;
pub mod storage;

use std::sync::Arc;

use anyhow::Context;
use library_application::ports::{FileStorage, Mailer, PasswordHasher, RateLimiter, SessionStore, TokenCodec};
use library_domain::materials::MaterialRepository;
use library_domain::submissions::SubmissionRepository;
use library_domain::unit_of_work::UnitOfWork;
use library_domain::users::UserRepository;
use sqlx::PgPool;

pub use crate::config::{InfrastructureConfig, RateLimitConfig, S3Config, SmtpConfig};
use crate::persistence::Db;
use crate::persistence::repositories::{SqlxMaterialRepository, SqlxSubmissionRepository, SqlxUserRepository};
use crate::persistence::unit_of_work::SqlxUnitOfWork;
use crate::rate_limit::MemoryRateLimiter;
use crate::security::{Argon2PasswordHasher, JwtTokenCodec};
use crate::sessions::PgSessionStore;
use crate::storage::S3FileStorage;

/// Реализации доменных абстракций и портов приложения — аналог `AddInfrastructureServices`.
#[derive(Clone)]
pub struct Infrastructure {
    pub db: PgPool,
    pub users: Arc<dyn UserRepository>,
    pub materials: Arc<dyn MaterialRepository>,
    pub submissions: Arc<dyn SubmissionRepository>,
    pub unit_of_work: Arc<dyn UnitOfWork>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub tokens: Arc<dyn TokenCodec>,
    pub sessions: Arc<dyn SessionStore>,
    pub storage: Arc<dyn FileStorage>,
    pub mailer: Arc<dyn Mailer>,
    /// `None` — ограничение выключено.
    pub rate_limiter: Option<Arc<dyn RateLimiter>>,
}

/// Адаптеры к сервисам с внешним состоянием (в тестах их заменяют фейками).
pub struct Adapters {
    pub sessions: Arc<dyn SessionStore>,
    pub storage: Arc<dyn FileStorage>,
    pub mailer: Arc<dyn Mailer>,
    pub rate_limiter: Option<Arc<dyn RateLimiter>>,
}

impl Infrastructure {
    /// Подключение к PostgreSQL и S3 (бакет создаётся, если его нет), миграции.
    pub async fn init(config: &InfrastructureConfig) -> anyhow::Result<Self> {
        let db = persistence::connect(&config.database_url)
            .await
            .context("could not connect to the database")?;
        tracing::info!("connected to the database");
        if config.run_migrations {
            persistence::migrate(&db).await;
        }

        let storage = S3FileStorage::connect(&config.s3)
            .await
            .context("could not prepare the S3 bucket")?;

        let adapters = Adapters {
            sessions: Arc::new(PgSessionStore::new(db.clone())),
            storage: Arc::new(storage),
            mailer: mail::mailer(config),
            rate_limiter: MemoryRateLimiter::new(&config.rate_limit)
                .map(|limiter| Arc::new(limiter) as Arc<dyn RateLimiter>),
        };
        Ok(Self::new(db, &config.hmac_key, adapters))
    }

    /// Репозитории на `db`, argon2 и JWT — свои, остальное — из `adapters`.
    pub fn new(db: PgPool, hmac_key: &str, adapters: Adapters) -> Self {
        Self {
            users: Arc::new(SqlxUserRepository::new(Db::Pool(db.clone()))),
            materials: Arc::new(SqlxMaterialRepository::new(Db::Pool(db.clone()))),
            submissions: Arc::new(SqlxSubmissionRepository::new(Db::Pool(db.clone()))),
            unit_of_work: Arc::new(SqlxUnitOfWork::new(db.clone())),
            password_hasher: Arc::new(Argon2PasswordHasher),
            tokens: Arc::new(JwtTokenCodec::new(hmac_key)),
            sessions: adapters.sessions,
            storage: adapters.storage,
            mailer: adapters.mailer,
            rate_limiter: adapters.rate_limiter,
            db,
        }
    }
}
