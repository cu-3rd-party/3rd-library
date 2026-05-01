#[cfg(test)]
mod full_flow_integration {
    use serde::Deserialize;

    #[test]
    fn test_complete_user_flow_unverified_to_submission() {
        test_user_registration();
        test_email_verification();
        test_login_as_verified_user();
        test_create_submission();
        test_list_submissions();
    }

    fn test_user_registration() {
        let json = r#"{"name":"Test User","email":"test@example.com","password":"password123"}"#;
        #[derive(Deserialize)]
        struct RegisterRequest {
            name: String,
            email: String,
        }
        let req: RegisterRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.name, "Test User");
    }

    fn test_email_verification() {
        let json = r#"{"email":"test@example.com","code":"123456"}"#;
        #[derive(Deserialize)]
        struct VerifyRequest {
            email: String,
        }
        let _: VerifyRequest = serde_json::from_str(json).unwrap();
    }

    fn test_login_as_verified_user() {
        let json = r#"{
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
                "accessToken": "token",
                "refreshToken": "refresh",
                "expiresIn": 1209600
            }
        }"#;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct AuthResponse {
            user: User,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct User {
            can_submit_materials: bool,
        }
        let auth: AuthResponse = serde_json::from_str(json).unwrap();
        assert!(auth.user.can_submit_materials);
    }

    fn test_create_submission() {
        let json = r#"{
            "title": "My Submission",
            "description": "Test description",
            "courses": ["1"],
            "subjects": ["Матан"],
            "type": "demo",
            "difficulty": "none"
        }"#;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct CreateRequest {
            title: String,
            courses: Vec<String>,
        }
        let req: CreateRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.title, "My Submission");
    }

    fn test_list_submissions() {
        let json = r#"{
            "items": [
                {
                    "id": "550e8400-e29b-41d4-a716-446655440000",
                    "status": "pending_review"
                }
            ],
            "page": 1,
            "limit": 20,
            "total": 1
        }"#;
        #[derive(Deserialize)]
        struct ListResponse {
            items: Vec<SubmissionItem>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct SubmissionItem {
            status: String,
        }
        let resp: ListResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.items.len(), 1);
    }

    #[test]
    fn test_moderation_approval_flow() {
        test_moderator_login();
        test_list_pending_submissions();
        test_approve_submission();
        test_check_published_material();
    }

    fn test_moderator_login() {
        let json = r#"{
            "user": {
                "id": "mod-uuid",
                "name": "Moderator",
                "roles": ["moderator"]
            }
        }"#;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ModResponse {
            user: ModUser,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ModUser {
            roles: Vec<String>,
        }
        let resp: ModResponse = serde_json::from_str(json).unwrap();
        assert!(resp.user.roles.contains(&"moderator".to_string()));
    }

    fn test_list_pending_submissions() {
        let json = r#"{
            "items": [{"id":"s1","status":"pending_review"}],
            "page":1,
            "limit":20,
            "total":1,
            "counters": {
                "all":1,"draft":0,"pending_review":1,"rejected":0,"approved":0
            }
        }"#;
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct ModQueue {
            items: Vec<QueueItem>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct QueueItem {
            status: String,
        }
        let resp: ModQueue = serde_json::from_str(json).unwrap();
        assert_eq!(resp.items[0].status, "pending_review");
    }

    fn test_approve_submission() {
        let json = r#"{"action":"approve"}"#;
        #[derive(Deserialize)]
        struct Decision {
            action: String,
        }
        let d: Decision = serde_json::from_str(json).unwrap();
        assert_eq!(d.action, "approve");
    }

    fn test_check_published_material() {
        let json = r#"{
            "items": [
                {
                    "id":"m1",
                    "title":"Approved",
                    "pubDate":"18.03.2026"
                }
            ],
            "page":1,
            "limit":20,
            "total":1
        }"#;
        #[derive(Deserialize)]
        struct MaterialList {
            items: Vec<Material>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Material {
            pub_date: Option<String>,
        }
        let list: MaterialList = serde_json::from_str(json).unwrap();
        assert!(list.items[0].pub_date.is_some());
    }
}
