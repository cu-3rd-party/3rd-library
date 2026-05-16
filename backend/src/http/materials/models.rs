use crate::constants::ALLOWED_EXTENSIONS;
use crate::http::materials::helpers::to_rfc3339;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::postgres::PgRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct Material {
    pub id: Uuid,
    pub author_id: Uuid,
    pub author_name: Option<String>,
    pub title: String,
    pub description: String,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub difficulty: String,
    pub pub_date: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct MaterialFile {
    pub id: Uuid,
    pub name: String,
    pub size_bytes: i64,
    pub extension: String,
    pub mime_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct MaterialDetails {
    #[serde(flatten)]
    pub material: Material,
    pub files: Vec<MaterialFile>,
    pub published_at: Option<String>,
    pub submitted_at: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedMaterialsResponse {
    pub items: Vec<Material>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ListMaterialsQuery {
    pub search: Option<String>,
    pub courses: Option<String>,
    pub subjects: Option<String>,
    pub types: Option<String>,
    pub difficulties: Option<String>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ListSubmissionsQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct Submission {
    pub id: Uuid,
    pub files: Vec<MaterialFile>,
    pub status: String,
    pub moderator_comment: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub submitted_at: Option<String>,
    pub reviewed_at: Option<String>,
    pub published_at: Option<String>,
}

impl From<PgRow> for Submission {
    fn from(s: PgRow) -> Self {
        Submission {
            id: s.get::<Uuid, _>("submission_id"),
            files: vec![],
            status: s.get::<String, _>("status"),
            moderator_comment: Some(
                s.get::<Option<String>, _>("moderator_comment")
                    .unwrap_or_default(),
            ),
            created_at: to_rfc3339(s.get::<DateTime<Utc>, _>("created_at")),
            updated_at: to_rfc3339(s.get::<DateTime<Utc>, _>("updated_at")),
            submitted_at: s
                .get::<Option<DateTime<Utc>>, _>("submitted_at")
                .map(to_rfc3339),
            reviewed_at: s
                .get::<Option<DateTime<Utc>>, _>("reviewed_at")
                .map(to_rfc3339),
            published_at: s
                .get::<Option<DateTime<Utc>>, _>("published_at")
                .map(to_rfc3339),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedSubmissionsResponse {
    pub items: Vec<Submission>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSubmissionRequest {
    pub title: String,
    pub description: Option<String>,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    pub r#type: String,
    pub difficulty: String,
    pub files: Option<Vec<Vec<u8>>>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSubmissionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub courses: Option<Vec<String>>,
    pub subjects: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub difficulty: Option<String>,
    pub files: Option<Vec<Vec<u8>>>,
    pub keep_file_ids: Option<Vec<Uuid>>,
}

pub struct FileName {
    pub name: String,
    pub extension: String,
}

impl FileName {
    pub fn new(name: String) -> Option<Self> {
        let extension = name.split('.').last()?.to_string();
        Some(FileName { name, extension })
    }
    pub fn new_valid(name: String) -> Option<Self> {
        let obj = FileName::new(name)?;
        if !obj.is_valid() {
            return None;
        }
        Some(obj)
    }

    fn is_valid(&self) -> bool {
        ALLOWED_EXTENSIONS.contains(&self.extension.as_str())
    }
}
