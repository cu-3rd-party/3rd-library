use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ContentItem {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub description: String,
    pub filename: String,
    pub size_bytes: i64,
    pub created_at: i64,
}

#[derive(Debug)]
pub enum ContentError {
    InvalidInput(String),
    NotFound,
    Io(String),
    Db(String),
}

#[derive(Clone)]
pub struct ContentService {
    storage_dir: PathBuf,
    pool: PgPool,
}

impl ContentService {
    pub async fn new(pool: PgPool, storage_dir: String) -> Result<Self, ContentError> {
        tokio::fs::create_dir_all(&storage_dir)
            .await
            .map_err(|err| ContentError::Io(err.to_string()))?;
        Ok(Self {
            storage_dir: PathBuf::from(storage_dir),
            pool,
        })
    }

    pub async fn upload(
        &self,
        owner_id: String,
        title: String,
        description: String,
        filename: String,
        file_bytes: Vec<u8>,
    ) -> Result<ContentItem, ContentError> {
        let owner_id = owner_id.trim().to_string();
        let title = title.trim().to_string();
        let description = description.trim().to_string();
        let filename = filename.trim().to_string();
        if owner_id.is_empty() || title.is_empty() || filename.is_empty() || file_bytes.is_empty() {
            return Err(ContentError::InvalidInput("missing required fields".to_string()));
        }

        let id = Uuid::new_v4().simple().to_string();
        let created_at = now_timestamp();
        let item = ContentItem {
            id: id.clone(),
            owner_id,
            title,
            description,
            filename: filename.clone(),
            size_bytes: file_bytes.len() as i64,
            created_at,
        };

        let file_path = self.file_path(&item.id, &filename);
        tokio::fs::write(&file_path, &file_bytes)
            .await
            .map_err(|err| ContentError::Io(err.to_string()))?;

        if let Err(err) = sqlx::query(
            r#"
            INSERT INTO contents (id, owner_id, title, description, filename, size_bytes, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(&item.id)
        .bind(&item.owner_id)
        .bind(&item.title)
        .bind(&item.description)
        .bind(&item.filename)
        .bind(item.size_bytes)
        .bind(item.created_at)
        .execute(&self.pool)
        .await
        {
            let _ = tokio::fs::remove_file(&file_path).await;
            return Err(ContentError::Db(err.to_string()));
        }

        Ok(item)
    }

    pub async fn get(&self, content_id: String) -> Result<(ContentItem, Vec<u8>), ContentError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(ContentError::InvalidInput("content_id required".to_string()));
        }

        let row = sqlx::query_as::<_, ContentRow>(
            r#"
            SELECT id, owner_id, title, description, filename, size_bytes, created_at
            FROM contents
            WHERE id = $1
            "#,
        )
        .bind(&content_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| ContentError::Db(err.to_string()))?
        .ok_or(ContentError::NotFound)?;

        let item = row.into_item();
        let file_path = self.file_path(&item.id, &item.filename);
        let bytes = tokio::fs::read(&file_path)
            .await
            .map_err(|err| ContentError::Io(err.to_string()))?;
        Ok((item, bytes))
    }

    pub async fn list_items(&self, page_size: i32, page_token: String) -> Result<(Vec<ContentItem>, String), ContentError> {
        let mut page_size = if page_size <= 0 { 20 } else { page_size };
        if page_size > 100 {
            page_size = 100;
        }
        let mut offset = 0i64;
        if !page_token.is_empty() {
            offset = page_token
                .parse::<i64>()
                .map_err(|_| ContentError::InvalidInput("invalid page_token".to_string()))?;
            if offset < 0 {
                offset = 0;
            }
        }

        let rows = sqlx::query_as::<_, ContentRow>(
            r#"
            SELECT id, owner_id, title, description, filename, size_bytes, created_at
            FROM contents
            ORDER BY created_at ASC, id ASC
            OFFSET $1
            LIMIT $2
            "#,
        )
        .bind(offset)
        .bind((page_size + 1) as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|err| ContentError::Db(err.to_string()))?;

        let has_more = rows.len() > page_size as usize;
        let items = rows
            .into_iter()
            .take(page_size as usize)
            .map(ContentRow::into_item)
            .collect::<Vec<_>>();
        let next_token = if has_more {
            (offset + page_size as i64).to_string()
        } else {
            String::new()
        };
        Ok((items, next_token))
    }

    fn file_path(&self, content_id: &str, filename: &str) -> PathBuf {
        let safe_name = filename.replace('/', "_");
        self.storage_dir.join(format!("{content_id}-{safe_name}"))
    }
}

#[derive(Debug, FromRow)]
struct ContentRow {
    id: String,
    owner_id: String,
    title: String,
    description: String,
    filename: String,
    size_bytes: i64,
    created_at: i64,
}

impl ContentRow {
    fn into_item(self) -> ContentItem {
        ContentItem {
            id: self.id,
            owner_id: self.owner_id,
            title: self.title,
            description: self.description,
            filename: self.filename,
            size_bytes: self.size_bytes,
            created_at: self.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_service() -> ContentService {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres@localhost/postgres")
            .expect("lazy pool");
        ContentService {
            storage_dir: PathBuf::from("/tmp/content-service-test"),
            pool,
        }
    }

    #[tokio::test]
    async fn file_path_sanitizes_slashes() {
        let service = test_service();
        let path = service.file_path("content123", "folder/file.txt");
        assert!(path.ends_with("content123-folder_file.txt"));
    }

    #[tokio::test]
    async fn file_path_preserves_name() {
        let service = test_service();
        let path = service.file_path("content123", "file.txt");
        assert!(path.ends_with("content123-file.txt"));
    }

    #[tokio::test]
    async fn upload_rejects_missing_fields() {
        let service = test_service();
        let err = service
            .upload(
                "".to_string(),
                "title".to_string(),
                "desc".to_string(),
                "file.txt".to_string(),
                vec![1, 2, 3],
            )
            .await
            .expect_err("missing owner_id rejected");
        assert!(matches!(err, ContentError::InvalidInput(_)));

        let err = service
            .upload(
                "owner".to_string(),
                "".to_string(),
                "desc".to_string(),
                "file.txt".to_string(),
                vec![1, 2, 3],
            )
            .await
            .expect_err("missing title rejected");
        assert!(matches!(err, ContentError::InvalidInput(_)));

        let err = service
            .upload(
                "owner".to_string(),
                "title".to_string(),
                "desc".to_string(),
                "".to_string(),
                vec![1, 2, 3],
            )
            .await
            .expect_err("missing filename rejected");
        assert!(matches!(err, ContentError::InvalidInput(_)));

        let err = service
            .upload(
                "owner".to_string(),
                "title".to_string(),
                "desc".to_string(),
                "file.txt".to_string(),
                Vec::new(),
            )
            .await
            .expect_err("missing file bytes rejected");
        assert!(matches!(err, ContentError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn get_rejects_empty_content_id() {
        let service = test_service();
        let err = service
            .get("".to_string())
            .await
            .expect_err("empty content_id rejected");
        assert!(matches!(err, ContentError::InvalidInput(_)));
    }

    #[tokio::test]
    async fn list_items_rejects_invalid_page_token() {
        let service = test_service();
        let err = service
            .list_items(10, "nope".to_string())
            .await
            .expect_err("invalid page token rejected");
        assert!(matches!(err, ContentError::InvalidInput(_)));
    }
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
