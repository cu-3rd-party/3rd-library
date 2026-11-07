//! Групповой коммит. Изменения, пришедшие почти одновременно, уходят в БД одной пачкой:
//! один оператор с CTE на все таблицы (строки — JSON-массивами), один коммит и один fsync на всех.
//! Вызывающий ждёт коммита своей пачки, поэтому успешный ответ всегда означает «записано в БД».
//!
//! Оператор идемпотентен (upsert полного состояния строки, вставки `on conflict do nothing`, удаления по id),
//! поэтому после обрыва связи или временной ошибки пачка просто отправляется ещё раз. Если пачка отвергнута
//! самой БД (например, нарушено ограничение), её задания выполняются по одному: ошибка одного
//! не роняет остальных.

use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::{Mutex, mpsc, oneshot};
use uuid::Uuid;

use crate::metrics::observe_db_query;

/// Сколько полос записи (у каждой — своё соединение из пула на время пачки).
const LANES: usize = 4;
/// Предел заданий в одной пачке.
const MAX_JOBS_PER_BATCH: usize = 256;
/// Повторы пачки при обрыве связи и временных ошибках.
const RETRIES: usize = 3;

pub(crate) type WriteError = Arc<sqlx::Error>;

#[derive(Serialize)]
pub(crate) struct UserRow {
    pub user_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub verification_code: Option<String>,
    pub verification_code_issued_at: Option<DateTime<Utc>>,
    pub verification_code_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub(crate) struct FileRow {
    pub file_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub extension: String,
    pub mime_type: Option<String>,
    pub storage_key: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub(crate) struct MaterialRow {
    pub material_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: String,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    pub r#type: String,
    pub difficulty: String,
    pub published_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub(crate) struct MaterialFileRow {
    pub material_id: Uuid,
    pub file_id: Uuid,
}

#[derive(Serialize)]
pub(crate) struct SubmissionRow {
    pub submission_id: Uuid,
    pub user_id: Uuid,
    pub material_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    pub r#type: String,
    pub difficulty: String,
    pub status: &'static str,
    pub moderator_id: Option<Uuid>,
    pub moderator_comment: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

/// Полный набор файлов заявки: связи с остальными удаляются.
#[derive(Serialize)]
pub(crate) struct SubmissionFilesRow {
    pub submission_id: Uuid,
    pub file_ids: Vec<Uuid>,
}

#[derive(Serialize)]
pub(crate) struct SessionRow {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub verified: bool,
    pub expires_at: DateTime<Utc>,
}

/// Строки одной пачки по таблицам.
#[derive(Default)]
pub(crate) struct Batch {
    pub users: Vec<UserRow>,
    pub files: Vec<FileRow>,
    pub materials: Vec<MaterialRow>,
    pub material_files: Vec<MaterialFileRow>,
    pub submissions: Vec<SubmissionRow>,
    pub submission_files: Vec<SubmissionFilesRow>,
    pub sessions: Vec<SessionRow>,
    pub deleted_sessions: Vec<Uuid>,
}

impl Batch {
    fn is_empty(&self) -> bool {
        self.users.is_empty()
            && self.files.is_empty()
            && self.materials.is_empty()
            && self.material_files.is_empty()
            && self.submissions.is_empty()
            && self.submission_files.is_empty()
            && self.sessions.is_empty()
            && self.deleted_sessions.is_empty()
    }
}

/// Порядок колонок в `x(...)` совпадает со списком колонок `insert` — поэтому `select *`.
/// Внешние ключи проверяются в конце оператора, так что строки родителя и потомка могут быть в одной пачке.
const BATCH_SQL: &str = r#"
with
users as (
    insert into web_user (user_id, email, password_hash, name, bio, is_email_verified, verification_code,
                          verification_code_issued_at, verification_code_expires_at, created_at, updated_at)
    select * from jsonb_to_recordset($1::text::jsonb) as x(user_id uuid, email text, password_hash text, name text,
        bio text, is_email_verified bool, verification_code text, verification_code_issued_at timestamptz,
        verification_code_expires_at timestamptz, created_at timestamptz, updated_at timestamptz)
    on conflict (user_id) do update set name = excluded.name, bio = excluded.bio,
        is_email_verified = excluded.is_email_verified, verification_code = excluded.verification_code,
        verification_code_issued_at = excluded.verification_code_issued_at,
        verification_code_expires_at = excluded.verification_code_expires_at, updated_at = excluded.updated_at
),
files as (
    insert into material_file (file_id, user_id, name, size_bytes, extension, mime_type, storage_key, created_at)
    select * from jsonb_to_recordset($2::text::jsonb) as x(file_id uuid, user_id uuid, name text, size_bytes int8,
        extension text, mime_type text, storage_key text, created_at timestamptz)
    on conflict (file_id) do nothing
),
materials as (
    insert into material (material_id, user_id, title, description, courses, subjects, type, difficulty,
                          published_at, created_at, updated_at)
    select * from jsonb_to_recordset($3::text::jsonb) as x(material_id uuid, user_id uuid, title text,
        description text, courses text[], subjects text[], type text, difficulty text, published_at timestamptz,
        created_at timestamptz, updated_at timestamptz)
    on conflict (material_id) do nothing
),
material_files as (
    insert into material_file_rel (material_id, file_id)
    select * from jsonb_to_recordset($4::text::jsonb) as x(material_id uuid, file_id uuid)
    on conflict do nothing
),
submissions as (
    insert into submission (submission_id, user_id, material_id, title, description, courses, subjects, type,
                            difficulty, status, moderator_id, moderator_comment, published_at, created_at,
                            updated_at, submitted_at, reviewed_at)
    select * from jsonb_to_recordset($5::text::jsonb) as x(submission_id uuid, user_id uuid, material_id uuid,
        title text, description text, courses text[], subjects text[], type text, difficulty text, status text,
        moderator_id uuid, moderator_comment text, published_at timestamptz, created_at timestamptz,
        updated_at timestamptz, submitted_at timestamptz, reviewed_at timestamptz)
    on conflict (submission_id) do update set material_id = excluded.material_id, title = excluded.title,
        description = excluded.description, courses = excluded.courses, subjects = excluded.subjects,
        type = excluded.type, difficulty = excluded.difficulty, status = excluded.status,
        moderator_id = excluded.moderator_id, moderator_comment = excluded.moderator_comment,
        published_at = excluded.published_at, updated_at = excluded.updated_at,
        submitted_at = excluded.submitted_at, reviewed_at = excluded.reviewed_at
),
submission_files_dropped as (
    delete from submission_file_rel r
    using jsonb_to_recordset($6::text::jsonb) as x(submission_id uuid, file_ids uuid[])
    where r.submission_id = x.submission_id and not (r.file_id = any(x.file_ids))
),
submission_files as (
    insert into submission_file_rel (submission_id, file_id)
    select x.submission_id, unnest(x.file_ids)
    from jsonb_to_recordset($6::text::jsonb) as x(submission_id uuid, file_ids uuid[])
    on conflict do nothing
),
sessions as (
    insert into session (session_id, user_id, verified, expires_at)
    select * from jsonb_to_recordset($7::text::jsonb) as x(session_id uuid, user_id uuid, verified bool,
        expires_at timestamptz)
    on conflict (session_id) do update set verified = excluded.verified, expires_at = excluded.expires_at
),
sessions_dropped as (
    delete from session where session_id in (select (value #>> '{}')::uuid from jsonb_array_elements($8::text::jsonb))
)
select 1
"#;

struct Job {
    batch: Batch,
    done: oneshot::Sender<Result<(), WriteError>>,
}

/// Очередь записи и полосы, которые её разбирают.
pub(crate) struct Writer {
    queue: mpsc::UnboundedSender<Job>,
}

impl Writer {
    pub(crate) fn start(db: PgPool) -> Self {
        let (queue, receiver) = mpsc::unbounded_channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..LANES {
            tokio::spawn(lane(db.clone(), receiver.clone()));
        }
        Self { queue }
    }

    /// Записывает пачку и ждёт коммита.
    pub(crate) async fn write(&self, batch: Batch) -> Result<(), WriteError> {
        if batch.is_empty() {
            return Ok(());
        }
        let (done, wait) = oneshot::channel();
        let closed = || Arc::new(sqlx::Error::PoolClosed);
        self.queue.send(Job { batch, done }).map_err(|_| closed())?;
        wait.await.map_err(|_| closed())?
    }
}

async fn lane(db: PgPool, receiver: Arc<Mutex<mpsc::UnboundedReceiver<Job>>>) {
    loop {
        let jobs = {
            let mut receiver = receiver.lock().await;
            let Some(first) = receiver.recv().await else {
                return;
            };
            let mut jobs = vec![first];
            while jobs.len() < MAX_JOBS_PER_BATCH {
                match receiver.try_recv() {
                    Ok(job) => jobs.push(job),
                    Err(_) => break,
                }
            }
            jobs
        };
        run(&db, jobs).await;
    }
}

async fn run(db: &PgPool, jobs: Vec<Job>) {
    if jobs.len() == 1 {
        let job = jobs.into_iter().next().expect("one job");
        let result = execute(db, &merge(std::iter::once(&job.batch))).await;
        job.done.send(result).ok();
        return;
    }

    let merged = merge(jobs.iter().map(|job| &job.batch));
    match execute(db, &merged).await {
        Ok(()) => {
            for job in jobs {
                job.done.send(Ok(())).ok();
            }
        }
        Err(error) if error.as_database_error().is_some() => {
            // пачку отвергла БД — виновника ищем, выполняя задания по одному
            tracing::debug!(%error, jobs = jobs.len(), "batch rejected, writing jobs one by one");
            for job in jobs {
                let result = execute(db, &merge(std::iter::once(&job.batch))).await;
                job.done.send(result).ok();
            }
        }
        Err(error) => {
            for job in jobs {
                job.done.send(Err(error.clone())).ok();
            }
        }
    }
}

/// Пачки как JSON-строки по таблицам (сериализуются по ссылке, без копирования строк).
fn merge<'a>(batches: impl Iterator<Item = &'a Batch> + Clone) -> MergedBatch {
    fn json<'a, T: Serialize + 'a>(items: impl Iterator<Item = &'a T>) -> String {
        let items: Vec<&T> = items.collect();
        serde_json::to_string(&items).expect("rows serialize to JSON")
    }
    MergedBatch {
        users: json(batches.clone().flat_map(|batch| &batch.users)),
        files: json(batches.clone().flat_map(|batch| &batch.files)),
        materials: json(batches.clone().flat_map(|batch| &batch.materials)),
        material_files: json(batches.clone().flat_map(|batch| &batch.material_files)),
        submissions: json(batches.clone().flat_map(|batch| &batch.submissions)),
        submission_files: json(batches.clone().flat_map(|batch| &batch.submission_files)),
        sessions: json(batches.clone().flat_map(|batch| &batch.sessions)),
        deleted_sessions: json(batches.flat_map(|batch| &batch.deleted_sessions)),
    }
}

struct MergedBatch {
    users: String,
    files: String,
    materials: String,
    material_files: String,
    submissions: String,
    submission_files: String,
    sessions: String,
    deleted_sessions: String,
}

async fn execute(db: &PgPool, batch: &MergedBatch) -> Result<(), WriteError> {
    let mut attempt = 0;
    loop {
        observe_db_query();
        let result = sqlx::query(BATCH_SQL)
            .bind(&batch.users)
            .bind(&batch.files)
            .bind(&batch.materials)
            .bind(&batch.material_files)
            .bind(&batch.submissions)
            .bind(&batch.submission_files)
            .bind(&batch.sessions)
            .bind(&batch.deleted_sessions)
            .execute(db)
            .await;
        match result {
            Ok(_) => return Ok(()),
            Err(error) if attempt < RETRIES && is_transient(&error) => {
                attempt += 1;
                tracing::warn!(%error, attempt, "batch write failed, retrying");
                tokio::time::sleep(Duration::from_millis(20 * attempt as u64)).await;
            }
            Err(error) => return Err(Arc::new(error)),
        }
    }
}

/// Обрыв связи, таймаут пула, сериализация/взаимоблокировка, отмена или перезапуск сервера.
fn is_transient(error: &sqlx::Error) -> bool {
    match error {
        sqlx::Error::Io(_) | sqlx::Error::PoolTimedOut | sqlx::Error::Protocol(_) | sqlx::Error::Tls(_) => true,
        sqlx::Error::Database(error) => error.code().is_some_and(|code| {
            ["08", "40", "53", "55", "57"]
                .iter()
                .any(|class| code.starts_with(class))
        }),
        _ => false,
    }
}

/// Нарушение уникальности (23505).
pub(crate) fn is_unique_violation(error: &sqlx::Error) -> bool {
    error
        .as_database_error()
        .and_then(|error| error.code())
        .is_some_and(|code| code == "23505")
}
