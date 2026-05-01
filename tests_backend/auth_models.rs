use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

#[derive(Deserialize)]
pub struct ResendVerificationCodeRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct WebUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub bio: String,
    pub is_email_verified: bool,
    pub can_submit_materials: bool,
    pub roles: Vec<String>,
}

#[derive(Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user: WebUser,
    pub tokens: TokenPair,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user: WebUser,
    pub verification_required: bool,
    pub verification_channel: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_request_deserialization() {
        let json = r#"{"name":"Test User","email":"test@example.com","password":"password123"}"#;
        let req: RegisterRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "Test User");
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.password, "password123");
    }

    #[test]
    fn test_verify_email_request_deserialization() {
        let json = r#"{"email":"test@example.com","code":"123456"}"#;
        let req: VerifyEmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.code, "123456");
    }

    #[test]
    fn test_login_request_deserialization() {
        let json = r#"{"email":"test@example.com","password":"password123"}"#;
        let req: LoginRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.password, "password123");
    }

    #[test]
    fn test_refresh_token_request_deserialization() {
        let json = r#"{"refresh_token":"some-uuid"}"#;
        let req: RefreshTokenRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.refresh_token, "some-uuid");
    }

    #[test]
    fn test_web_user_serialization() {
        let user = WebUser {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            bio: "Bio text".to_string(),
            is_email_verified: true,
            can_submit_materials: true,
            roles: vec!["user".to_string()],
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("Test User"));
    }

    #[test]
    fn test_web_user_deserialization() {
        let json = r#"{
            "id":"550e8400-e29b-41d4-a716-446655440000",
            "name":"John Doe",
            "email":"john@example.com",
            "bio":"Some bio",
            "isEmailVerified":true,
            "canSubmitMaterials":true,
            "roles":["user"]
        }"#;
        let user: WebUser = serde_json::from_str(json).unwrap();
        assert_eq!(user.name, "John Doe");
        assert!(user.is_email_verified);
    }

    #[test]
    fn test_token_pair_serialization() {
        let tokens = TokenPair {
            access_token: "access_token".to_string(),
            refresh_token: "refresh_token".to_string(),
            expires_in: 1209600,
        };
        let json = serde_json::to_string(&tokens).unwrap();
        assert!(json.contains("access_token"));
    }

    #[test]
    fn test_auth_response_serialization() {
        let user = WebUser {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            bio: "Bio".to_string(),
            is_email_verified: true,
            can_submit_materials: true,
            roles: vec!["user".to_string()],
        };
        let tokens = TokenPair {
            access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            expires_in: 1209600,
        };
        let response = AuthResponse { user, tokens };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Test User"));
    }

    #[test]
    fn test_register_response_serialization() {
        let user = WebUser {
            id: Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            bio: "".to_string(),
            is_email_verified: false,
            can_submit_materials: false,
            roles: vec!["user".to_string()],
        };
        let response = RegisterResponse {
            user,
            verification_required: true,
            verification_channel: Some("email_code".to_string()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("verification_required"));
    }
}