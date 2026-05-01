#[cfg(test)]
mod integration_contract {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct RegisterRequest {
        name: String,
        email: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct RegisterResponse {
        user: UserResponse,
        verification_required: bool,
        verification_channel: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct UserResponse {
        id: String,
        name: String,
        email: String,
        bio: String,
        is_email_verified: bool,
        can_submit_materials: bool,
        roles: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    struct AuthResponse {
        user: UserResponse,
        tokens: TokenPair,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct TokenPair {
        access_token: String,
        refresh_token: String,
        expires_in: i64,
    }

    #[derive(Deserialize, Debug)]
    struct MaterialResponse {
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

    #[derive(Deserialize, Debug)]
    struct PaginatedMaterialsResponse {
        items: Vec<MaterialResponse>,
        page: i64,
        limit: i64,
        total: i64,
    }

    #[derive(Deserialize, Debug)]
    struct SubmissionResponse {
        id: String,
        material: MaterialResponse,
        files: Vec<MaterialFileResponse>,
        status: String,
        moderator_comment: String,
        created_at: String,
        updated_at: String,
        submitted_at: Option<String>,
        reviewed_at: Option<String>,
        published_at: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct MaterialFileResponse {
        id: String,
        name: String,
        size_bytes: i64,
        extension: String,
        mime_type: Option<String>,
        url: Option<String>,
    }

    #[test]
    fn test_user_register_request_contract() {
        let register_json = serde_json::json!({
            "name": "Test User",
            "email": "test@example.com",
            "password": "password123"
        });
        let req: RegisterRequest = serde_json::from_value(register_json).unwrap();
        assert_eq!(req.name, "Test User");
        assert_eq!(req.email, "test@example.com");
    }

    #[test]
    fn test_user_register_response_contract() {
        let response_json = r#"{
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
        let resp: RegisterResponse = serde_json::from_str(response_json).unwrap();
        assert!(resp.verification_required);
        assert_eq!(resp.verification_channel, Some("email_code".to_string()));
        assert!(!resp.user.is_email_verified);
    }

    #[test]
    fn test_auth_response_contract() {
        let verified_json = r#"{
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
                "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature",
                "refreshToken": "660e8400-e29b-41d4-a716-446655440001",
                "expiresIn": 1209600
            }
        }"#;
        let auth: AuthResponse = serde_json::from_str(verified_json).unwrap();
        assert!(auth.user.is_email_verified);
        assert!(auth.user.can_submit_materials);
        assert!(!auth.tokens.access_token.is_empty());
        assert_eq!(auth.tokens.expires_in, 1209600);
    }

    #[test]
    fn test_user_profile_response_contract() {
        let profile_json = r#"{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "Test User",
            "email": "test@example.com",
            "bio": "User biography",
            "isEmailVerified": true,
            "canSubmitMaterials": true,
            "roles": ["user"]
        }"#;
        let user: UserResponse = serde_json::from_str(profile_json).unwrap();
        assert_eq!(user.name, "Test User");
        assert!(user.is_email_verified);
    }

    #[test]
    fn test_materials_list_response_contract() {
        let materials_json = r#"{
            "items": [
                {
                    "id": "550e8400-e29b-41d4-a716-446655440000",
                    "authorId": "660e8400-e29b-41d4-a716-446655440001",
                    "authorName": "Author Name",
                    "title": "Test Material",
                    "description": "Description",
                    "courses": ["1"],
                    "subjects": ["Матан"],
                    "type": "demo",
                    "difficulty": "none",
                    "pubDate": "18.03.2026"
                }
            ],
            "page": 1,
            "limit": 20,
            "total": 1
        }"#;
        let resp: PaginatedMaterialsResponse = serde_json::from_str(materials_json).unwrap();
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].title, "Test Material");
        assert_eq!(resp.items[0].r#type, "demo");
        assert_eq!(resp.page, 1);
        assert_eq!(resp.total, 1);
    }

    #[test]
    fn test_submission_response_contract() {
        let submission_json = r#"{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "material": {
                "id": "770e8400-e29b-41d4-a716-446655440002",
                "authorId": "660e8400-e29b-41d4-a716-446655440001",
                "authorName": "Author",
                "title": "My Submission",
                "description": "Description",
                "courses": ["1"],
                "subjects": ["Матан"],
                "type": "demo",
                "difficulty": "none",
                "pubDate": null
            },
            "files": [],
            "status": "pending_review",
            "moderatorComment": "",
            "createdAt": "2026-03-18T10:00:00Z",
            "updatedAt": "2026-03-18T10:00:00Z",
            "submittedAt": "2026-03-18T10:00:00Z",
            "reviewedAt": null,
            "publishedAt": null
        }"#;
        let sub: SubmissionResponse = serde_json::from_str(submission_json).unwrap();
        assert_eq!(sub.status, "pending_review");
        assert_eq!(sub.material.title, "My Submission");
        assert!(sub.submitted_at.is_some());
    }
}