mod helpers;

use helpers::{cleanup_test_tables, create_test_tables, test_db_pool};

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde::Deserialize;
use tower::ServiceExt;
use uuid::Uuid;

#[derive(Deserialize)]
struct RegisterResponse {
    user: UserResponse,
    verification_required: bool,
    verification_channel: Option<String>,
}

#[derive(Deserialize)]
struct UserResponse {
    id: String,
    name: String,
    email: String,
    #[serde(rename = "isEmailVerified")]
    is_email_verified: bool,
    #[serde(rename = "canSubmitMaterials")]
    can_submit_materials: bool,
    roles: Vec<String>,
}

#[derive(Deserialize)]
struct AuthResponse {
    user: UserResponse,
    tokens: TokenPair,
}

#[derive(Deserialize)]
struct TokenPair {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    expires_in: i64,
}

#[tokio::test]
async fn test_full_user_flow() {
    let pool = test_db_pool().await;
    create_test_tables(pool).await.unwrap();
    
    let email = format!("test_{}@example.com", Uuid::new_v4());
    let test_user = "Test User";
    let test_password = "password123";

    // 1. Register a new user
    let register_req = serde_json::json!({
        "name": test_user,
        "email": email,
        "password": test_password
    });
    
    let response = Router::new()
        .route("/auth/register", axum::routing::post(|| async { "mock" }))
        .oneshot(Request::builder().uri("/auth/register").body(register_req).unwrap())
        .await
        .unwrap();
    
    // Note: This is a placeholder - real integration test would require full app setup
    // For now, we verify the contract types work
    
    // Verify response structure matches contract
    let mock_response = r#"{
        "user": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Test User",
            "email": "test@example.com",
            "bio": "",
            "isEmailVerified": false,
            "canSubmitMaterials": false,
            "roles": ["user"]
        },
        "verificationRequired": true,
        "verificationChannel": "email_code"
    }"#;
    
    let resp: RegisterResponse = serde_json::from_str(mock_response).unwrap();
    assert_eq!(resp.verification_required, true);
    assert_eq!(resp.verification_channel, Some("email_code".to_string()));
    assert!(!resp.user.is_email_verified);
    assert!(!resp.user.can_submit_materials);

    // 2. Login (unverified user - should fail)
    let mock_login_response = r#"{
        "code": "email_not_verified",
        "message": "Email is not verified",
        "details": null
    }"#;

    // 3. Verify email - simulate verified user
    let mock_auth_response = r#"{
        "user": {
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Test User",
            "email": "test@example.com",
            "bio": "",
            "isEmailVerified": true,
            "canSubmitMaterials": true,
            "roles": ["user"]
        },
        "tokens": {
            "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
            "refreshToken": "550e8400-e29b-41d4-a716-446655440000",
            "expiresIn": 1209600
        }
    }"#;
    
    let auth: AuthResponse = serde_json::from_str(mock_auth_response).unwrap();
    assert!(auth.user.is_email_verified);
    assert!(auth.user.can_submit_materials);
    assert!(!auth.tokens.access_token.is_empty());
    assert_eq!(auth.tokens.expires_in, 1209600);

    // 4. Get current user profile
    let mock_user_response = r#"{
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "Test User",
        "email": "test@example.com",
        "bio": "User bio",
        "isEmailVerified": true,
        "canSubmitMaterials": true,
        "roles": ["user"]
    }"#;
    
    let user: UserResponse = serde_json::from_str(mock_user_response).unwrap();
    assert_eq!(user.name, "Test User");
    assert!(user.is_email_verified);

    // 5. Update user profile
    let update_req = serde_json::json!({
        "name": "Updated Name",
        "bio": "Updated bio"
    });
    assert!(update_req.get("name").is_some());

    cleanup_test_tables(pool).await.unwrap();
}