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
  pub fn as_tuple(&self) -> UnifiedTuple {
    match self {
      ResponsesTypes::Informational(code) => code.as_tuple(),
      ResponsesTypes::Success(code) => code.as_tuple(),
      ResponsesTypes::Redirection(code) => code.as_tuple(),
      ResponsesTypes::ClientError(code) => code.as_tuple(),
      ResponsesTypes::ServerError(code) => code.as_tuple(),
      ResponsesTypes::ServiceError(code) => code.as_tuple(),
      ResponsesTypes::CrawlerError(code) => code.as_tuple(),
      ResponsesTypes::LocalApiError(code) => code.as_tuple(),
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
}

/// Represents a unified structure with five fields for response metadata.
#[derive(Debug, PartialEq)]
pub enum UnifiedTuple {
  ThreeFields(u16, &'static str, &'static str),
  FiveFields(u16, &'static str, &'static str, u16, &'static str),
}

impl UnifiedTuple {
  pub fn get_description(&self) -> &'static str {
    match self {
      UnifiedTuple::ThreeFields(_, _, description) => description,
      UnifiedTuple::FiveFields(_, _, description, _, _) => description,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_to_u16() {
    assert_eq!(
      ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader).to_u16(),
      400
    );
  }
  
  #[test]
  fn test_as_tuple() {
    // Cas où le code interne et standard sont identiques
    assert_eq!(
      ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader).as_tuple(),
      UnifiedTuple::ThreeFields(400, "Parsing Error", "Parsing error: Unfinished header.")
    );
    
    // Cas où le code interne et standard sont différents
    assert_eq!(
      ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ExcludedByRobotsTxtFile).as_tuple(),
      UnifiedTuple::FiveFields(
        403,
        "Forbidden",
        "Access denied by Robots.txt file.",
        700,
        "Excluded by Robots.txt"
      )
    );
  }
  
  #[test]
  fn test_as_json() {
    // Cas où le code interne et standard sont identiques
    let json_value = ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader).as_json();
    let expected_json = serde_json::json!({
        "code": 400,
        "name": "Parsing Error",
        "description": "Parsing error: Unfinished header."
    });
    assert_eq!(json_value, expected_json);
    
    // Cas où le code interne et standard sont différents
    let json_value = ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ExcludedByRobotsTxtFile).as_json();
    let expected_json = serde_json::json!({
        "std_code": 403,
        "std_name": "Forbidden",
        "description": "Access denied by Robots.txt file.",
        "int_code": 700,
        "int_name": "Excluded by Robots.txt"
    });
    assert_eq!(json_value, expected_json);
  }
  
  #[test]
  fn test_from_u16() {
    assert_eq!(
      ResponsesTypes::from_u16(400),
      Some(ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader))
    );
    assert_eq!(ResponsesTypes::from_u16(999), None);
  }
}