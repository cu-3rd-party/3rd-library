use axum::body::Body;
use axum::http::{HeaderValue, StatusCode};
use axum::response::Response;
use prometheus::{Encoder, TextEncoder};

use super::REGISTRY;

pub async fn metrics_handler() -> Result<Response, StatusCode> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = String::from_utf8(buffer).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut response = Response::new(Body::from(body));
    response.headers_mut().insert(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; version=0.0.4"),
    );

    Ok(response)
}
