#[macro_use]

pub mod helpers;
pub mod mocks;
pub mod responses;
pub mod utils;

pub use helpers::auth_middleware::AuthMiddleware;
pub use helpers::http_interceptor_helper::HttpInterceptor;
pub use helpers::unified_middleware_helper::UnifiedMiddleware;
pub use mocks::mock_responses::MockResponses;
pub use responses::wrapper::ResponseWrapper;

pub use helpers::generate_responses_functions;

pub use inflector::Inflector;
pub use strum::IntoEnumIterator;
pub use strum_macros::EnumIter;

pub use crate::responses::ResponsesClientCodes;
pub use crate::responses::ResponsesCrawlerCodes;
pub use crate::responses::ResponsesInformationalCodes;
pub use crate::responses::ResponsesLocalApiCodes;
pub use crate::responses::ResponsesRedirectionCodes;
pub use crate::responses::ResponsesServerCodes;
pub use crate::responses::ResponsesServiceCodes;
pub use crate::responses::ResponsesSuccessCodes;

#[cfg(test)]
mod tests {
  use crate::responses::ResponsesCrawlerCodes;
  use crate::responses::UnifiedTuple;

  #[test]
  fn test_crawler_codes_to_u16() {
    let code = ResponsesCrawlerCodes::ParsingErrorHeader;
    assert_eq!(code.to_u16(), 400);
  }

  #[test]
  fn test_crawler_codes_from_u16() {
    let status = ResponsesCrawlerCodes::from_u16(400);
    assert_eq!(status, Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader));
  }

  #[test]
  fn test_crawler_codes_as_tuple() {
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
  fn test_crawler_codes_as_json() {
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
}
