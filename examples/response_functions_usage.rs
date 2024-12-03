use crate::helpers::response_functions::ResponseFunctions;
use inflector::Inflector;
use simbld_http::helpers::response_helpers;
use simbld_http::responses::*;
use strum::IntoEnumIterator;

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

fn example_response_with_metadata() {
  let response = ResponsesTypes::Client(ResponsesClientCodes::BadRequest);
  let enriched_response = response_helpers::get_enriched_response_with_metadata(
    response,
    Some("http://example.com"),
    std::time::Duration::from_millis(200),
  );
  println!("{}", enriched_response);
}
