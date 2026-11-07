mod common;

use crate::common::TestApp;
use axum::http::StatusCode;

#[tokio::test]
async fn health_and_openapi_are_served() {
    let app = TestApp::spawn().await;

    let health = app.get("/api/health", None).await;
    let openapi = app.get("/openapi/v1.json", None).await;

    assert_eq!(health.body["status"], "ok");
    assert_eq!(openapi.status, StatusCode::OK);
    let paths = openapi.body["paths"].as_object().unwrap();
    for path in [
        "/api/auth/register",
        "/api/users/{userId}",
        "/api/materials/submissions/{submissionId}",
        "/api/moderation/submissions/{submissionId}/decision",
    ] {
        assert!(paths.contains_key(path), "{path} is missing in OpenAPI");
    }
    assert!(openapi.body["components"]["securitySchemes"]["bearer_auth"].is_object());
}
