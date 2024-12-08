/// Core library module exposing helpers and responses
#[macro_use]
pub mod helpers;
pub mod responses;

// HTTP Middleware and Interceptors
pub use helpers::http_interceptor_helper::HttpInterceptor;
pub use helpers::unified_middleware_helper::UnifiedMiddleware;

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
  use crate::helpers::response_functions::ResponseFunctions;
  use crate::helpers::response_helpers::ok;
  use crate::responses::ResponsesSuccessCodes;

  #[test]
  fn test_snake_case_function_generation() {
    ResponsesSuccessCodes::generate_responses();
    ResponsesSuccessCodes::generate_responses_with_metadata();

    let response = ok();
    assert_eq!(response, (200, "Ok"));
  }
}
