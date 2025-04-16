//! # HTTP Status Code Representation
//!
//! This module provides a standardized way to represent and work with HTTP status codes.
//! It defines the `HttpCode` struct which encapsulates both standard HTTP status codes
//! and custom internal codes used for application-specific error tracking.
//!
//! The module supports conversion between different representations of HTTP codes and
//! provides utilities for serializing them in a consistent format.

use crate::traits::into_http_code_trait::IntoHttpCode;
use serde::Serialize;

/// Represents an HTTP status code with standard and internal identifiers.
///
/// This structure combines standard HTTP status codes (like 200, 404) with optional
/// internal application-specific codes for more detailed error tracking and reporting.
///
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

/// Implement IntoHttpCode for u16, converting it to an HttpCode with default values.
impl IntoHttpCode for (u16, &'static str, &'static str) {
    /// Converts a tuple of (code, name, description) into an HttpCode.
    fn into_http_code(self) -> HttpCode {
        let (standard_code, standard_name, unified_description) = self;
        HttpCode {
            standard_code,
            standard_name,
            unified_description,
            internal_code: None,
            internal_name: None,
        }
    }
}

impl HttpCode {
    /// Creates a new HttpCode with both standard and internal identifiers.
    ///
    /// # Arguments
    /// * `standard_code` - The standard HTTP status code
    /// * `standard_name` - The standard HTTP status name
    /// * `unified_description` - Human-readable description
    /// * `internal_code` - Application-specific status code
    /// * `internal_name` - Application-specific status name
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

    /// Returns the standard HTTP status code.
    pub fn get_code(&self) -> u16 {
        self.standard_code
    }

    /// Converts the HttpCode to a UnifiedTuple representation for consistent formatting.
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

    /// Tests the creation of an HttpCode with the new() constructor.
    #[test]
    fn test_http_code_new() {
        let http_code = HttpCode::new(200, "ContentDeleted", "File deleted", 215, "Accepted");
        assert_eq!(http_code.standard_code, 200);
        assert_eq!(http_code.standard_name, "ContentDeleted");
        assert_eq!(http_code.unified_description, "File deleted");
        assert_eq!(http_code.internal_code, Some(215));
        assert_eq!(http_code.internal_name, Some("Accepted"));
    }

    /// Tests conversion to UnifiedTuple with only standard fields.
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

    /// Tests conversion to UnifiedTuple with both standard and internal fields.
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

    /// Tests the get_code method returns the correct standard code.
    #[test]
    fn test_http_code_get_code() {
        let http_code = HttpCode::new(202, "Accepted", "Request processed", 202, "Accepted");
        assert_eq!(http_code.get_code(), 202);
    }

    /// Tests JSON serialization of HttpCode instances.
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
