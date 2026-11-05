use std::net::{IpAddr, SocketAddr};

use axum::body::Body;
use axum::extract::State;
use axum::extract::connect_info::ConnectInfo;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::AppState;

/// Ограничение частоты по IP клиента. Ошибка ограничителя запрос не блокирует.
pub async fn rate_limit_middleware(State(state): State<AppState>, request: Request<Body>, next: Next) -> Response {
    let (Some(limiter), Some(ip)) = (&state.rate_limiter, client_ip(&request)) else {
        return next.run(request).await;
    };

    match limiter.allow(&ip.to_string()).await {
        Ok(true) => next.run(request).await,
        Ok(false) => (StatusCode::TOO_MANY_REQUESTS, "rate limit exceeded").into_response(),
        Err(error) => {
            tracing::error!(%error, "rate limit check failed");
            next.run(request).await
        }
    }
}

/// `X-Forwarded-For` (первый адрес), затем `X-Real-IP`, затем адрес соединения.
fn client_ip(request: &Request<Body>) -> Option<IpAddr> {
    let header = |name: &str| request.headers().get(name).and_then(|value| value.to_str().ok());

    header("x-forwarded-for")
        .and_then(|value| value.split(',').next())
        .and_then(|first| first.trim().parse().ok())
        .or_else(|| header("x-real-ip").and_then(|value| value.trim().parse().ok()))
        .or_else(|| {
            request
                .extensions()
                .get::<ConnectInfo<SocketAddr>>()
                .map(|info| info.0.ip())
        })
}
