use std::fmt;

use chrono::{DateTime, Utc};

use crate::DomainError;
use crate::common::entity_id;
use crate::users::UserId;

entity_id!(FileId);

/// Расширения файлов, которые можно прикладывать к материалам.
pub const ALLOWED_EXTENSIONS: &[&str] = &["pdf", "doc", "docx", "ppt", "pptx", "ipynb", "py"];

/// Имя загружаемого файла с допустимым расширением (всё после последней точки).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileName {
    name: String,
    extension: String,
}

impl FileName {
    pub fn parse(name: impl Into<String>) -> Result<Self, DomainError> {
        let name = name.into();
        let extension = name.rsplit('.').next().unwrap_or_default().to_owned();
        if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
            return Err(DomainError::validation(
                "files",
                format!("file extension must be one of: {}", ALLOWED_EXTENSIONS.join(", ")),
            ));
        }
        Ok(Self { name, extension })
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }

    pub fn extension(&self) -> &str {
        &self.extension
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

/// Файл, загруженный пользователем в хранилище; прикрепляется к заявкам и материалам.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialFile {
    pub id: FileId,
    pub owner: UserId,
    pub name: String,
    pub extension: String,
    pub size_bytes: i64,
    pub mime_type: Option<String>,
    /// Ключ объекта в хранилище.
    pub storage_key: String,
    pub created: DateTime<Utc>,
}

impl MaterialFile {
    /// Новый файл. Ключ уникален для файла (`materials/{id}/{name}`): одинаковые имена не перезаписывают друг друга.
    pub fn new(owner: UserId, name: FileName, size_bytes: i64, mime_type: Option<String>) -> Self {
        let id = FileId::new();
        Self {
            storage_key: format!("materials/{id}/{name}"),
            id,
            owner,
            extension: name.extension,
            name: name.name,
            size_bytes,
            mime_type,
            created: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_takes_last_extension() {
        let name = FileName::parse("lecture.v2.pdf").unwrap();
        assert_eq!((name.as_str(), name.extension()), ("lecture.v2.pdf", "pdf"));
    }

    #[test]
    fn parse_rejects_unknown_extension() {
        assert!(FileName::parse("virus.exe").is_err());
        assert!(FileName::parse("noextension").is_err());
    }

    #[test]
    fn storage_keys_differ_for_same_name() {
        let owner = UserId::new();
        let a = MaterialFile::new(owner, FileName::parse("a.pdf").unwrap(), 1, None);
        let b = MaterialFile::new(owner, FileName::parse("a.pdf").unwrap(), 1, None);
        assert_ne!(a.storage_key, b.storage_key);
    }
}
