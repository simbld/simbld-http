//! # Unified HTTP Response Representation
//!
//! This module provides the `UnifiedTuple` structure which offers a standardized
//! way to represent HTTP responses across the application.
//!
//! The `UnifiedTuple` combines standard HTTP status information with optional
//! application-specific internal codes and names, enabling consistent handling
//! of both standard HTTP responses and custom application responses.
//!
//! ## Example
//!
//! ```rust
//! use simbld_http::helpers::unified_tuple_helper::UnifiedTuple;
//!
//! let tuple = UnifiedTuple {
//!     standard_code: 200,
//!     standard_name: "OK",
//!     unified_description: "Successful request",
//!     internal_code: None,
//!     internal_name: None,
//! };
//!
//! // Convert to JSON for serialization
//! let json = tuple.as_json();
//! assert_eq!(json["description"], "Successful request");
//! ```

use crate::responses::ResponsesTypes;
use serde::Serialize;

/// A standardized representation of HTTP response codes and metadata.
///
/// This structure provides a unified way to represent both standard HTTP
/// response codes and application-specific internal codes, making it easier
/// to handle responses consistently throughout the application.
///
#[derive(Debug, PartialEq, Serialize)]
pub struct UnifiedTuple {
    /// Standard HTTP code.
    pub standard_code: u16,
    /// Standard HTTP name.
    pub standard_name: &'static str,
    /// Description of the response.
    pub unified_description: &'static str,
    /// Internal HTTP code (None if equal to standard code).
    pub internal_code: Option<u16>,
    /// Internal HTTP name (None if equal to standard name).
    pub internal_name: Option<&'static str>,
}

impl UnifiedTuple {
    /// Converts the UnifiedTuple to a structured JSON representation.
    ///
    /// The resulting JSON structure includes:
    /// - Standard HTTP code information (code and name)
    /// - Description text
    /// - Internal HTTP code information if available (code and name)
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` containing the structured JSON representation
    ///
    pub fn as_json(&self) -> serde_json::Value {
        serde_json::json!({
            "standard http code": {
                "code": self.standard_code,
                "name": self.standard_name,
            },
            "description": self.unified_description,
            "internal http code": {
                "code": self.internal_code,
                "name": self.internal_name,
            },
        })
    }

    pub fn new(
        standard_code: u16,
        standard_name: &'static str,
        unified_description: &'static str,
        internal_code: u16,
        internal_name: &'static str,
    ) -> Self {
        Self {
            standard_code,
            standard_name,
            unified_description,
            internal_code: Some(internal_code),
            internal_name: Some(internal_name),
        }
    }
}

/// Implements automatic conversion from ResponsesTypes to UnifiedTuple.
///
/// This allows for seamless integration between the enum-based response types
/// and the structured tuple representation.
///
impl From<ResponsesTypes> for UnifiedTuple {
    fn from(response: ResponsesTypes) -> Self {
        let standard_code = response.get_code();
        let standard_name = response.get_description();
        let unified_description = response.get_description();
        UnifiedTuple {
            standard_code,
            standard_name,
            unified_description,
            internal_code: None,
            internal_name: None,
        }
    }
}
