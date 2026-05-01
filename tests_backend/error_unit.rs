#[cfg(test)]
mod tests {
    use crate::http::error::Error;
    use serde_json;

    #[test]
    fn test_error_unauthorized_message() {
        let err = Error::Unauthorized;
        assert_eq!(err.to_string(), "authentication required");
    }

    #[test]
    fn test_error_forbidden_message() {
        let err = Error::Forbidden;
        assert_eq!(err.to_string(), "user may not perform that action");
    }

    #[test]
    fn test_error_not_found_message() {
        let err = Error::NotFound;
        assert_eq!(err.to_string(), "request path not found");
    }

    #[test]
    fn test_error_bad_request() {
        let err = Error::BadRequest;
        assert_eq!(err.to_string(), "bad request");
    }

    #[test]
    fn test_error_conflict() {
        let err = Error::Conflict("email exists".to_string());
        assert_eq!(err.to_string(), "email exists");
    }

    #[test]
    fn test_api_error_serialization() {
        let err = Error::api_error("test_code", "test message");
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("test_code"));
        assert!(json.contains("test message"));
    }

    #[test]
    fn test_unprocessable_entity_serialization() {
        let err = Error::unprocessable_entity([
            ("field1", "error1"),
            ("field2", "error2"),
        ]);
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("field1"));
        assert!(json.contains("error1"));
    }

    #[test]
    fn test_error_conversion_to_string() {
        let err = Error::Unauthorized;
        let s = err.to_string();
        assert!(s.contains("authentication required"));
    }
}