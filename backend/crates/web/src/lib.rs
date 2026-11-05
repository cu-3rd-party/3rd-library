//! Web-слой: HTTP-контракт (request/response модели, OpenAPI), маппинг ошибок, аутентификация запросов,
//! rate limit, HTTP-метрики и сборка зависимостей. Бизнес-логики здесь нет — только вызовы сервисов приложения.

pub mod auth;
pub mod config;
pub mod error;
pub mod extract;
pub mod metrics;
pub mod rate_limit;
pub mod routes;
pub mod state;

use axum::Router;
use axum::http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User profile and discovery endpoints"),
        (name = "materials", description = "Material catalogue and submission endpoints"),
        (name = "moderation", description = "Moderator-only review endpoints"),
        (name = "system", description = "Service health and system endpoints")
    ),
    info(
        title = "3rd Library Backend API",
        description = "OpenAPI documentation for the 3rd-library backend"
    )
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme("bearer_auth", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)));
    }
}

/// Полный роутер API: `/api/...`, Swagger UI на `/api/docs/swagger-ui`, rate limit, CORS, метрики, трейсинг.
pub fn app(state: AppState, cors: CorsLayer) -> Router {
    let (router, openapi) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::router())
        .split_for_parts();

    router
        .merge(SwaggerUi::new("/api/docs/swagger-ui").url("/api/docs/openapi.json", openapi))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            rate_limit::rate_limit_middleware,
        ))
        .with_state(state)
        .layer(cors)
        .layer(axum::middleware::from_fn(metrics::metrics_middleware))
        .layer(TraceLayer::new_for_http())
}

/// `CORS_ALLOWED_ORIGIN`: пусто или `*` — любой origin, иначе ровно указанный.
pub fn cors_layer(allowed_origin: &str) -> anyhow::Result<CorsLayer> {
    let cors = CorsLayer::new().allow_methods(Any).allow_headers(Any);
    Ok(match allowed_origin {
        "" | "*" => cors.allow_origin(Any),
        origin => cors.allow_origin(
            HeaderValue::from_str(origin).map_err(|_| anyhow::anyhow!("invalid CORS_ALLOWED_ORIGIN: {origin}"))?,
        ),
    })
}
