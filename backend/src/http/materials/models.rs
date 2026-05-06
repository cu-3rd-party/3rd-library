use crate::constants::ALLOWED_EXTENSIONS;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct MaterialFile {
    pub id: Uuid,
    pub name: String,
    pub size_bytes: u64,
    pub extension: String,
    pub mime_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize)]
pub struct MaterialDetails {
    #[serde(flatten)]
    pub material: Material,
    pub files: Vec<MaterialFile>,
    pub published_at: Option<String>,
    pub submitted_at: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedMaterialsResponse {
    pub items: Vec<Material>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct ListSubmissionsQuery {
    pub status: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct PaginatedSubmissionsResponse {
    pub items: Vec<Submission>,
    pub page: i64,
    pub limit: i64,
    pub total: i64,
}

#[derive(Deserialize)]
pub struct CreateSubmissionRequest {
    pub title: String,
    pub description: Option<String>,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    pub r#type: String,
    pub difficulty: String,
    pub files: Option<Vec<Vec<u8>>>,
}

#[derive(Deserialize)]
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
