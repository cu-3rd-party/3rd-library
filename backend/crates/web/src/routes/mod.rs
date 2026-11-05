pub mod auth;
pub mod health;
pub mod materials;
pub mod moderation;
pub mod submissions;
pub mod users;

use chrono::{DateTime, Utc};
use utoipa_axum::router::OpenApiRouter;

use crate::AppState;

/// Все эндпоинты `/api`. Новая фича — модуль рядом и строка здесь.
pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .merge(health::router())
        .merge(auth::router())
        .merge(users::router())
        .merge(materials::router())
        .merge(submissions::router())
        .merge(moderation::router())
}

/// Формат времени в ответах API: `2026-01-02T03:04:05.123456Z`.
pub(crate) fn timestamp(value: DateTime<Utc>) -> String {
    value.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string()
}
