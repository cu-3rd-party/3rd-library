use axum::body::Body;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;

use super::{HTTP_REQUEST_DURATION_SECONDS, HTTP_REQUESTS_TOTAL};

pub async fn metrics_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().as_str().to_string();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|matched| matched.as_str().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let start = Instant::now();
    let response = next.run(req).await;
    let status = response.status().as_u16().to_string();

    HTTP_REQUESTS_TOTAL
        .with_label_values(&[method.as_str(), status.as_str(), route.as_str()])
        .inc();
    HTTP_REQUEST_DURATION_SECONDS
        .with_label_values(&[method.as_str(), status.as_str(), route.as_str()])
        .observe(start.elapsed().as_secs_f64());

    response
}
