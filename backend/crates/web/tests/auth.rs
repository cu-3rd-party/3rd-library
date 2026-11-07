mod common;

use axum::http::{header, StatusCode};
use common::TestApp;
use serde_json::json;

#[tokio::test]
async fn register_returns_unverified_user_and_sends_code() {
    let app = TestApp::spawn().await;

    let response = app.register("new@example.com").await;

    assert_eq!(response.status, StatusCode::OK);
    let user = &response.body["user"];
    assert_eq!(user["email"], "new@example.com");
    assert_eq!(user["is_email_verified"], false);
    assert_eq!(user["can_submit_materials"], false);
    assert_eq!(user["roles"], json!(["user"]));
    assert_eq!(response.body["verification_required"], true);
    assert_eq!(response.body["verification_channel"], "email_code");
    assert_eq!(app.code_for("new@example.com").len(), 6);
}

#[tokio::test]
async fn register_errors_use_api_error_format() {
    let app = TestApp::spawn().await;
    app.register("taken@example.com").await;

    let invalid = app.register("not-an-email").await;
    let taken = app.register("taken@example.com").await;

    assert_eq!(invalid.status, StatusCode::BAD_REQUEST);
    assert_eq!(invalid.body["code"], "bad_request");
    assert_eq!(invalid.body["details"][0]["field"], "email");
    assert_eq!(taken.status, StatusCode::CONFLICT);
    assert_eq!(
        taken.body,
        json!({"code": "conflict", "message": "Email already exists", "details": null})
    );
}

#[tokio::test]
async fn verify_email_checks_code() {
    let app = TestApp::spawn().await;
    app.register("v@example.com").await;
    let code = app.code_for("v@example.com");

    let wrong = app
        .post(
            "/api/auth/verify-email",
            None,
            json!({"email": "v@example.com", "code": "000000x"}),
        )
        .await;
    let unknown = app
        .post(
            "/api/auth/verify-email",
            None,
            json!({"email": "ghost@example.com", "code": code}),
        )
        .await;
    let ok = app
        .post(
            "/api/auth/verify-email",
            None,
            json!({"email": "v@example.com", "code": code}),
        )
        .await;
    let again = app
        .post(
            "/api/auth/verify-email",
            None,
            json!({"email": "v@example.com", "code": code}),
        )
        .await;

    assert_eq!(wrong.status, StatusCode::BAD_REQUEST);
    assert_eq!(unknown.status, StatusCode::NOT_FOUND);
    assert_eq!(ok.status, StatusCode::OK);
    assert_eq!(ok.body["user"]["is_email_verified"], true);
    assert_eq!(ok.body["tokens"]["expires_in"], 1_209_600);
    assert_eq!(again.status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn resend_code_is_rate_limited() {
    let app = TestApp::spawn().await;
    app.register("r@example.com").await;

    let response = app
        .post(
            "/api/auth/resend-verification-code",
            None,
            json!({"email": "r@example.com"}),
        )
        .await;
    let unknown = app
        .post(
            "/api/auth/resend-verification-code",
            None,
            json!({"email": "ghost@example.com"}),
        )
        .await;

    assert_eq!(response.status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(unknown.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn login_requires_password_and_verified_email() {
    let app = TestApp::spawn().await;
    app.register("pending@example.com").await;
    app.verified_user("ok@example.com").await;
    let login = |email: &str, password: &str| json!({"email": email, "password": password});

    let wrong = app.post("/api/auth/login", None, login("ok@example.com", "nope")).await;
    let unverified = app
        .post("/api/auth/login", None, login("pending@example.com", "password123"))
        .await;
    let ok = app
        .post("/api/auth/login", None, login("ok@example.com", "password123"))
        .await;

    assert_eq!(wrong.status, StatusCode::UNAUTHORIZED);
    assert_eq!(wrong.headers[header::WWW_AUTHENTICATE], "Bearer");
    assert_eq!(wrong.body["code"], "unauthorized");
    assert_eq!(unverified.status, StatusCode::FORBIDDEN);
    assert_eq!(ok.status, StatusCode::OK);
    assert_eq!(ok.body["user"]["can_submit_materials"], true);
}

#[tokio::test]
async fn refresh_rotates_tokens_and_logout_ends_session() {
    let app = TestApp::spawn().await;
    let (_, access, refresh) = app.verified_user("s@example.com").await;

    let refreshed = app
        .post("/api/auth/refresh", None, json!({"refreshToken": refresh}))
        .await;
    assert_eq!(refreshed.status, StatusCode::OK, "{}", refreshed.body);
    let new_access = refreshed.body["access_token"].as_str().unwrap();

    assert_eq!(
        app.get("/api/users/me", Some(&access)).await.status,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(app.get("/api/users/me", Some(new_access)).await.status, StatusCode::OK);
    let reused = app
        .post("/api/auth/refresh", None, json!({"refreshToken": refresh}))
        .await;
    assert_eq!(reused.status, StatusCode::UNAUTHORIZED);

    let logout = app
        .json(axum::http::Method::POST, "/api/auth/logout", Some(new_access), None)
        .await;
    assert_eq!(logout.status, StatusCode::OK);
    assert_eq!(
        app.get("/api/users/me", Some(new_access)).await.status,
        StatusCode::UNAUTHORIZED
    );
}

#[tokio::test]
async fn protected_routes_reject_missing_or_bad_tokens() {
    let app = TestApp::spawn().await;

    assert_eq!(app.get("/api/users/me", None).await.status, StatusCode::UNAUTHORIZED);
    assert_eq!(
        app.get("/api/users/me", Some("garbage")).await.status,
        StatusCode::UNAUTHORIZED
    );
    // публичный каталог с неверным токеном — тоже 401, как раньше
    assert_eq!(
        app.get("/api/materials", Some("garbage")).await.status,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(app.get("/api/materials", None).await.status, StatusCode::OK);
}

#[tokio::test]
async fn profile_update_keeps_missing_fields() {
    let app = TestApp::spawn().await;
    let (_, token, _) = app.verified_user("p@example.com").await;

    let response = app
        .json(
            axum::http::Method::PATCH,
            "/api/users/me",
            Some(&token),
            Some(json!({"bio": "hello"})),
        )
        .await;

    assert_eq!(response.status, StatusCode::OK);
    assert_eq!(response.body["bio"], "hello");
    assert_eq!(response.body["name"], "User p@example.com");
}

