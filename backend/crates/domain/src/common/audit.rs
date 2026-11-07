use chrono::{DateTime, Utc};

use super::now;

/// Аудит-метки сущности (`created_at`/`updated_at`). Обновляются самой сущностью при изменениях (`touch`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Audit {
    created: DateTime<Utc>,
    last_modified: DateTime<Utc>,
}

impl Audit {
    pub fn new() -> Self {
        let now = now();
        Self {
            created: now,
            last_modified: now,
        }
    }

    /// Восстановление из хранилища.
    pub fn from_parts(created: DateTime<Utc>, last_modified: DateTime<Utc>) -> Self {
        Self { created, last_modified }
    }

    pub fn touch(&mut self) {
        self.last_modified = now();
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.created
    }

    pub fn last_modified(&self) -> DateTime<Utc> {
        self.last_modified
    }
}

impl Default for Audit {
    fn default() -> Self {
        Self::new()
    }
}
