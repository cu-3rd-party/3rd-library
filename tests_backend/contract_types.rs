use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Role {
    pub role: String,
}

#[test]
fn test_role_serialization() {
    let role = Role {
        role: "user".to_string(),
    };
    let json = serde_json::to_string(&role).unwrap();
    assert!(json.contains("user"));
}

#[test]
fn test_role_deserialization() {
    let json = r#"{"role":"moderator"}"#;
    let role: Role = serde_json::from_str(json).unwrap();
    assert_eq!(role.role, "moderator");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Course(pub String);

#[test]
fn test_course_enum() {
    let courses = vec![Course("1".to_string()), Course("2".to_string())];
    let json = serde_json::to_string(&courses).unwrap();
    assert!(json.contains("1"));
    assert!(json.contains("2"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject(pub String);

#[test]
fn test_subject_enum() {
    let subjects = vec![
        Subject("Матан".to_string()),
        Subject("Английский".to_string()),
    ];
    let json = serde_json::to_string(&subjects).unwrap();
    assert!(json.contains("Матан"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialType(pub String);

#[test]
fn test_material_type_enum() {
    let types = vec![
        MaterialType("demo".to_string()),
        MaterialType("longread".to_string()),
        MaterialType("solution".to_string()),
        MaterialType("cheatlist".to_string()),
        MaterialType("shortread".to_string()),
        MaterialType("other".to_string()),
    ];
    let json = serde_json::to_string(&types).unwrap();
    assert!(json.contains("demo"));
    assert!(json.contains("solution"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Difficulty(pub String);

#[test]
fn test_difficulty_enum() {
    let difficulties = vec![
        Difficulty("none".to_string()),
        Difficulty("blue".to_string()),
        Difficulty("red".to_string()),
        Difficulty("black".to_string()),
    ];
    let json = serde_json::to_string(&difficulties).unwrap();
    assert!(json.contains("none"));
    assert!(json.contains("blue"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionStatus(pub String);

#[test]
fn test_submission_status_enum() {
    let statuses = vec![
        SubmissionStatus("draft".to_string()),
        SubmissionStatus("pending_review".to_string()),
        SubmissionStatus("rejected".to_string()),
        SubmissionStatus("approved".to_string()),
    ];
    let json = serde_json::to_string(&statuses).unwrap();
    assert!(json.contains("draft"));
    assert!(json.contains("pending_review"));
    assert!(json.contains("approved"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModerationStatusFilter(pub String);

#[test]
fn test_moderation_status_filter() {
    let filters = vec![
        ModerationStatusFilter("pending_review".to_string()),
        ModerationStatusFilter("rejected".to_string()),
        ModerationStatusFilter("approved".to_string()),
        ModerationStatusFilter("all".to_string()),
    ];
    let json = serde_json::to_string(&filters).unwrap();
    assert!(json.contains("all"));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<FieldError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[test]
fn test_api_error_serialization() {
    let err = ApiError {
        code: "validation_error".to_string(),
        message: "Invalid request payload".to_string(),
        details: None,
    };
    let json = serde_json::to_string(&err).unwrap();
    assert!(json.contains("validation_error"));
    assert!(json.contains("Invalid request payload"));
}

#[test]
fn test_api_error_with_details() {
    let err = ApiError {
        code: "validation_error".to_string(),
        message: "Invalid request payload".to_string(),
        details: Some(vec![
            FieldError {
                field: "email".to_string(),
                message: "Invalid email format".to_string(),
            },
        ]),
    };
    let json = serde_json::to_string(&err).unwrap();
    assert!(json.contains("email"));
    assert!(json.contains("Invalid email format"));
}

#[test]
fn test_api_error_deserialization() {
    let json = r#"{
        "code": "not_found",
        "message": "Resource not found",
        "details": null
    }"#;
    let err: ApiError = serde_json::from_str(json).unwrap();
    assert_eq!(err.code, "not_found");
    assert_eq!(err.message, "Resource not found");
}