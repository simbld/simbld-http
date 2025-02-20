/// Provides response codes, helpers, and utility functions for HTTP response management.
#[macro_use]
pub mod helpers;
pub mod mocks;
pub mod responses;
pub mod utils;

// Public exports for helpers
pub use helpers::auth_middleware::AuthMiddleware;
pub use helpers::generate_responses_functions;
pub use helpers::http_interceptor_helper::HttpInterceptor;
pub use helpers::unified_middleware_helper::UnifiedMiddleware;

// Public exports for mocks
pub use mocks::mock_responses::MockResponses;

// External crates re-exported for convenience
pub use inflector::Inflector;
pub use serde_json::{json, Value};

// Public exports for response modules
pub use responses::ResponsesClientCodes;
pub use responses::ResponsesCrawlerCodes;
pub use responses::ResponsesInformationalCodes;
pub use responses::ResponsesLocalApiCodes;
pub use responses::ResponsesRedirectionCodes;
pub use responses::ResponsesServerCodes;
pub use responses::ResponsesServiceCodes;
pub use responses::ResponsesSuccessCodes;

// Module for tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::http_code_helper::HttpCode;
    use crate::responses::ResponsesCrawlerCodes;
    
    /// Test `to_u16` method for `ResponsesCrawlerCodes`.
    #[test]
    fn test_crawler_codes_to_u16() {
        assert_eq!(ResponsesCrawlerCodes::ParsingErrorHeader.to_u16(), 400);
    }

    /// Test `from_u16` method for `ResponsesCrawlerCodes`.
    #[test]
    fn test_crawler_codes_from_u16() {
        assert_eq!(
            ResponsesCrawlerCodes::from_u16(400),
            Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader)
        );
    }

    /// Test `as_tuple` method for `ResponsesCrawlerCodes`.
    #[test]
    fn test_crawler_codes_as_tuple() {
        let code = ResponsesCrawlerCodes::InvalidURL;
        let http_code = code.to_http_code();
        assert_eq!(
            http_code,
            HttpCode {
                standard_code: 400,
                standard_name: "Bad Request",
                unified_description: "Invalid URL encountered by crawler.",
                internal_code: 786,
                internal_name: "Invalid URL"
            }
        );
    }

    /// Test `as_json` method for `ResponsesCrawlerCodes`.
    #[test]
    fn test_crawler_codes_as_json() {
        let code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
        let json_result = code.as_json();
        let expected = json!({
            "standard http code": {
                "standard_code": 503,
                "standard_name": "Service Unavailable"
            },
            "internal http code": {
                "internal_code": 741,
                "internal_name": "Robots Temporarily Unavailable"
            },
            "unified_description": "Robots temporarily unavailable."
        });

        assert_eq!(json_result, expected);
    }
}
