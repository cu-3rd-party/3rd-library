#[cfg(test)]
mod tests {
    use crate::http::materials::models::*;
    use serde_json;

    #[test]
    fn test_material_serialization() {
        let material = Material {
            id: uuid::Uuid::new_v4(),
            author_id: uuid::Uuid::new_v4(),
            author_name: Some("Author".to_string()),
            title: "Test Material".to_string(),
            description: "Description".to_string(),
            courses: vec!["1".to_string()],
            subjects: vec!["Матан".to_string()],
            r#type: "demo".to_string(),
            difficulty: "none".to_string(),
            pub_date: Some("18.03.2026".to_string()),
        };
        let json = serde_json::to_string(&material).unwrap();
        assert!(json.contains("Test Material"));
        assert!(json.contains("demo"));
    }

    #[test]
    fn test_material_deserialization() {
        let json = r#"{
            "id":"550e8400-e29b-41d4-a716-446655440000",
            "authorId":"660e8400-e29b-41d4-a716-446655440001",
            "authorName":"Author",
            "title":"Test",
            "description":"Desc",
            "courses":["1"],
            "subjects":["Матан"],
            "type":"demo",
            "difficulty":"none",
            "pubDate":"18.03.2026"
        }"#;
        let material: Material = serde_json::from_str(json).unwrap();
        assert_eq!(material.title, "Test");
        assert_eq!(material.r#type, "demo");
    }

    #[test]
    fn test_material_file_serialization() {
        let file = MaterialFile {
            id: uuid::Uuid::new_v4(),
            name: "test.pdf".to_string(),
            size_bytes: 1024,
            extension: "pdf".to_string(),
            mime_type: Some("application/pdf".to_string()),
            url: Some("https://storage.example.com/file.pdf".to_string()),
        };
        let json = serde_json::to_string(&file).unwrap();
        assert!(json.contains("test.pdf"));
        assert!(json.contains("1024"));
    }

    #[test]
    fn test_material_details_serialization() {
        let details = MaterialDetails {
            material: Material {
                id: uuid::Uuid::new_v4(),
                author_id: uuid::Uuid::new_v4(),
                author_name: Some("Author".to_string()),
                title: "Test".to_string(),
                description: "Desc".to_string(),
                courses: vec!["1".to_string()],
                subjects: vec!["Матан".to_string()],
                r#type: "demo".to_string(),
                difficulty: "none".to_string(),
                pub_date: Some("18.03.2026".to_string()),
            },
            files: vec![],
            published_at: Some("2026-03-18T00:00:00Z".to_string()),
            submitted_at: Some("2026-03-17T00:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&details).unwrap();
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_paginated_materials_response_serialization() {
        let response = PaginatedMaterialsResponse {
            items: vec![],
            page: 1,
            limit: 20,
            total: 0,
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("page"));
        assert!(json.contains("total"));
    }

    #[test]
    fn test_create_submission_request_deserialization() {
        let json = r#"{
            "title":"My Submission",
            "description":"Some description",
            "courses":["1"],
            "subjects":["Матан"],
            "type":"demo",
            "difficulty":"none"
        }"#;
        let req: CreateSubmissionRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.title, "My Submission");
        assert_eq!(req.courses, vec!["1"]);
    }

    #[test]
    fn test_list_materials_query_deserialization() {
        let json = r#"{"search":"test","page":1,"limit":10}"#;
        let req: ListMaterialsQuery = serde_json::from_str(json).unwrap();
        assert_eq!(req.search, Some("test".to_string()));
        assert_eq!(req.page, Some(1));
    }
}