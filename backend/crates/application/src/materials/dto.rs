use chrono::{DateTime, Utc};
use library_domain::materials::{MaterialFile, MaterialInfo, MaterialSummary};
use uuid::Uuid;

/// Описание материала (общее у заявки и материала).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialInfoDto {
    pub title: String,
    pub description: String,
    pub courses: Vec<String>,
    pub subjects: Vec<String>,
    pub kind: String,
    pub difficulty: String,
}

impl From<MaterialInfo> for MaterialInfoDto {
    fn from(info: MaterialInfo) -> Self {
        Self {
            title: info.title,
            description: info.description,
            courses: info.courses,
            subjects: info.subjects,
            kind: info.kind,
            difficulty: info.difficulty,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialDto {
    pub id: Uuid,
    pub author_id: Uuid,
    pub author_name: String,
    pub info: MaterialInfoDto,
    pub published_at: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
}

impl From<MaterialSummary> for MaterialDto {
    fn from(material: MaterialSummary) -> Self {
        Self {
            id: material.id.into_inner(),
            author_id: material.author_id.into_inner(),
            author_name: material.author_name,
            info: material.info.into(),
            published_at: material.published_at,
            created: material.created,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialFileDto {
    pub id: Uuid,
    pub name: String,
    pub extension: String,
    pub size_bytes: i64,
    pub mime_type: Option<String>,
}

impl From<&MaterialFile> for MaterialFileDto {
    fn from(file: &MaterialFile) -> Self {
        Self {
            id: file.id.into_inner(),
            name: file.name.clone(),
            extension: file.extension.clone(),
            size_bytes: file.size_bytes,
            mime_type: file.mime_type.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialDetailsDto {
    pub material: MaterialDto,
    pub files: Vec<MaterialFileDto>,
}
