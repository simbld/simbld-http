use simbld_http::define_response_code;
use simbld_http::responses::{ResponsesInformationalCodes, ResponsesTypes};

#[test]
fn test_informational_codes() {
  let response = ResponsesInformationalCodes::Continue;
  assert_eq!(response.to_u16(), 100);
  assert_eq!(response.description(), "Waiting for the continuation of the request");
}

#[test]
fn test_custom_code_macro() {
  define_response_code!(CustomCode, 999, "Custom HTTP response code");
  assert_eq!(CustomCode::code(), 999);
  assert_eq!(CustomCode::description(), "Custom HTTP response code");
}

#[test]
fn test_response_types_mapping() {
  let response = ResponsesTypes::Informational(ResponsesInformationalCodes::Continue);
  assert_eq!(response.to_u16(), 100);
  assert_eq!(response.description(), "Waiting for the continuation of the request");
}
