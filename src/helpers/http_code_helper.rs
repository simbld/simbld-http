/// This file defines the HttpCode struct which represents detailed HTTP code metadata.
/// The as_unified_tuple method returns a UnifiedTuple struct with optional internal fields.
/// If standard_code equals internal_code, the optional fields are None; otherwise they are Some(...).
/// Example: HttpCode::new(202, "Accepted", "Success", 202, "Accepted") returns a UnifiedTuple with None for internal_code and internal_name.
use serde::Serialize;


#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
pub struct HttpCode {
    /// Standard HTTP status code.
    pub standard_code: u16,
    /// Standard HTTP status message.
    pub standard_name: &'static str,
    /// Unified description of the HTTP response.
    pub unified_description: &'static str,
    /// Optional internal HTTP status code.
    pub internal_code: Option<u16>,
    /// Optional internal HTTP status name.
    pub internal_name: Option<&'static str>,
}

impl HttpCode {
    pub fn to_u16(&self) -> u16 {
        self.standard_code
    }
}

impl HttpCode {
    /// Creates a new HttpCode instance.
    /// If `standard_code` equals `internal_code`, then internal fields are set to None.
    /// Example:
    /// let code = HttpCode::new(200, "OK", "Successful request", 200, "OK");
    pub fn new(
        standard_code: u16,
        standard_name: &'static str,
        unified_description: &'static str,
        internal_code: u16,
        internal_name: &'static str,
    ) -> Self {
        let (int_code, int_name) = if standard_code == internal_code {
            (None, None)
        } else {
            (Some(internal_code), Some(internal_name))
        };
        HttpCode {
            standard_code,
            standard_name,
            unified_description,
            internal_code: int_code,
            internal_name: int_name,
        }
    }

    /// Returns a unified tuple representation of the HttpCode.
    /// Example: let tuple = code.as_unified_tuple();
    pub fn as_unified_tuple(&self) -> crate::helpers::unified_tuple_helper::UnifiedTuple {
        crate::helpers::unified_tuple_helper::UnifiedTuple {
            standard_code: self.standard_code,
            standard_name: self.standard_name,
            unified_description: self.unified_description,
            internal_code: self.internal_code,
            internal_name: self.internal_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use serde_json::json;

    #[test]
    fn test_http_code_new() {
        let http_code = HttpCode::new(200, "ContentDeleted", "File deleted", 215, "Accepted");
        assert_eq!(http_code.standard_code, 200);
        assert_eq!(http_code.standard_name, "ContentDeleted");
        assert_eq!(http_code.unified_description, "File deleted");
        assert_eq!(http_code.internal_code, Some(215));
        assert_eq!(http_code.internal_name, Some("Accepted"));
    }

    #[test]
    fn test_http_code_as_unified_tuple() {
        let http_code = HttpCode::new(202, "Accepted", "Request processed", 202, "Accepted");
        let unified_tuple = http_code.as_unified_tuple();
        let expected_tuple = UnifiedTuple {
            standard_code: 202,
            standard_name: "Accepted",
            unified_description: "Request processed",
            internal_code: None,
            internal_name: None,
        };
        assert_eq!(unified_tuple, expected_tuple);
    }

    #[test]
    fn test_http_code_as_unified_tuple_with_internal() {
        let http_code = HttpCode::new(
            202,
            "Accepted",
            "Request processed",
            203,
            "Non-Authoritative Information",
        );
        let unified_tuple = http_code.as_unified_tuple();
        let expected_tuple = UnifiedTuple {
            standard_code: 202,
            standard_name: "Accepted",
            unified_description: "Request processed",
            internal_code: Some(203),
            internal_name: Some("Non-Authoritative Information"),
        };
        assert_eq!(unified_tuple, expected_tuple);
    }

    #[test]
    fn test_http_code_to_u16() {
        let http_code = HttpCode::new(202, "Accepted", "Request processed", 202, "Accepted");
        assert_eq!(http_code.to_u16(), 202);
    }

    #[test]
    fn test_http_code_as_json() {
        let http_code = HttpCode::new(202, "Accepted", "Request processed", 202, "Accepted");
        let unified_tuple = http_code.as_unified_tuple();
        let json_result = serde_json::to_value(&unified_tuple).unwrap();
        let expected_json = json!({
            "standard_code": 202,
            "standard_name": "Accepted",
            "unified_description": "Request processed",
            "internal_code": null,
            "internal_name": null
        });
        assert_eq!(json_result, expected_json);
    }
}
