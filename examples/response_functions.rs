use simbld_http::helpers::response_macros::*;
use simbld_http::responses::*;

/// Example of dynamic response generation
fn main() {
  // Generate functions for all HTTP status codes
  ResponsesInformationalCodes::generate_responses();
  ResponsesSuccessCodes::generate_responses();
  ResponsesRedirectionCodes::generate_responses();
  ResponsesClientCodes::generate_responses();
  ResponsesServerCodes::generate_responses();
  ResponsesServiceCodes::generate_responses();
  ResponsesCrawlerCodes::generate_responses();
  ResponsesLocalApiCodes::generate_responses();
}
