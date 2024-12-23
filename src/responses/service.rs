/// The above Rust code defines an enum `ResponsesServiceCodes` with associated error codes and descriptions, along with conversion and helper functions for working with these error codes.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
#[repr(u16)]

pub enum ResponsesServiceCodes {
  #[strum(props(
    Description = "An error occurred while reading the response or data from the server"
  ))]
  ReadingError = 611,
  #[strum(props(
    Description = "A connection issue occurred, preventing successful communication with the server"
  ))]
  ConnectionError = 612,
  #[strum(props(
    Description = "The reading operation exceeded the allowed time limit, resulting in a timeout"
  ))]
  ReadingTimeExpired = 613,
  #[strum(props(
    Description = "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols"
  ))]
  SSLHandshakeFailed = 614,
  #[strum(props(Description = "A generic error occurred while reading the response or data"))]
  AnotherReadingError = 615,
  #[strum(props(
    Description = "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input"
  ))]
  FBAAnomaly = 616,
  #[strum(props(
    Description = "An error in the implementation or logic caused the request to fail"
  ))]
  CodingError = 617,
  #[strum(props(
    Description = "The server issued a redirect response but did not provide a valid redirect URL"
  ))]
  RedirectWithoutRedirectURL = 618,
  #[strum(props(
    Description = "The DNS lookup for the specified domain failed, indicating a potential network or configuration issue"
  ))]
  DNSLookupFailed = 680,
  #[strum(props(
    Description = "The provided URL is syntactically incorrect and cannot be processed"
  ))]
  SyntacticallyIncorrectURL = 690,
  #[strum(props(
    Description = "The connection to the server was lost unexpectedly during communication"
  ))]
  LostConnection = 691,
  #[strum(props(
    Description = "The operation timed out while attempting to write data to the server"
  ))]
  WriteTimeout = 692,
  #[strum(props(
    Description = "The requested operation failed during a selection or matching process"
  ))]
  SelectionFailed = 693,
  #[strum(props(
    Description = "An error occurred while attempting to write data to the destination"
  ))]
  WriteError = 694,
  #[strum(props(
    Description = "A block header was incomplete or malformed, preventing further processing"
  ))]
  IncompleteBlockHeader = 695,
  #[strum(props(
    Description = "An unexpected error occurred, often indicative of an unforeseen issue or bug"
  ))]
  UnexpectedError = 699,
}

/// implementation of a custom trait `ToU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “to_u16” method which converts a value from the enumeration into a “u16” type. Self accesses the value of the enum In the implementation, it calls the `into()` method to perform the conversion, which relies on the `Into<u16>` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl ToU16 for ResponsesServiceCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

/// implementation of a custom trait `FromU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “from_u16” method which converts a value from the enumeration into an `Option<Self>` type. The method uses the `try_from` method to perform the conversion, which relies on the `TryFromPrimitive` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl FromU16 for ResponsesServiceCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

/// implementation of a custom trait `Into` for the `ResponsesLocalApiCodes` enumeration. We provide an “into” method which converts a value from the enumeration into a tuple containing a `u16` and a `&'static str`. The method calls the `to_u16` method to get the status code and the `get_str` method to get the description. The `unwrap_or` method is used to provide a default value in case the description is not found. The method returns the tuple containing the status code and the description
impl Into<(u16, &'static str)> for ResponsesServiceCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

/// The functions returns a tuple containing an unsigned 16-bit integer and a static string indicating that the operation was approved with no further action required.
pub fn reading_error_tuple() -> (u16, &'static str) {
  (611, "An error occurred while reading the response or data from the server")
}

pub fn connection_error_tuple() -> (u16, &'static str) {
  (612, "A connection issue occurred, preventing successful communication with the server")
}

pub fn reading_time_expired_tuple() -> (u16, &'static str) {
  (613, "The reading operation exceeded the allowed time limit, resulting in a timeout")
}

pub fn ssl_handshake_failed_tuple() -> (u16, &'static str) {
  (
    614,
    "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols",
  )
}

pub fn another_reading_error_tuple() -> (u16, &'static str) {
  (615, "A generic error occurred while reading the response or data")
}

pub fn fba_anomaly_tuple() -> (u16, &'static str) {
  (616, "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input")
}

pub fn coding_error_tuple() -> (u16, &'static str) {
  (617, "An error in the implementation or logic caused the request to fail")
}

pub fn redirect_without_redirect_url_tuple() -> (u16, &'static str) {
  (618, "The server issued a redirect response but did not provide a valid redirect URL")
}

pub fn dns_lookup_failed_tuple() -> (u16, &'static str) {
  (680, "The DNS lookup for the specified domain failed, indicating a potential network or configuration issue")
}

pub fn syntactically_incorrect_url_tuple() -> (u16, &'static str) {
  (690, "The provided URL is syntactically incorrect and cannot be processed")
}

pub fn lost_connection_tuple() -> (u16, &'static str) {
  (691, "The connection to the server was lost unexpectedly during communication")
}

pub fn write_timeout_tuple() -> (u16, &'static str) {
  (692, "The operation timed out while attempting to write data to the server")
}

pub fn selection_failed_tuple() -> (u16, &'static str) {
  (693, "The requested operation failed during a selection or matching process")
}

pub fn write_error_tuple() -> (u16, &'static str) {
  (694, "An error occurred while attempting to write data to the destination")
}

pub fn incomplete_block_header_tuple() -> (u16, &'static str) {
  (695, "A block header was incomplete or malformed, preventing further processing")
}

pub fn unexpected_error_tuple() -> (u16, &'static str) {
  (699, "An unexpected error occurred, often indicative of an unforeseen issue or bug")
}

/// The functions returns a tuple containing a status code and a JSON value with status and description fields.
pub fn reading_error() -> (u16, serde_json::Value) {
  let (code, desc) = reading_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn connection_error() -> (u16, serde_json::Value) {
  let (code, desc) = connection_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn reading_time_expired() -> (u16, serde_json::Value) {
  let (code, desc) = reading_time_expired_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn ssl_handshake_failed() -> (u16, serde_json::Value) {
  let (code, desc) = ssl_handshake_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn another_reading_error() -> (u16, serde_json::Value) {
  let (code, desc) = another_reading_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn fba_anomaly() -> (u16, serde_json::Value) {
  let (code, desc) = fba_anomaly_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn coding_error() -> (u16, serde_json::Value) {
  let (code, desc) = coding_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn redirect_without_redirect_url() -> (u16, serde_json::Value) {
  let (code, desc) = redirect_without_redirect_url_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn dns_lookup_failed() -> (u16, serde_json::Value) {
  let (code, desc) = dns_lookup_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn syntactically_incorrect_url() -> (u16, serde_json::Value) {
  let (code, desc) = syntactically_incorrect_url_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn lost_connection() -> (u16, serde_json::Value) {
  let (code, desc) = lost_connection_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn write_timeout() -> (u16, serde_json::Value) {
  let (code, desc) = write_timeout_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn selection_failed() -> (u16, serde_json::Value) {
  let (code, desc) = selection_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn write_error() -> (u16, serde_json::Value) {
  let (code, desc) = write_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn incomplete_block_header() -> (u16, serde_json::Value) {
  let (code, desc) = incomplete_block_header_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn unexpected_error() -> (u16, serde_json::Value) {
  let (code, desc) = unexpected_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

// Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_reading_error() {
    let response = ResponsesServiceCodes::ReadingError;
    let (code, description) = response.into();
    assert_eq!(code, 611);
    assert_eq!(description, "An error occurred while reading the response or data from the server");
  }

  #[test]
  fn test_to_u16_connection_error() {
    let response = ResponsesServiceCodes::ConnectionError;
    let code = response.to_u16();
    assert_eq!(code, 612);
  }

  #[test]
  fn test_from_u16_reading_time_expired() {
    let response = ResponsesServiceCodes::from_u16(613);
    assert_eq!(response, Some(ResponsesServiceCodes::ReadingTimeExpired));
  }

  #[test]
  fn test_ssl_handshake_failed() {
    assert_eq!
    (ssl_handshake_failed_tuple(), (
      614,
      "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols",
    ));
  }

  #[test]
  fn test_another_reading_error() {
    let (code, response) = another_reading_error_tuple();
    assert_eq!(code, 615);
    assert_eq!(response, "A generic error occurred while reading the response or data");
  }

  #[test]
  fn test_fba_anomaly() {
    let (code, response) = fba_anomaly();
    assert_eq!(code, 616);
    assert_eq!(
      response,
      json!({"status": 616, "description": "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input"}),
    );
  }
}
