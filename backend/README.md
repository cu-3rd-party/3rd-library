# Backend

Rust REST API: axum + sqlx, DDD / layered clean architecture по образцу template-rust-svelte.
Зависимости: `web → infrastructure → application → domain` (+ `infrastructure → migration`);
`test-support` — только dev-зависимость. Внутренние слои не знают о внешних — это гарантирует граф Cargo.

```
crates/
├── domain/          агрегаты User / Submission / Material, value objects (Email, FileName, VerificationCode,
│                    SubmissionStatus), Audit, PageRequest, DomainError, трейты репозиториев и UnitOfWork
├── application/     сервисы сценариев (AuthService, UserService, MaterialService, SubmissionService,
│                    ModerationService), DTO, AppError, порты внешних сервисов (ports.rs); юнит-тесты на mockall
├── infrastructure/  sqlx-репозитории (Db: пул или общая транзакция UnitOfWork), сессии в Postgres,
│                    rate limit в памяти, S3, SMTP (фича smtp), JWT, argon2, Prometheus; Infrastructure
│   └── tests/            интеграционные тесты репозиториев
├── migration/       SQL-миграции sqlx (migrations/*.sql) + CLI
├── test-support/    PostgreSQL для интеграционных тестов: сервер на бинарь, БД на тест
└── web/             main.rs (composition root), routes/<feature>.rs, error.rs, auth.rs (экстракторы),
    │                rate_limit.rs, metrics.rs, config.rs
    └── tests/            интеграционные тесты API (oneshot, настоящая БД, фейки S3/почты)
```

## HTTP-контракт

Пути `/api/...`, JSON и формат ошибок `{"code", "message", "details"}` совпадают с прежним бэкендом —
на них завязан фронтенд. Токен — `Authorization: Token <jwt>` (или `Bearer <jwt>`). OpenAPI —
`/api/docs/openapi.json`, Swagger UI — `/api/docs/swagger-ui`, health — `/api/health`,
Prometheus — `:9091/metrics`. `library-web healthcheck` — для HEALTHCHECK в distroless.

## Настройки (env)

`DATABASE_URL` (или `POSTGRES_HOST`/`PORT`/`DB`/`USER`/`PASSWORD`), `LISTEN_ADDR` (`0.0.0.0:8080`),
`METRICS_ADDR` (`0.0.0.0:9091`), `HMAC_KEY`, `CORS_ALLOWED_ORIGIN` (пусто/`*` — любой),
`RATE_LIMIT_CAPACITY`/`RATE_LIMIT_REFILL_PER_SEC` (0 — выключено), `S3_ENDPOINT` или `S3_URL`, `S3_ACCESS_KEY`,
`S3_SECRET_KEY`, `S3_REGION`, `S3_BUCKET_NAME`, `SMTP_HOST`/`PORT`/`USER`/`PASSWORD` (без `SMTP_HOST` код
подтверждения пишется в лог), `RUST_LOG`. Локально читается `.env`.

## Добавить фичу

1. **domain**: `src/foo/` — агрегат, value objects, `repository.rs` (`trait FooRepository`); при нужде — метод в
   `unit_of_work::Transaction`.
2. **migration**: новый `migrations/NN_foo.sql` (применённые файлы не менять — sqlx сверяет контрольные суммы).
3. **infrastructure**: строка в `persistence/rows.rs`, `persistence/repositories/foo_repository.rs`
   (`SqlxFooRepository<'a>` поверх `Db`), поле в `Infrastructure`.
4. **application**: `src/foo/` — `service.rs`, `dto.rs`; юнит-тесты с моками из `testing.rs`.
5. **web**: `routes/foo.rs` (модели, хендлеры с `#[utoipa::path]`, `router()`), строка в `routes/mod.rs`,
   сервис в `AppState`.
6. Интеграционный тест в `crates/web/tests/` (`TestApp::spawn()`).

## Тесты и миграции

```bash
cargo test --workspace                                      # Postgres в Testcontainers, один на бинарь
TEST_DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres cargo test --workspace
cargo run -p library-migration -- status                    # up | status
```

Миграции применяются при старте web; ошибка миграции не роняет старт (как раньше), а пишется в лог.

## Docker

Контекст — корень репозитория: `docker build -f backend/Dockerfile .` (фильтр — `backend/Dockerfile.dockerignore`),
cargo-chef для кэша зависимостей, рантайм — distroless `cc-debian13:nonroot`, сборка с фичей `smtp`.
