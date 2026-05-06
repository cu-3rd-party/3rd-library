use chrono::{DateTime, Utc};

pub fn to_rfc3339(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
}