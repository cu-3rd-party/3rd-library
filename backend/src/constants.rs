use tokio::time::Duration;

// Extractor
pub const DEFAULT_SESSION_LENGTH: chrono::Duration = chrono::Duration::weeks(2);
pub const SCHEME_PREFIX: &str = "Token ";

// Metrics
pub const BUSINESS_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

// Articles
pub const UPLOAD_DIR: &str = "/var/app/upload";
