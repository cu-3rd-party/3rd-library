use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
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
}

#[derive(Clone)]
pub struct ContentService {
    storage_dir: PathBuf,
    store: std::sync::Arc<Mutex<ContentStore>>,
}

impl ContentService {
    pub async fn new(storage_dir: String) -> Result<Self, ContentError> {
        tokio::fs::create_dir_all(&storage_dir)
            .await
            .map_err(|err| ContentError::Io(err.to_string()))?;
        Ok(Self {
            storage_dir: PathBuf::from(storage_dir),
            store: std::sync::Arc::new(Mutex::new(ContentStore::default())),
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

        let mut store = self.store.lock().await;
        store.items.insert(item.id.clone(), item.clone());
        store.order.push(item.id.clone());
        Ok(item)
    }

    pub async fn get(&self, content_id: String) -> Result<(ContentItem, Vec<u8>), ContentError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(ContentError::InvalidInput("content_id required".to_string()));
        }

        let item = {
            let store = self.store.lock().await;
            store.items.get(&content_id).cloned()
        }
        .ok_or(ContentError::NotFound)?;

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
        let mut offset = 0usize;
        if !page_token.is_empty() {
            offset = page_token
                .parse::<usize>()
                .map_err(|_| ContentError::InvalidInput("invalid page_token".to_string()))?;
        }

        let store = self.store.lock().await;
        let offset = offset.min(store.order.len());
        let slice_end = (offset + page_size as usize).min(store.order.len());
        let items = store.order[offset..slice_end]
            .iter()
            .filter_map(|id| store.items.get(id))
            .cloned()
            .collect::<Vec<_>>();
        let next_token = if slice_end < store.order.len() {
            slice_end.to_string()
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

#[derive(Default)]
struct ContentStore {
    items: HashMap<String, ContentItem>,
    order: Vec<String>,
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
