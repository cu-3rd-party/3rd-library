#[cfg(test)]
mod tests {
    use crate::http::auth::models::*;
    use crate::http::error::Error;
    use serde_json;

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
    fn test_resend_verification_request_deserialization() {
        let json = r#"{"email":"test@example.com"}"#;
        let req: ResendVerificationCodeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.email, "test@example.com");
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
            id: uuid::Uuid::new_v4(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            bio: "Bio text".to_string(),
            is_email_verified: true,
            can_submit_materials: true,
            roles: vec!["user".to_string()],
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("Test User"));
        assert!(json.contains("test@example.com"));
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
        assert!(json.contains("refresh_token"));
    }

    #[test]
    fn test_auth_response_serialization() {
        let user = WebUser {
            id: uuid::Uuid::new_v4(),
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
        assert!(json.contains("access"));
    }

    #[test]
    fn test_register_response_serialization() {
        let user = WebUser {
            id: uuid::Uuid::new_v4(),
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
        assert!(json.contains("email_code"));
    }
}