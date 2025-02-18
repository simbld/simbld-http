/// This module organizes and provides enums for various HTTP response status codes and categories.
/// It includes the following categories:
/// - Informational (1xx)
/// - Success (2xx)
/// - Redirection (3xx)
/// - Client (4xx)
/// - Server (5xx)
/// - Local API codes (9xx)
/// - Service responses (6xx)
/// - Crawler-specific responses (7xx)

#[macro_use]
pub mod actix_responder;
pub mod client;
pub mod crawler;
pub mod informational;
pub mod local;
pub mod redirection;
pub mod server;
pub mod service;
pub mod success;

// Public exports for response codes
use crate::helpers::get_description_field_helper::GetDescription;
use crate::helpers::response_helpers;
pub use actix_responder::CustomResponse;
pub use client::ResponsesClientCodes;
pub use crawler::ResponsesCrawlerCodes;
pub use informational::ResponsesInformationalCodes;
pub use local::ResponsesLocalApiCodes;
pub use redirection::ResponsesRedirectionCodes;
pub use server::ResponsesServerCodes;
pub use service::ResponsesServiceCodes;
use strum_macros::EnumProperty;
pub use success::ResponsesSuccessCodes;

// Public exports for response types
use crate::helpers::http_code_helper::HttpCode;

/// Enum representing the main categories of HTTP response codes.
/// Combines multiple categories into a unified type for simplified handling.

/// Enum representing all HTTP response families.
#[derive(Debug, Clone, Copy, EnumProperty, PartialEq)]
pub enum ResponsesTypes {
    Informational(ResponsesInformationalCodes),
    Success(ResponsesSuccessCodes),
    Redirection(ResponsesRedirectionCodes),
    ClientError(ResponsesClientCodes),
    ServerError(ResponsesServerCodes),
    ServiceError(ResponsesServiceCodes),
    CrawlerError(ResponsesCrawlerCodes),
    LocalApiError(ResponsesLocalApiCodes),
}

impl ResponsesTypes {
    /// Converts the enum variant to its corresponding HTTP status code as `u16`.
    pub fn to_u16(&self) -> u16 {
        match self {
            ResponsesTypes::Informational(code) => code.to_u16(),
            ResponsesTypes::Success(code) => code.to_u16(),
            ResponsesTypes::Redirection(code) => code.to_u16(),
            ResponsesTypes::ClientError(code) => code.to_u16(),
            ResponsesTypes::ServerError(code) => code.to_u16(),
            ResponsesTypes::ServiceError(code) => code.to_u16(),
            ResponsesTypes::CrawlerError(code) => code.to_u16(),
            ResponsesTypes::LocalApiError(code) => code.to_u16(),
        }
    }

    /// Converts the enum variant into a JSON representation.
    pub fn as_json(&self) -> serde_json::Value {
        match self {
            ResponsesTypes::Informational(code) => code.as_json(),
            ResponsesTypes::Success(code) => code.as_json(),
            ResponsesTypes::Redirection(code) => code.as_json(),
            ResponsesTypes::ClientError(code) => code.as_json(),
            ResponsesTypes::ServerError(code) => code.as_json(),
            ResponsesTypes::ServiceError(code) => code.as_json(),
            ResponsesTypes::CrawlerError(code) => code.as_json(),
            ResponsesTypes::LocalApiError(code) => code.as_json(),
        }
    }

    /// Converts the enum variant into a tuple representation.
    pub fn as_tuple(&self) -> HttpCode {
        match self {
            ResponsesTypes::Informational(code) => code.to_http_code(),
            ResponsesTypes::Success(code) => code.to_http_code(),
            ResponsesTypes::Redirection(code) => code.to_http_code(),
            ResponsesTypes::ClientError(code) => code.to_http_code(),
            ResponsesTypes::ServerError(code) => code.to_http_code(),
            ResponsesTypes::ServiceError(code) => code.to_http_code(),
            ResponsesTypes::CrawlerError(code) => code.to_http_code(),
            ResponsesTypes::LocalApiError(code) => code.to_http_code(),
        }
    }

    /// Attempts to construct a `ResponsesTypes` variant from a given `u16` code.
    pub fn from_u16(code: u16) -> Option<Self> {
        if let Some(c) = ResponsesInformationalCodes::from_u16(code) {
            return Some(ResponsesTypes::Informational(c));
        }
        if let Some(c) = ResponsesSuccessCodes::from_u16(code) {
            return Some(ResponsesTypes::Success(c));
        }
        if let Some(c) = ResponsesRedirectionCodes::from_u16(code) {
            return Some(ResponsesTypes::Redirection(c));
        }
        if let Some(c) = ResponsesClientCodes::from_u16(code) {
            return Some(ResponsesTypes::ClientError(c));
        }
        if let Some(c) = ResponsesServerCodes::from_u16(code) {
            return Some(ResponsesTypes::ServerError(c));
        }
        if let Some(c) = ResponsesServiceCodes::from_u16(code) {
            return Some(ResponsesTypes::ServiceError(c));
        }
        if let Some(c) = ResponsesCrawlerCodes::from_u16(code) {
            return Some(ResponsesTypes::CrawlerError(c));
        }
        if let Some(c) = ResponsesLocalApiCodes::from_u16(code) {
            return Some(ResponsesTypes::LocalApiError(c));
        }
        None
    }

    /// Returns the description associated with a response code.
    pub fn description(&self) -> &'static str {
        match self {
            ResponsesTypes::Informational(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
            ResponsesTypes::Success(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
            ResponsesTypes::Redirection(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
            ResponsesTypes::ClientError(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
            ResponsesTypes::ServerError(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
            ResponsesTypes::ServiceError(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
            ResponsesTypes::CrawlerError(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
            ResponsesTypes::LocalApiError(code_enum) => {
                code_enum.get_description_field("Description").unwrap_or("No description")
            },
        }
    }

    pub fn get_response_description(&self) -> (u16, &'static str) {
        match self {
            ResponsesTypes::Informational(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
            ResponsesTypes::Success(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
            ResponsesTypes::Redirection(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
            ResponsesTypes::ClientError(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
            ResponsesTypes::ServerError(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
            ResponsesTypes::ServiceError(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
            ResponsesTypes::CrawlerError(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
            ResponsesTypes::LocalApiError(code) => {
                (code.to_u16(), code.get_description_field("Description").unwrap_or(""))
            },
        }
    }

    pub fn as_normalized_json(&self) -> serde_json::Value {
        if let Some(normalized) = response_helpers::get_response_by_type(self) {
            normalized.as_json()
        } else {
            self.as_json()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ResponsesCrawlerCodes::ExcludedByRobotsTxtFile;
    use serde_json::json;
    
    #[test]
    fn test_to_u16() {
        assert_eq!(
            ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader)
                .to_u16(),
            400
        );
    }

    #[test]
    fn test_as_tuple() {
        // Case where the internal and standard codes are identical
        let tuple_result = ResponsesTypes::Success(ResponsesSuccessCodes::Ok).as_tuple();
        assert_eq!(tuple_result.standard_code, 200);
        assert_eq!(tuple_result.standard_name, "OK");
        assert_eq!(tuple_result.description, "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response");
        assert_eq!(tuple_result.internal_code, 200);
        assert_eq!(tuple_result.internal_name, "OK");

        // Case where the internal and standard codes are different
        let tuple_result_diff = ResponsesTypes::CrawlerError(ExcludedByRobotsTxtFile).as_tuple();
        assert_eq!(tuple_result_diff.standard_code, 403);
        assert_eq!(tuple_result_diff.standard_name, "Forbidden");
        assert_eq!(tuple_result_diff.description, "Excluded by robots.txt file.");
        assert_eq!(tuple_result_diff.internal_code, 740);
        assert_eq!(tuple_result_diff.internal_name, "Excluded by Robots.txt file");
    }

    #[test]
    fn test_as_json() {
        let json_value = ResponsesTypes::CrawlerError(ExcludedByRobotsTxtFile).as_json();
        let expected_json = json!({
            "description": "Excluded by robots.txt file",
            "internal_http_code": {
                "code": 740,
                "name": "Excluded by Robots.txt file"
            },
            "standard_http_code": {
                "code": 403, "name": "Forbidden"
            }
        });
        assert_eq!(json_value, expected_json);
    }

    #[test]
    fn test_from_u16() {
        assert_eq!(
            ResponsesTypes::from_u16(700),
            Some(ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader))
        );
        assert_eq!(ResponsesTypes::from_u16(999), None);
    }
}

#[test]
fn test_as_normalized() {
    let client_error = ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest);

    // recovery via get_response_by_type
    let normalized = response_helpers::get_response_by_type(&client_error);

    // Verification that standardization is possible
    assert_eq!(normalized, Some(ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest)));

    // Verification of an unknown code
    let unknown_code = ResponsesTypes::from_u16(9999);
    let normalized_unknown =
        unknown_code.as_ref().and_then(|code| response_helpers::get_response_by_type(code));

    assert_eq!(normalized_unknown, None);
}

#[test]
fn test_get_advance_response_description() {
    let client_error = ResponsesTypes::ClientError(ResponsesClientCodes::SSLCertificateError);
    let (code, description) = response_helpers::get_advance_response_description(client_error);

    assert_eq!(code, 400);
    assert_eq!(description, "An invalid or untrusted SSL certificate was encountered");
}
