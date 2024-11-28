pub use client::ResponsesClientCodes;
pub use crawler::ResponsesCrawlerCodes;
pub use informational::ResponsesInformationalCodes;
pub use local::ResponsesLocalApiCodes;
pub use redirection::ResponsesRedirectionCodes;
pub use server::ResponsesServerCodes;
pub use service::ResponsesServiceCodes;
pub use success::ResponsesSuccessCodes;

mod client;
mod crawler;
mod informational;
mod local;
mod redirection;
mod server;
mod service;
mod success;

generate_response_functions!(ResponsesInformationalCodes, informational);
generate_response_functions!(ResponsesSuccessCodes, success);
generate_response_functions!(ResponsesRedirectionCodes, redirection);
generate_response_functions!(ResponsesClientCodes, client);
generate_response_functions!(ResponsesServerCodes, server);
generate_response_functions!(ResponsesServiceCodes, service);
generate_response_functions!(ResponsesCrawlerCodes, crawler);
generate_response_functions!(ResponsesLocalApiCodes, local);

use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use strum::EnumProperty;
use strum_macros::EnumProperty;

/// Generate simple functions (snake_case)
generate_http_response_functions!(
  ResponsesInformationalCodes,
  ResponsesSuccessCodes,
  ResponsesRedirectionCodes,
  ResponsesClientCodes,
  ResponsesServerCodes,
  ResponsesServiceCodes,
  ResponsesCrawlerCodes,
  ResponsesLocalApiCodes
);

/// Generate functions with metadata (snake_case)
generate_http_response_with_metadata!(
  ResponsesInformationalCodes,
  ResponsesSuccessCodes,
  ResponsesRedirectionCodes,
  ResponsesClientCodes,
  ResponsesServerCodes,
  ResponsesServiceCodes,
  ResponsesCrawlerCodes,
  ResponsesLocalApiCodes
);

/// Enum representing all HTTP response families.
#[derive(Debug, Clone, Copy, EnumProperty)]
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
  /// Converts a `u16` to a `ResponsesTypes` corresponding to one of the families.
  pub fn from_u16(code: u16) -> Option<Self> {
    if let Some(info_code) = ResponsesInformationalCodes::from_u16(code) {
      return Some(ResponsesTypes::Informational(info_code));
    }
    if let Some(success_code) = ResponsesSuccessCodes::from_u16(code) {
      return Some(ResponsesTypes::Success(success_code));
    }
    if let Some(redirect_code) = ResponsesRedirectionCodes::from_u16(code) {
      return Some(ResponsesTypes::Redirection(redirect_code));
    }
    if let Some(client_error) = ResponsesClientCodes::from_u16(code) {
      return Some(ResponsesTypes::ClientError(client_error));
    }
    if let Some(server_error) = ResponsesServerCodes::from_u16(code) {
      return Some(ResponsesTypes::ServerError(server_error));
    }
    if let Some(service_error) = ResponsesServiceCodes::from_u16(code) {
      return Some(ResponsesTypes::ServiceError(service_error));
    }
    if let Some(crawler_error) = ResponsesCrawlerCodes::from_u16(code) {
      return Some(ResponsesTypes::CrawlerError(crawler_error));
    }
    if let Some(local_api_error) = ResponsesLocalApiCodes::from_u16(code) {
      return Some(ResponsesTypes::LocalApiError(local_api_error));
    }
    None
  }

  /// Returns the description associated with a response code.
  pub fn description(&self) -> &'static str {
    match self {
      ResponsesTypes::Informational(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
      ResponsesTypes::Success(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
      ResponsesTypes::Redirection(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
      ResponsesTypes::ClientError(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
      ResponsesTypes::ServerError(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
      ResponsesTypes::ServiceError(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
      ResponsesTypes::CrawlerError(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
      ResponsesTypes::LocalApiError(code_enum) => {
        code_enum.get_str("Description").unwrap_or("No description")
      },
    }
  }
}

/// Implementation to convert a `ResponsesTypes` to `u16`.
impl ToU16 for ResponsesTypes {
  fn to_u16(self) -> u16 {
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
}
