use anyhow::Context;
use log::info;
use redis::aio::ConnectionManager;
use redis::{Client, TypedCommands};
use reqwest;
use serde::Deserialize;
use sqlx::Connection;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

const BASE_URL: &str = "http://localhost:8080";

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct ApiError {
    code: String,
    message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct WebUser {
    id: String,
    name: String,
    email: String,
    bio: String,
    is_email_verified: bool,
    can_submit_materials: bool,
    roles: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct TokenPair {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct AuthResponse {
    user: WebUser,
    tokens: TokenPair,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct RegisterResponse {
    user: WebUser,
    verification_required: bool,
    verification_channel: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Material {
    id: String,
    author_id: String,
    author_name: Option<String>,
    title: String,
    description: String,
    courses: Vec<String>,
    subjects: Vec<String>,
    #[serde(rename = "type")]
    r#type: String,
    difficulty: String,
    pub_date: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PaginatedMaterialsResponse {
    items: Vec<Material>,
    page: i64,
    limit: i64,
    total: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserPublicProfile {
    id: String,
    name: String,
    bio: String,
    is_email_verified: bool,
    materials_count: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PaginatedUsersResponse {
    items: Vec<UserPublicProfile>,
    page: i64,
    limit: i64,
    total: i64,
}

fn auth_client() -> reqwest::Client {
    reqwest::Client::new()
}

#[tokio::test]
async fn test_auth_register() {
    setup_api().await.unwrap();
    let client = auth_client();
    let random_email = format!("test_{}@example.com", uuid::Uuid::new_v4());

    let response = client
        .post(format!("{}/auth/register", BASE_URL))
        .json(&serde_json::json!({
            "name": "Test User",
            "email": random_email,
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success(), "Expected 201 Created");

    let body: RegisterResponse = response.json().await.unwrap();

    assert!(!body.user.id.is_empty());
    assert_eq!(body.user.name, "Test User");
    assert_eq!(body.user.email, random_email);
    assert!(!body.user.is_email_verified);
    assert!(!body.user.can_submit_materials);
    assert_eq!(body.user.roles, vec!["user"]);
    assert!(body.verification_required);
    assert_eq!(body.verification_channel, Some("email_code".to_string()));
}

#[tokio::test]
async fn test_auth_register_invalid_email() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/register", BASE_URL))
        .json(&serde_json::json!({
            "name": "Test",
            "email": "not-an-email",
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert!(
        response.status().is_client_error(),
        "Expected 400 Bad Request, received: {}",
        response.status().to_string()
    );

    let error: ApiError = response.json().await.unwrap();
    assert_eq!(error.code, "bad_request");
}

#[tokio::test]
async fn test_auth_register_duplicate_email() {
    setup_api().await.unwrap();
    let client = auth_client();
    let email = format!("duplicate_{}@example.com", uuid::Uuid::new_v4());

    let _ = client
        .post(format!("{}/auth/register", BASE_URL))
        .json(&serde_json::json!({
            "name": "User One",
            "email": email,
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    let response2 = client
        .post(format!("{}/auth/register", BASE_URL))
        .json(&serde_json::json!({
            "name": "User Two",
            "email": email,
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response2.status(), 409, "Expected 409 Conflict");

    let error: ApiError = response2.json().await.unwrap();
    assert_eq!(error.code, "conflict");
}

#[tokio::test]
async fn test_auth_verify_email_bad_request() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/verify-email", BASE_URL))
        .json(&serde_json::json!({
            "email": "notregistered@example.com",
            "code": "123456"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404, "Expected 404 Not Found");
}

#[tokio::test]
async fn test_auth_resend_verification_code() {
    setup_api().await.unwrap();
    let client = auth_client();
    let email = format!("resend_{}@example.com", uuid::Uuid::new_v4());

    let _ = client
        .post(format!("{}/auth/register", BASE_URL))
        .json(&serde_json::json!({
            "name": "Resend Test",
            "email": email,
            "password": "password123"
        }))
        .send()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/auth/resend-verification-code", BASE_URL))
        .json(&serde_json::json!({
            "email": email
        }))
        .send()
        .await
        .unwrap();

    assert!(
        response.status().is_client_error(),
        "Expected 429 Too many requests, got: {}",
        response.status()
    );
}

#[tokio::test]
async fn test_auth_resend_verification_code_not_found() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/resend-verification-code", BASE_URL))
        .json(&serde_json::json!({
            "email": "nonexistent@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404, "Expected 404 Not Found");
}

#[tokio::test]
async fn test_auth_login_bad_request() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/login", BASE_URL))
        .json(&serde_json::json!({
            "email": "notregistered@example.com",
            "password": "password"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 400 Bad Request");
}

#[tokio::test]
async fn test_auth_login_invalid_credentials() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/login", BASE_URL))
        .json(&serde_json::json!({
            "email": "nonexistent@example.com",
            "password": "wrongpassword"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    let error: ApiError = response.json().await.unwrap();
    assert_eq!(error.code, "unauthorized");
}

#[tokio::test]
async fn test_auth_refresh_token_bad_request() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/refresh", BASE_URL))
        .json(&serde_json::json!({
            "refreshToken": ""
        }))
        .send()
        .await
        .unwrap();

    assert!(
        response.status().is_client_error(),
        "Expected 400 Bad Request"
    );
}

#[tokio::test]
async fn test_auth_refresh_token_unauthorized() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/refresh", BASE_URL))
        .json(&serde_json::json!({
            "refreshToken": "invalid-token"
        }))
        .send()
        .await
        .unwrap();

    assert!(
        response.status().is_client_error(),
        "Expected 400 Bad Request"
    );
}

#[tokio::test]
async fn test_auth_logout_unauthenticated() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/auth/logout", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");
}

#[tokio::test]
async fn test_users_me_unauthorized() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!("{}/users/me", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    let error: ApiError = response.json().await.unwrap();
    assert_eq!(error.code, "unauthorized");
}

#[tokio::test]
async fn test_users_list() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!("{}/users", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200, "Expected 200 OK");

    let body: PaginatedUsersResponse = response.json().await.unwrap();
    assert!(body.page >= 1);
    assert!(body.limit >= 1);
    assert!(body.total >= 0);
}

#[tokio::test]
async fn test_users_list_with_pagination() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!("{}/users?page=1&limit=10", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200, "Expected 200 OK");

    let body: PaginatedUsersResponse = response.json().await.unwrap();
    assert_eq!(body.page, 1);
    assert_eq!(body.limit, 10);
}

#[tokio::test]
async fn test_users_get_by_id_not_found() {
    setup_api().await.unwrap();
    let client = auth_client();
    let nonexistent_id = uuid::Uuid::nil();

    let response = client
        .get(format!("{}/users/{}", BASE_URL, nonexistent_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404, "Expected 404 Not Found");

    let error: ApiError = response.json().await.unwrap();
    assert_eq!(error.code, "not_found");
}

#[tokio::test]
async fn test_materials_list() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!("{}/materials", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200, "Expected 200 OK");

    let body: PaginatedMaterialsResponse = response.json().await.unwrap();
    assert!(body.page >= 1);
    assert!(body.limit >= 1);
    assert!(body.total >= 0);
}

#[tokio::test]
async fn test_materials_list_with_filters() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!(
            "{}/materials?search=test&courses=1&subjects=Английский&types=demo&difficulties=none",
            BASE_URL
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200, "Expected 200 OK");

    let body: PaginatedMaterialsResponse = response.json().await.unwrap();
    for m in &body.items {
        assert!(m.description.len() <= 10000);
    }
}

#[tokio::test]
async fn test_materials_list_pagination() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!("{}/materials?page=2&limit=50", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200, "Expected 200 OK");

    let body: PaginatedMaterialsResponse = response.json().await.unwrap();
    assert_eq!(body.page, 2);
    assert_eq!(body.limit, 50);
}

#[tokio::test]
async fn test_materials_get_by_id_not_found() {
    setup_api().await.unwrap();
    let client = auth_client();
    let nonexistent_id = uuid::Uuid::nil();

    let response = client
        .get(format!("{}/materials/{}", BASE_URL, nonexistent_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404, "Expected 404 Not Found");

    let error: ApiError = response.json().await.unwrap();
    assert_eq!(error.code, "not_found");
}

#[tokio::test]
async fn test_materials_submissions_list_unauthorized() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!("{}/materials/submissions", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");
}

#[tokio::test]
async fn test_materials_submissions_create_unauthorized() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!("{}/materials/submissions", BASE_URL))
        .json(&serde_json::json!({
            "title": "Test Material",
            "courses": ["1"],
            "subjects": ["Английский"],
            "type": "demo",
            "difficulty": "none"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");
}

#[tokio::test]
async fn test_materials_submissions_get_not_found() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!(
            "{}/materials/submissions/{}",
            BASE_URL,
            uuid::Uuid::nil()
        ))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");
}

#[tokio::test]
async fn test_materials_submissions_update_not_found() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .patch(format!(
            "{}/materials/submissions/{}",
            BASE_URL,
            uuid::Uuid::nil()
        ))
        .json(&serde_json::json!({
            "title": "Updated Title"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");
}

#[tokio::test]
async fn test_moderation_list_unauthorized() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .get(format!("{}/moderation/submissions", BASE_URL))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    let error: ApiError = response.json().await.unwrap();
    assert_eq!(error.code, "unauthorized");
}

#[tokio::test]
async fn test_moderation_decision_unauthorized() {
    setup_api().await.unwrap();
    let client = auth_client();

    let response = client
        .post(format!(
            "{}/moderation/submissions/{}/decision",
            BASE_URL,
            uuid::Uuid::nil()
        ))
        .json(&serde_json::json!({
            "action": "approve"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");
}

async fn setup_api() -> anyhow::Result<()> {
    info!("Starting full flow test...");

    let config = backend::config::Config::init_from_env();

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.db.database_url())
        .await
        .context("could not connect to database_url")?;
    db.acquire()
        .await?
        .ping()
        .await
        .context("could not ping database")?;
    info!("Database ping successful");
    info!("Successfully connected to database");

    let redis_client =
        Client::open(config.redis.url.clone()).context("could not build Redis client")?;
    let mut redis_client = redis_client.clone();
    redis_client.ping()?;
    info!("Redis ping successful");
    let redis = ConnectionManager::new(redis_client)
        .await
        .context("could not connect to Redis")?;
    info!("Successfully connected to redis");

    tokio::spawn(async move {
        if let Err(err) = sqlx::migrate!().run(&db).await {
            log::warn!("skipping migrations: {err}");
        }

        backend::http::serve(config, db, redis).await.unwrap();
    });

    loop {
        let response = reqwest::get("http://localhost:8080/api/health").await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    break;
                }
            }
            Err(_) => {
                info!("Failed to healthcheck backend");
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    info!("health check passed");

    Ok(())
}
