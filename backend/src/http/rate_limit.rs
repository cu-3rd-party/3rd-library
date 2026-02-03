use crate::http::ApiContext;
use axum::body::Body;
use axum::extract::State;
use axum::extract::connect_info::ConnectInfo;
use axum::http::header::HeaderName;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use redis::Script;
use std::net::{IpAddr, SocketAddr};
use std::sync::OnceLock;
use time::OffsetDateTime;

const LUA_SCRIPT: &str = r#"
local key = KEYS[1]
local capacity = tonumber(ARGV[1])
local refill_per_sec = tonumber(ARGV[2])
local now_ms = tonumber(ARGV[3])
local ttl = tonumber(ARGV[4])

local data = redis.call('HMGET', key, 'tokens', 'ts')
local tokens = tonumber(data[1])
local ts = tonumber(data[2])

if tokens == nil then
  tokens = capacity
  ts = now_ms
end

if refill_per_sec > 0 then
  local delta = math.max(0, now_ms - ts)
  local refill = (delta / 1000) * refill_per_sec
  if refill > 0 then
    tokens = math.min(capacity, tokens + refill)
  end
end

local allowed = 0
if tokens >= 1 then
  tokens = tokens - 1
  allowed = 1
end

redis.call('HMSET', key, 'tokens', tokens, 'ts', now_ms)
if ttl and ttl > 0 then
  redis.call('EXPIRE', key, ttl)
end

return allowed
"#;

static SCRIPT: OnceLock<Script> = OnceLock::new();
static FORWARDED_FOR_HEADER: HeaderName = HeaderName::from_static("x-forwarded-for");
static REAL_IP_HEADER: HeaderName = HeaderName::from_static("x-real-ip");

pub async fn rate_limit_middleware(
    State(ctx): State<ApiContext>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = match extract_ip(&req) {
        Some(ip) => ip,
        None => return next.run(req).await,
    };

    let capacity = ctx.config.rate_limit.capacity;
    let refill_per_sec = ctx.config.rate_limit.refill_per_sec;
    if capacity == 0 || refill_per_sec == 0 {
        return next.run(req).await;
    }

    let ttl_seconds = ctx.rate_limit_ttl_seconds;
    let now_ms = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64;
    let key = format!("rate_limit:{ip}");

    let mut redis = ctx.redis.clone();
    let script = SCRIPT.get_or_init(|| Script::new(LUA_SCRIPT));
    let allowed: i32 = match script
        .key(key)
        .arg(capacity)
        .arg(refill_per_sec)
        .arg(now_ms)
        .arg(ttl_seconds)
        .invoke_async(&mut redis)
        .await
    {
        Ok(allowed) => allowed,
        Err(err) => {
            log::error!("rate limit redis error: {err}");
            return next.run(req).await;
        }
    };

    if allowed == 1 {
        next.run(req).await
    } else {
        (StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded").into_response()
    }
}

fn extract_ip(req: &Request<Body>) -> Option<IpAddr> {
    if let Some(value) = req.headers().get(&FORWARDED_FOR_HEADER) {
        if let Ok(value) = value.to_str() {
            if let Some(first) = value.split(',').next() {
                if let Ok(ip) = first.trim().parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }
    }

    if let Some(value) = req.headers().get(&REAL_IP_HEADER) {
        if let Ok(value) = value.to_str() {
            if let Ok(ip) = value.trim().parse::<IpAddr>() {
                return Some(ip);
            }
        }
    }

    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|info| info.0.ip())
}

pub fn bucket_ttl_seconds(capacity: u64, refill_per_sec: u64) -> u64 {
    if refill_per_sec == 0 {
        return 60;
    }

    let refill_time = capacity.saturating_add(refill_per_sec - 1) / refill_per_sec;
    let ttl = refill_time.saturating_mul(2);
    ttl.max(60)
}
