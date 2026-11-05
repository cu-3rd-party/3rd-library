use chrono::{DateTime, Utc};

use super::{FileId, MaterialInfo};
use crate::common::{Audit, entity_id};
use crate::users::UserId;

entity_id!(MaterialId);

/// Агрегат «опубликованный материал». Появляется только из одобренной заявки
/// (`Submission::approve`); читается через [`super::MaterialSummary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Material {
    id: MaterialId,
    author: UserId,
    info: MaterialInfo,
    file_ids: Vec<FileId>,
    published_at: DateTime<Utc>,
    audit: Audit,
}

impl Material {
    pub fn publish(author: UserId, info: MaterialInfo, file_ids: Vec<FileId>, now: DateTime<Utc>) -> Self {
        Self {
            id: MaterialId::new(),
            author,
            info,
            file_ids,
            published_at: now,
            audit: Audit::new(),
        }
    }

    pub fn id(&self) -> MaterialId {
        self.id
    }

    pub fn author(&self) -> UserId {
        self.author
    }

    pub fn info(&self) -> &MaterialInfo {
        &self.info
    }

    pub fn file_ids(&self) -> &[FileId] {
        &self.file_ids
    }

    pub fn published_at(&self) -> DateTime<Utc> {
        self.published_at
    }

    pub fn audit(&self) -> &Audit {
        &self.audit
    }
}
