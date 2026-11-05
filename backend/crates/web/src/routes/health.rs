use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::extract::Json;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(health_check))
}

#[derive(Serialize, ToSchema)]
pub struct HealthCheck {
    status: String,
    timestamp: String,
}

#[utoipa::path(get, path = "/api/health", tag = "system",
    responses((status = 200, description = "Service is healthy", body = HealthCheck)))]
async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "ok".into(),
        timestamp: chrono::Local::now().to_rfc2822(),
    })
}
