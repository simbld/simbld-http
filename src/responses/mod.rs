/// This module organizes and provides enums for various HTTP response status codes and categories.
/// It includes the following categories:
/// - Informational (1xx)
/// - Success (2xx)
/// - Redirection (3xx)
/// - Client (4xx)
/// - Server (5xx)
/// - Local API and custom codes
/// - Service and Crawler-specific responses

#[macro_use]
pub mod actix_responder;
pub mod client;
pub mod crawler;
pub mod http_code;
pub mod informational;
pub mod local;
pub mod redirection;
pub mod server;
pub mod service;
pub mod success;

// Public exports for response codes
pub use actix_responder::CustomResponse;
pub use client::ResponsesClientCodes;
pub use crawler::ResponsesCrawlerCodes;
pub use http_code::HttpCode;
pub use informational::ResponsesInformationalCodes;
pub use local::ResponsesLocalApiCodes;
pub use redirection::ResponsesRedirectionCodes;
pub use server::ResponsesServerCodes;
pub use service::ResponsesServiceCodes;
pub use success::ResponsesSuccessCodes;

use strum::IntoEnumIterator;

/// Enum representing the main categories of HTTP response codes.
/// Combines multiple categories into a unified type for simplified handling.

#[derive(Debug, PartialEq)]
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
      ResponsesTypes::Informational(code_enum) => code_enum.to_u16(),
      ResponsesTypes::Success(code_enum) => code_enum.to_u16(),
      ResponsesTypes::Redirection(code_enum) => code_enum.to_u16(),
      ResponsesTypes::ClientError(code_enum) => code_enum.to_u16(),
      ResponsesTypes::ServerError(code_enum) => code_enum.to_u16(),
      ResponsesTypes::ServiceError(code_enum) => code_enum.to_u16(),
      ResponsesTypes::CrawlerError(code_enum) => code_enum.to_u16(),
      ResponsesTypes::LocalApiError(code_enum) => code_enum.to_u16(),
    }
  }

  /// Converts the enum variant into a JSON representation.
  pub fn as_json(&self) -> serde_json::Value {
    match self {
      ResponsesTypes::Informational(code_enum) => code_enum.as_json(),
      ResponsesTypes::Success(code_enum) => code_enum.as_json(),
      ResponsesTypes::Redirection(code_enum) => code_enum.as_json(),
      ResponsesTypes::ClientError(code_enum) => code_enum.as_json(),
      ResponsesTypes::ServerError(code_enum) => code_enum.as_json(),
      ResponsesTypes::ServiceError(code_enum) => code_enum.as_json(),
      ResponsesTypes::CrawlerError(code_enum) => code_enum.as_json(),
      ResponsesTypes::LocalApiError(code_enum) => code_enum.as_json(),
    }
  }

  /// Converts the enum variant into a tuple representation.
  pub fn as_tuple(&self) -> UnifiedTuple {
    match self {
      ResponsesTypes::Informational(code_enum) => code_enum.as_tuple(),
      ResponsesTypes::Success(code_enum) => code_enum.as_tuple(),
      ResponsesTypes::Redirection(code_enum) => code_enum.as_tuple(),
      ResponsesTypes::ClientError(code_enum) => code_enum.as_tuple(),
      ResponsesTypes::ServerError(code_enum) => code_enum.as_tuple(),
      ResponsesTypes::ServiceError(code_enum) => code_enum.as_tuple(),
      ResponsesTypes::CrawlerError(code_enum) => code_enum.as_tuple(),
      ResponsesTypes::LocalApiError(code_enum) => code_enum.as_tuple(),
    }
  }

  /// Attempts to construct a `ResponsesTypes` variant from a given `u16` code.
  pub fn from_u16(code: u16) -> Option<Self> {
    ResponsesInformationalCodes::from_u16(code)
      .map(ResponsesTypes::Informational)
      .or_else(|| ResponsesSuccessCodes::from_u16(code).map(ResponsesTypes::Success))
      .or_else(|| ResponsesRedirectionCodes::from_u16(code).map(ResponsesTypes::Redirection))
      .or_else(|| ResponsesClientCodes::from_u16(code).map(ResponsesTypes::ClientError))
      .or_else(|| ResponsesServerCodes::from_u16(code).map(ResponsesTypes::ServerError))
      .or_else(|| ResponsesServiceCodes::from_u16(code).map(ResponsesTypes::ServiceError))
      .or_else(|| ResponsesCrawlerCodes::from_u16(code).map(ResponsesTypes::CrawlerError))
      .or_else(|| ResponsesLocalApiCodes::from_u16(code).map(ResponsesTypes::LocalApiError))
  }
}

/// Represents a unified structure with five fields for response metadata.
#[derive(Debug, PartialEq)]
pub enum UnifiedTuple {
  FiveFields(u16, &'static str, &'static str, u16, &'static str),
}

impl UnifiedTuple {
  /// Extracts the description field from the tuple.
  pub fn get_description(&self) -> &'static str {
    match self {
      UnifiedTuple::FiveFields(_, _, desc, _, _) => desc,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::ResponsesCrawlerCodes;

  #[test]
  fn test_to_u16() {
    let code = ResponsesCrawlerCodes::ParsingErrorHeader;
    assert_eq!(code.to_u16(), 400);
  }

  #[test]
  fn test_from_u16() {
    let status = ResponsesCrawlerCodes::from_u16(400);
    assert_eq!(status, Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader));
  }

  #[test]
  fn test_as_tuple() {
    let code = ResponsesCrawlerCodes::InvalidURL;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::FiveFields(
        400,
        "Bad Request",
        "Invalid URL encountered by crawler.",
        786,
        "Invalid URL"
      )
    );
  }

  #[test]
  fn test_as_json() {
    let code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
    let json_result = code.as_json();
    let expected = json!({
        "standard http code": {
            "code": 503,
            "name": "Service Unavailable"
        },
        "internal http code": {
            "code": 741,
            "name": "Robots Temporarily Unavailable"
        },
        "description": "Robots temporarily unavailable."
    });
    assert_eq!(json_result, expected);
  }

  #[test]
  fn test_into_tuple() {
    let code = ResponsesCrawlerCodes::ProgrammableRedirection;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 302);
    assert_eq!(std_name, "Found");
  }

  #[test]
  fn test_from_u16_unknown_code() {
    let result = ResponsesTypes::from_u16(9999);
    assert!(result.is_none());
  }
}
