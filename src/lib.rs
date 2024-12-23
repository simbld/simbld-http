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
  use super::responses::success::ok_tuple;
  #[test]
  fn test_ok() {
    let response = ok_tuple();
    assert_eq!(response, (200, "Ok"));
  }

  #[test]
  fn ok_json() {
    let response = ok_tuple();
    assert_eq!(response, (200, "Ok"));
  }
}
