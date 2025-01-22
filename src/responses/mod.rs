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

pub use actix_responder::CustomResponse;
pub use client::ResponsesClientCodes;
pub use crawler::ResponsesCrawlerCodes;
pub use informational::ResponsesInformationalCodes;
pub use local::ResponsesLocalApiCodes;
pub use redirection::ResponsesRedirectionCodes;
pub use server::ResponsesServerCodes;
pub use service::ResponsesServiceCodes;
pub use success::ResponsesSuccessCodes;

use serde_json::Value;

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

  pub fn as_json(&self) -> Value {
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

  pub fn as_tuple(&self) -> (u16, &'static str, &'static str) {
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

  pub fn from_u16(code: u16) -> Option<Self> {
    if let Some(enum_code) = ResponsesInformationalCodes::from_u16(code) {
      return Some(ResponsesTypes::Informational(enum_code));
    }
    if let Some(enum_code) = ResponsesSuccessCodes::from_u16(code) {
      return Some(ResponsesTypes::Success(enum_code));
    }
    if let Some(enum_code) = ResponsesRedirectionCodes::from_u16(code) {
      return Some(ResponsesTypes::Redirection(enum_code));
    }
    if let Some(enum_code) = ResponsesClientCodes::from_u16(code) {
      return Some(ResponsesTypes::ClientError(enum_code));
    }
    if let Some(enum_code) = ResponsesServerCodes::from_u16(code) {
      return Some(ResponsesTypes::ServerError(enum_code));
    }
    if let Some(enum_code) = ResponsesServiceCodes::from_u16(code) {
      return Some(ResponsesTypes::ServiceError(enum_code));
    }
    if let Some(enum_code) = ResponsesCrawlerCodes::from_u16(code) {
      return Some(ResponsesTypes::CrawlerError(enum_code));
    }
    if let Some(enum_code) = ResponsesLocalApiCodes::from_u16(code) {
      return Some(ResponsesTypes::LocalApiError(enum_code));
    }
    None
  }
}

/// Represents a unified structure with 5 fields.
#[derive(Debug, PartialEq)]
pub enum UnifiedTuple {
  FiveFields(u16, &'static str, &'static str, u16, &'static str),
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
      UnifiedTuple::FivesFields(
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
    let expected = serde_json::json!({
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
}
