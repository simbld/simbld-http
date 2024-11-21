pub mod client;
pub mod crawler;
pub mod informational;
pub mod local;
pub mod redirection;
pub mod server;
pub mod service;
pub mod success;

pub use client::ResponsesClientCodes;
pub use crawler::ResponsesCrawlerCodes;
pub use informational::ResponsesInformationalCodes;
pub use local::ResponsesLocalApiCodes;
pub use redirection::ResponsesRedirectionCodes;
pub use server::ResponsesServerCodes;
pub use service::ResponsesServiceCodes;
pub use success::ResponsesSuccessCodes;

use strum::EnumProperty;

#[derive(Debug)]
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
      ResponsesTypes::Informational(code_enum) => (*code_enum).into(),
      ResponsesTypes::Success(code_enum) => (*code_enum).into(),
      ResponsesTypes::Redirection(code_enum) => (*code_enum).into(),
      ResponsesTypes::ClientError(code_enum) => (*code_enum).into(),
      ResponsesTypes::ServerError(code_enum) => (*code_enum).into(),
      ResponsesTypes::ServiceError(code_enum) => (*code_enum).into(),
      ResponsesTypes::CrawlerError(code_enum) => (*code_enum).into(),
      ResponsesTypes::LocalApiError(code_enum) => (*code_enum).into(),
    }
  }

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
