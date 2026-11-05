//! Строки таблиц (sqlx) и маппинг в доменные модели. Колонки, допускающие NULL по схеме, — `Option`.

use chrono::{DateTime, Utc};
use library_domain::RepositoryError;
use library_domain::common::Audit;
use library_domain::error::RepositoryResult;
use library_domain::materials::{MaterialFile, MaterialInfo, MaterialSummary};
use library_domain::submissions::{Submission, SubmissionData, SubmissionStatus};
use library_domain::users::{AuthorSummary, Email, PasswordHash, ROLE_USER, User, UserData, VerificationCode};
use uuid::Uuid;

pub(crate) const USER_COLUMNS: &str = "user_id, email, password_hash, name, bio, roles, is_email_verified, \
     verification_code, verification_code_issued_at, verification_code_expires_at, created_at, updated_at";

pub(crate) const FILE_COLUMNS: &str =
    "mf.file_id, mf.user_id, mf.name, mf.size_bytes, mf.extension, mf.mime_type, mf.storage_key, mf.created_at";

pub(crate) const MATERIAL_SUMMARY_SELECT: &str = "select m.material_id, m.user_id, u.name as author_name, m.title, \
     m.description, m.courses, m.subjects, m.type, m.difficulty, m.published_at, m.created_at \
     from material m inner join web_user u on m.user_id = u.user_id";

pub(crate) const SUBMISSION_COLUMNS: &str = "s.submission_id, s.user_id, s.material_id, s.title, s.description, \
     s.courses, s.subjects, s.type, s.difficulty, s.status, s.moderator_id, s.moderator_comment, \
     s.published_at, s.created_at, s.updated_at, s.submitted_at, s.reviewed_at";

fn audit(created: Option<DateTime<Utc>>, updated: Option<DateTime<Utc>>) -> Audit {
    let created = created.unwrap_or_default();
    Audit::from_parts(created, updated.unwrap_or(created))
}

#[derive(sqlx::FromRow)]
pub(crate) struct UserRow {
    user_id: Uuid,
    email: String,
    password_hash: String,
    name: String,
    bio: Option<String>,
    roles: Option<Vec<String>>,
    is_email_verified: Option<bool>,
    verification_code: Option<String>,
    verification_code_issued_at: Option<DateTime<Utc>>,
    verification_code_expires_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User::restore(UserData {
            id: row.user_id.into(),
            email: Email::restore(row.email),
            password_hash: PasswordHash::new(row.password_hash),
            name: row.name,
            bio: row.bio.unwrap_or_default(),
            roles: row.roles.unwrap_or_else(|| vec![ROLE_USER.to_owned()]),
            is_email_verified: row.is_email_verified.unwrap_or(false),
            verification_code: row.verification_code.map(VerificationCode::restore),
            code_issued_at: row.verification_code_issued_at,
            code_expires_at: row.verification_code_expires_at,
            audit: audit(row.created_at, row.updated_at),
        })
    }
}

#[derive(sqlx::FromRow)]
pub(crate) struct AuthorRow {
    user_id: Uuid,
    name: String,
    bio: Option<String>,
    is_email_verified: Option<bool>,
    materials_count: i64,
}

impl From<AuthorRow> for AuthorSummary {
    fn from(row: AuthorRow) -> Self {
        Self {
            id: row.user_id.into(),
            name: row.name,
            bio: row.bio.unwrap_or_default(),
            is_email_verified: row.is_email_verified.unwrap_or(false),
            materials_count: row.materials_count,
        }
    }
}

#[derive(sqlx::FromRow)]
pub(crate) struct FileRow {
    file_id: Uuid,
    user_id: Uuid,
    name: String,
    size_bytes: i64,
    extension: String,
    mime_type: Option<String>,
    storage_key: String,
    created_at: Option<DateTime<Utc>>,
}

impl From<FileRow> for MaterialFile {
    fn from(row: FileRow) -> Self {
        Self {
            id: row.file_id.into(),
            owner: row.user_id.into(),
            name: row.name,
            extension: row.extension,
            size_bytes: row.size_bytes,
            mime_type: row.mime_type,
            storage_key: row.storage_key,
            created: row.created_at.unwrap_or_default(),
        }
    }
}

/// Файл с id заявки, к которой он прикреплён (пакетная загрузка файлов для списка заявок).
#[derive(sqlx::FromRow)]
pub(crate) struct LinkedFileRow {
    pub(crate) submission_id: Uuid,
    #[sqlx(flatten)]
    pub(crate) file: FileRow,
}

#[derive(sqlx::FromRow)]
pub(crate) struct InfoRow {
    title: String,
    description: Option<String>,
    courses: Option<Vec<String>>,
    subjects: Option<Vec<String>>,
    #[sqlx(rename = "type")]
    kind: String,
    difficulty: Option<String>,
}

impl From<InfoRow> for MaterialInfo {
    fn from(row: InfoRow) -> Self {
        Self {
            title: row.title,
            description: row.description.unwrap_or_default(),
            courses: row.courses.unwrap_or_default(),
            subjects: row.subjects.unwrap_or_default(),
            kind: row.kind,
            difficulty: row.difficulty.unwrap_or_else(|| "none".to_owned()),
        }
    }
}

#[derive(sqlx::FromRow)]
pub(crate) struct MaterialSummaryRow {
    material_id: Uuid,
    user_id: Uuid,
    author_name: String,
    #[sqlx(flatten)]
    info: InfoRow,
    published_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
}

impl From<MaterialSummaryRow> for MaterialSummary {
    fn from(row: MaterialSummaryRow) -> Self {
        Self {
            id: row.material_id.into(),
            author_id: row.user_id.into(),
            author_name: row.author_name,
            info: row.info.into(),
            published_at: row.published_at,
            created: row.created_at.unwrap_or_default(),
        }
    }
}

#[derive(sqlx::FromRow)]
pub(crate) struct SubmissionRow {
    pub(crate) submission_id: Uuid,
    user_id: Uuid,
    material_id: Option<Uuid>,
    #[sqlx(flatten)]
    info: InfoRow,
    status: String,
    moderator_id: Option<Uuid>,
    moderator_comment: Option<String>,
    published_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    submitted_at: Option<DateTime<Utc>>,
    reviewed_at: Option<DateTime<Utc>>,
}

impl SubmissionRow {
    pub(crate) fn into_domain(self, files: Vec<MaterialFile>) -> RepositoryResult<Submission> {
        // статус в БД должен быть известен домену; если нет — это ошибка хранилища
        let status = SubmissionStatus::parse(&self.status).map_err(RepositoryError::new)?;
        Ok(Submission::restore(SubmissionData {
            id: self.submission_id.into(),
            author: self.user_id.into(),
            material_id: self.material_id.map(Into::into),
            info: self.info.into(),
            status,
            moderator_id: self.moderator_id.map(Into::into),
            moderator_comment: self.moderator_comment.unwrap_or_default(),
            files,
            submitted_at: self.submitted_at,
            reviewed_at: self.reviewed_at,
            published_at: self.published_at,
            audit: audit(self.created_at, self.updated_at),
        }))
    }
}
