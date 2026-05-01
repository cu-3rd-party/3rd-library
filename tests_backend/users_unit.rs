#[cfg(test)]
mod tests {
    use crate::http::users::models::*;
    use serde_json;

    #[test]
    fn test_web_user_model_serialization() {
        let user = WebUser {
            id: uuid::Uuid::new_v4(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            bio: "Some bio".to_string(),
            is_email_verified: true,
            can_submit_materials: true,
            roles: vec!["user".to_string()],
        };
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("john@example.com"));
        assert!(json.contains("Some bio"));
    }

    #[test]
    fn test_web_user_model_deserialization() {
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
        assert_eq!(user.email, "john@example.com");
        assert!(user.is_email_verified);
    }

    #[test]
    fn test_update_user_request_deserialization() {
        let json = r#"{"name":"New Name","bio":"New bio"}"#;
        let req: UpdateUserRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, Some("New Name".to_string()));
        assert_eq!(req.bio, Some("New bio".to_string()));
    }

    #[test]
    fn test_update_user_request_partial() {
        let json = r#"{"name":"New Name"}"#;
        let req: UpdateUserRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, Some("New Name".to_string()));
        assert_eq!(req.bio, None);
    }
}