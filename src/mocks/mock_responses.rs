//! # Mock HTTP Responses
//!
//! This module provides simplified HTTP response types for testing and mocking.
//!
//! `MockResponses` is a lightweight enum that represents common HTTP response codes
//! without the full complexity of the production response types. It's designed for use
//! in tests, examples, and mocked services where the full response functionality
//! isn't necessary.
//!
//! ## Example
//!
//! ```rust
//! use simbld_http::MockResponses;
//!
//! // Convert enum to HTTP status code
//! let status_code: u16 = MockResponses::NotFound.into();
//! assert_eq!(status_code, 404);
//!
//! // Display as string
//! assert_eq!(MockResponses::Ok.to_string(), "Ok");
//! ```

use std::fmt;

/// Represents common HTTP response types for testing and mocking.
///
/// This enum provides a simplified subset of HTTP status codes that are commonly
/// used in tests and examples. Each variant maps to a standard HTTP status code.
///
/// Unlike the full `ResponsesTypes` enum, this is a lightweight alternative
/// that doesn't include descriptions, internal codes, or serialization logic.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MockResponses {
    Ok,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

/// Enables conversion from MockResponses to the numeric HTTP status code.
///
/// This allows using the enum variant anywhere a u16 status code is expected.
impl From<MockResponses> for u16 {
    fn from(val: MockResponses) -> Self {
        match val {
            MockResponses::Ok => 200,
            MockResponses::BadRequest => 400,
            MockResponses::Unauthorized => 401,
            MockResponses::NotFound => 404,
            MockResponses::InternalServerError => 500,
        }
    }
}

/// Implements string representation for MockResponses.
///
/// This allows the enum to be easily displayed or logged.
impl fmt::Display for MockResponses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let response_name = match self {
            MockResponses::Ok => "Ok",
            MockResponses::BadRequest => "BadRequest",
            MockResponses::Unauthorized => "Unauthorized",
            MockResponses::NotFound => "NotFound",
            MockResponses::InternalServerError => "InternalServerError",
        };
        write!(f, "{}", response_name)
    }
}
