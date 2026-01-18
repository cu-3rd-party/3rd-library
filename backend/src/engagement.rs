use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Comment {
    pub id: String,
    pub content_id: String,
    pub user_id: String,
    pub body: String,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct EngagementSummary {
    pub likes: i64,
    pub comments: i64,
    pub liked_by_user: bool,
}

#[derive(Debug)]
pub enum EngagementError {
    InvalidInput(String),
}

#[derive(Clone)]
pub struct EngagementService {
    store: std::sync::Arc<Mutex<EngagementStore>>,
}

impl EngagementService {
    pub fn new() -> Self {
        Self {
            store: std::sync::Arc::new(Mutex::new(EngagementStore::default())),
        }
    }

    pub async fn add_comment(
        &self,
        content_id: String,
        user_id: String,
        body: String,
        created_at: i64,
    ) -> Result<Comment, EngagementError> {
        let content_id = content_id.trim().to_string();
        let user_id = user_id.trim().to_string();
        let body = body.trim().to_string();
        if content_id.is_empty() || user_id.is_empty() || body.is_empty() {
            return Err(EngagementError::InvalidInput(
                "content_id, user_id, and body required".to_string(),
            ));
        }

        let created_at = if created_at <= 0 {
            now_timestamp()
        } else {
            created_at
        };

        let comment = Comment {
            id: Uuid::new_v4().simple().to_string(),
            content_id: content_id.clone(),
            user_id,
            body,
            created_at,
        };

        let mut store = self.store.lock().await;
        store.comments.entry(content_id).or_default().push(comment.clone());
        Ok(comment)
    }

    pub async fn list_comments(&self, content_id: String) -> Result<Vec<Comment>, EngagementError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(EngagementError::InvalidInput("content_id required".to_string()));
        }
        let store = self.store.lock().await;
        Ok(store.comments.get(&content_id).cloned().unwrap_or_default())
    }

    pub async fn set_like(
        &self,
        content_id: String,
        user_id: String,
        liked: bool,
    ) -> Result<i64, EngagementError> {
        let content_id = content_id.trim().to_string();
        let user_id = user_id.trim().to_string();
        if content_id.is_empty() || user_id.is_empty() {
            return Err(EngagementError::InvalidInput(
                "content_id and user_id required".to_string(),
            ));
        }

        let mut store = self.store.lock().await;
        let likes = store.likes.entry(content_id).or_default();
        if liked {
            likes.insert(user_id);
        } else {
            likes.remove(&user_id);
        }
        Ok(likes.len() as i64)
    }

    pub async fn summary(&self, content_id: String, user_id: Option<String>) -> Result<EngagementSummary, EngagementError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(EngagementError::InvalidInput("content_id required".to_string()));
        }
        let user_id = user_id.and_then(|value| {
            let trimmed = value.trim().to_string();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        });

        let store = self.store.lock().await;
        let likes = store.likes.get(&content_id);
        let comments = store.comments.get(&content_id).map(|items| items.len()).unwrap_or(0);
        let liked_by_user = user_id
            .as_ref()
            .and_then(|user_id| likes.map(|set| set.contains(user_id)))
            .unwrap_or(false);
        let likes_count = likes.map(|set| set.len()).unwrap_or(0);
        Ok(EngagementSummary {
            likes: likes_count as i64,
            comments: comments as i64,
            liked_by_user,
        })
    }
}

#[derive(Default)]
struct EngagementStore {
    comments: HashMap<String, Vec<Comment>>,
    likes: HashMap<String, HashSet<String>>,
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
