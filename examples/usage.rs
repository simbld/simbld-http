use simbld_http::helpers::{response_helpers, to_u16_helper::ToU16};
use simbld_http::helpers::{response_with_cookie_helper::*, response_with_headers_helper::*};
use simbld_http::responses::*;
use std::collections::HashMap;
use strum::EnumProperty;

fn main() {
  // Example 1: Using standard HTTP codes
  let response = ResponsesInformationalCodes::Continue;
  println!("{} - {}", response as u16, response.get_str("Description").unwrap_or("No description"));

  // Example 2: Using ResponsesTypes for success codes
  let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
  println!("Code: {}, Description: {}", response.to_u16(), response.description());

  // Example 3: Transforming a response to JSON
  let response = ResponsesInformationalCodes::EarlyHints;
  let custom_response =
    response_helpers::transform_to_json(ResponsesTypes::Informational(response));
  println!("{}", custom_response);

  // Example 4: Searching for a description by code
  if let Some(description) = response_helpers::get_description_by_code(100) {
    println!("Found description: {}", description);
  } else {
    println!("Code not found");
  }

  // Example 5: Searching for a response type by code
  if let Some(response) = response_helpers::get_response_by_code(200) {
    let (code, description) = response_helpers::get_response_description(response);
    println!("Code: {}, Description: {}", code, description);
  } else {
    println!("Code not found");
  }

  // Example 6: Using Helpers with Cookies
  let cookie = ("session_id", "abc123"); // key-value for the cookie
  let ok_response = ok_with_cookie(cookie);
  println!("{}", ok_response);

  let error_cookie = ("error_id", "xyz789");
  let error_response = bad_request_with_cookie(error_cookie);
  println!("{}", error_response);

  // Example 7: Using Helpers with Headers
  let mut headers = HashMap::new();
  headers.insert("x-trace-id", "123456");
  headers.insert("x-correlation-id", "abc-def");

  let ok_response = ok_with_headers(headers.clone());
  println!("{}", ok_response);

  let error_response = bad_request_with_headers(headers);
  println!("{}", error_response);
}
