use bytes::Bytes;
use chrono::{DateTime, Utc};
use library_domain::submissions::{Submission, SubmissionStatus};
use uuid::Uuid;

use crate::AppResult;
use crate::materials::{MaterialFileDto, MaterialInfoDto};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionDto {
    pub id: Uuid,
    pub author_id: Uuid,
    pub info: MaterialInfoDto,
    pub status: SubmissionStatus,
    pub moderator_comment: String,
    pub files: Vec<MaterialFileDto>,
    pub created: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
}

impl From<&Submission> for SubmissionDto {
    fn from(submission: &Submission) -> Self {
        Self {
            id: submission.id().into_inner(),
            author_id: submission.author().into_inner(),
            info: submission.info().clone().into(),
            status: submission.status(),
            moderator_comment: submission.moderator_comment().to_owned(),
            files: submission.files().iter().map(MaterialFileDto::from).collect(),
            created: submission.audit().created(),
            last_modified: submission.audit().last_modified(),
            submitted_at: submission.submitted_at(),
            reviewed_at: submission.reviewed_at(),
            published_at: submission.published_at(),
        }
    }
}

/// Загруженный в запросе файл.
#[derive(Debug, Clone)]
pub struct NewFile {
    pub file_name: String,
    pub content_type: Option<String>,
    pub content: Bytes,
}

#[derive(Debug, Clone)]
pub struct CreateSubmission {
    pub title: String,
    pub description: Option<String>,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    pub kind: String,
    pub difficulty: String,
    pub files: Vec<NewFile>,
}

/// Правка заявки: `None` — поле не меняется (см. `SubmissionChanges`).
#[derive(Debug, Clone, Default)]
pub struct UpdateSubmission {
    pub title: Option<String>,
    pub description: Option<String>,
    pub courses: Option<Vec<String>>,
    pub subjects: Option<Vec<String>>,
    pub kind: Option<String>,
    pub difficulty: Option<String>,
    pub keep_file_ids: Option<Vec<Uuid>>,
    pub files: Vec<NewFile>,
}

/// Фильтр по статусу из query-параметра: не задан — `default`, `all` — без фильтра.
pub fn parse_status_filter(
    value: Option<&str>,
    default: Option<SubmissionStatus>,
) -> AppResult<Option<SubmissionStatus>> {
    match value {
        None => Ok(default),
        Some("all") => Ok(None),
        Some(value) => Ok(Some(SubmissionStatus::parse(value)?)),
    }
}
