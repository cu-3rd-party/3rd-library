use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum InteractionType {
    View,
    Download,
}

#[derive(Debug, Clone)]
pub struct ContentStats {
    pub views: i64,
    pub downloads: i64,
    pub unique_users: i64,
    pub last_interaction_at: i64,
}

#[derive(Debug)]
pub enum StatisticsError {
    InvalidInput(String),
}

#[derive(Clone)]
pub struct StatisticsService {
    store: std::sync::Arc<Mutex<StatisticsStore>>,
}

impl StatisticsService {
    pub fn new() -> Self {
        Self {
            store: std::sync::Arc::new(Mutex::new(StatisticsStore::default())),
        }
    }

    pub async fn record(
        &self,
        content_id: String,
        user_id: String,
        interaction_type: InteractionType,
        occurred_at: i64,
    ) -> Result<String, StatisticsError> {
        let content_id = content_id.trim().to_string();
        let user_id = user_id.trim().to_string();
        if content_id.is_empty() || user_id.is_empty() {
            return Err(StatisticsError::InvalidInput(
                "content_id and user_id required".to_string(),
            ));
        }
        let occurred_at = if occurred_at <= 0 {
            now_timestamp()
        } else {
            occurred_at
        };

        let mut store = self.store.lock().await;
        match interaction_type {
            InteractionType::View => {
                *store.views.entry(content_id.clone()).or_insert(0) += 1;
            }
            InteractionType::Download => {
                *store.downloads.entry(content_id.clone()).or_insert(0) += 1;
            }
        }
        store
            .users
            .entry(content_id.clone())
            .or_default()
            .insert(user_id);
        let previous = store.last_seen.get(&content_id).copied().unwrap_or(0);
        store
            .last_seen
            .insert(content_id, previous.max(occurred_at));
        Ok(Uuid::new_v4().simple().to_string())
    }

    pub async fn stats_for(&self, content_id: String) -> Result<ContentStats, StatisticsError> {
        let content_id = content_id.trim().to_string();
        if content_id.is_empty() {
            return Err(StatisticsError::InvalidInput("content_id required".to_string()));
        }
        let store = self.store.lock().await;
        Ok(ContentStats {
            views: *store.views.get(&content_id).unwrap_or(&0),
            downloads: *store.downloads.get(&content_id).unwrap_or(&0),
            unique_users: store.users.get(&content_id).map(|set| set.len()).unwrap_or(0) as i64,
            last_interaction_at: *store.last_seen.get(&content_id).unwrap_or(&0),
        })
    }
}

#[derive(Default)]
struct StatisticsStore {
    views: HashMap<String, i64>,
    downloads: HashMap<String, i64>,
    users: HashMap<String, HashSet<String>>,
    last_seen: HashMap<String, i64>,
}

fn now_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}
