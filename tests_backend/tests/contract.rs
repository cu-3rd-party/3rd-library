#[cfg(test)]
mod tests {
    mod auth_tests {
        use serde::Deserialize;

        #[test]
        fn test_register_request_contract() {
            let json =
                r#"{"name":"Test User","email":"test@example.com","password":"password123"}"#;
            #[derive(Deserialize)]
            struct RegisterRequest {
                name: String,
                email: String,
                password: String,
            }
            let req: RegisterRequest = serde_json::from_str(json).unwrap();
            assert_eq!(req.name, "Test User");
            assert_eq!(req.email, "test@example.com");
        }

        #[test]
        fn test_register_response_contract() {
            let json = r#"{
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
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
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
            }
            let resp: RegisterResponse = serde_json::from_str(json).unwrap();
            assert!(resp.verification_required);
            assert_eq!(resp.verification_channel, Some("email_code".to_string()));
        }

        #[test]
        fn test_auth_response_contract() {
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
                    "accessToken": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.signature",
                    "refreshToken": "660e8400-e29b-41d4-a716-446655440001",
                    "expiresIn": 1209600
                }
            }"#;
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct AuthResponse {
                user: UserResponse,
                tokens: TokenPair,
            }
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct UserResponse {
                is_email_verified: bool,
            }
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct TokenPair {
                access_token: String,
                expires_in: i64,
            }
            let auth: AuthResponse = serde_json::from_str(json).unwrap();
            assert!(auth.user.is_email_verified);
            assert!(!auth.tokens.access_token.is_empty());
            assert_eq!(auth.tokens.expires_in, 1209600);
        }
    }

    mod materials_tests {
        use serde::Deserialize;

        #[test]
        fn test_materials_list_response_contract() {
            let json = r#"{
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
            #[derive(Deserialize)]
            struct PaginatedResponse {
                items: Vec<Material>,
            }
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Material {
                title: String,
                r#type: String,
            }
            let resp: PaginatedResponse = serde_json::from_str(json).unwrap();
            assert_eq!(resp.items.len(), 1);
            assert_eq!(resp.items[0].title, "Test Material");
            assert_eq!(resp.items[0].r#type, "demo");
        }

        #[test]
        fn test_material_details_response() {
            let json = r#"{
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "title": "Test",
                "files": [
                    {"id":"f1","name":"test.pdf","sizeBytes":1024,"extension":"pdf"}
                ]
            }"#;
            #[derive(Deserialize)]
            struct Details {
                files: Vec<File>,
            }
            #[derive(Deserialize)]
            struct File {
                name: String,
            }
            let details: Details = serde_json::from_str(json).unwrap();
            assert_eq!(details.files[0].name, "test.pdf");
        }
    }

    mod submissions_tests {
        use serde::Deserialize;

        #[test]
        fn test_submission_response_contract() {
            let json = r#"{
                "id": "550e8400-e29b-41d4-a716-446655440000",
                "status": "pending_review",
                "moderatorComment": "",
                "submittedAt": "2026-03-18T10:00:00Z",
                "publishedAt": null
            }"#;
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Submission {
                status: String,
                moderator_comment: String,
                submitted_at: Option<String>,
            }
            let sub: Submission = serde_json::from_str(json).unwrap();
            assert_eq!(sub.status, "pending_review");
            assert!(sub.submitted_at.is_some());
        }

        #[test]
        fn test_create_submission_request() {
            let json = r#"{
                "title": "My Submission",
                "description": "Description",
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
            assert_eq!(req.courses, vec!["1"]);
        }

        #[test]
        fn test_moderation_decision_approve() {
            let json = r#"{"action":"approve"}"#;
            #[derive(Deserialize)]
            struct Decision {
                action: String,
            }
            let d: Decision = serde_json::from_str(json).unwrap();
            assert_eq!(d.action, "approve");
        }

        #[test]
        fn test_moderation_decision_reject() {
            let json = r#"{"action":"reject","moderatorComment":"Needs correction"}"#;
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Decision {
                action: String,
                moderator_comment: String,
            }
            let d: Decision = serde_json::from_str(json).unwrap();
            assert_eq!(d.action, "reject");
            assert_eq!(d.moderator_comment, "Needs correction");
        }

        #[test]
        fn test_status_query_params() {
            let statuses = vec!["draft", "pending_review", "rejected", "approved"];
            for status in statuses {
                let json = format!(r#"{{"status":"{}"}}"#, status);
                #[derive(Deserialize)]
                struct StatusQuery {
                    status: String,
                }
                let _: StatusQuery = serde_json::from_str(&json).unwrap();
            }
        }
    }
}
