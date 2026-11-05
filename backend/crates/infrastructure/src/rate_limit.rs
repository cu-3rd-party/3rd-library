use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use async_trait::async_trait;
use library_application::ports::{ExternalResult, RateLimiter};

use crate::RateLimitConfig;

/// Сколько ключей держать, прежде чем выбросить полные (давно не обращавшиеся) вёдра.
const MAX_KEYS: usize = 10_000;

/// Token bucket в памяти процесса: лимит действует на одну реплику.
pub struct MemoryRateLimiter {
    capacity: f64,
    refill_per_sec: f64,
    buckets: Mutex<HashMap<String, Bucket>>,
}

#[derive(Clone, Copy)]
struct Bucket {
    tokens: f64,
    updated: Instant,
}

impl MemoryRateLimiter {
    /// `None`, если ограничение выключено (нулевая ёмкость или пополнение).
    pub fn new(config: &RateLimitConfig) -> Option<Self> {
        if config.capacity == 0 || config.refill_per_sec == 0 {
            return None;
        }
        Some(Self {
            capacity: config.capacity as f64,
            refill_per_sec: config.refill_per_sec as f64,
            buckets: Mutex::default(),
        })
    }

    fn allow_at(&self, key: &str, now: Instant) -> bool {
        let mut buckets = self.buckets.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if buckets.len() >= MAX_KEYS && !buckets.contains_key(key) {
            buckets.retain(|_, bucket| self.refill(*bucket, now) < self.capacity);
        }

        let bucket = buckets.entry(key.to_owned()).or_insert(Bucket {
            tokens: self.capacity,
            updated: now,
        });
        bucket.tokens = self.refill(*bucket, now);
        bucket.updated = now;
        if bucket.tokens < 1.0 {
            return false;
        }
        bucket.tokens -= 1.0;
        true
    }

    fn refill(&self, bucket: Bucket, now: Instant) -> f64 {
        let elapsed = now.saturating_duration_since(bucket.updated).as_secs_f64();
        (bucket.tokens + elapsed * self.refill_per_sec).min(self.capacity)
    }
}

#[async_trait]
impl RateLimiter for MemoryRateLimiter {
    async fn allow(&self, key: &str) -> ExternalResult<bool> {
        Ok(self.allow_at(key, Instant::now()))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn limiter(capacity: u64, refill_per_sec: u64) -> Option<MemoryRateLimiter> {
        MemoryRateLimiter::new(&RateLimitConfig {
            capacity,
            refill_per_sec,
        })
    }

    #[test]
    fn zero_disables_limit() {
        assert!(limiter(0, 10).is_none());
        assert!(limiter(10, 0).is_none());
    }

    #[test]
    fn bucket_empties_and_refills() {
        let limiter = limiter(2, 1).unwrap();
        let start = Instant::now();

        assert!(limiter.allow_at("ip", start));
        assert!(limiter.allow_at("ip", start));
        assert!(!limiter.allow_at("ip", start));
        assert!(limiter.allow_at("other", start));
        assert!(limiter.allow_at("ip", start + Duration::from_secs(1)));
    }
}
