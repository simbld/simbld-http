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
    (code, description)
  }
}

/// Functions return raw data as a tuple for further processing or formats containing HTTP status code, status message and description of various client error responses.
pub fn reading_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::ReadingError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Reading Error", description)
}

pub fn connection_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::ConnectionError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Connection Error", description)
}

pub fn reading_time_expired_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::ReadingTimeExpired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Reading Time Expired", description)
}

pub fn ssl_handshake_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::SSLHandshakeFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "SSL Handshake Failed", description)
}

pub fn another_reading_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::AnotherReadingError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Another Reading Error", description)
}

pub fn fba_anomaly_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::FBAAnomaly;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "FBA Anomaly", description)
}

pub fn coding_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::CodingError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Coding Error", description)
}

pub fn redirect_without_redirect_url_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::RedirectWithoutRedirectURL;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Redirect Without Redirect URL", description)
}

pub fn dns_lookup_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::DNSLookupFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "DNS Lookup Failed", description)
}

pub fn syntactically_incorrect_url_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::SyntacticallyIncorrectURL;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Syntactically Incorrect URL", description)
}

pub fn lost_connection_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::LostConnection;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Lost Connection", description)
}

pub fn write_timeout_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::WriteTimeout;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Write Timeout", description)
}

pub fn selection_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::SelectionFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Selection Failed", description)
}

pub fn write_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::WriteError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Write Error", description)
}

pub fn incomplete_block_header_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::IncompleteBlockHeader;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Incomplete Block Header", description)
}

pub fn unexpected_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServiceCodes::UnexpectedError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unexpected Error", description)
}

/// Functions return formatted data as JSON containing HTTP status code, status message and description of various informational responses.
pub fn reading_error() -> serde_json::Value {
  let (code, name, desc) = reading_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn connection_error() -> serde_json::Value {
  let (code, name, desc) = connection_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn reading_time_expired() -> serde_json::Value {
  let (code, name, desc) = reading_time_expired_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn ssl_handshake_failed() -> serde_json::Value {
  let (code, name, desc) = ssl_handshake_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn another_reading_error() -> serde_json::Value {
  let (code, name, desc) = another_reading_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn fba_anomaly() -> serde_json::Value {
  let (code, name, desc) = fba_anomaly_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn coding_error() -> serde_json::Value {
  let (code, name, desc) = coding_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn redirect_without_redirect_url() -> serde_json::Value {
  let (code, name, desc) = redirect_without_redirect_url_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn dns_lookup_failed() -> serde_json::Value {
  let (code, name, desc) = dns_lookup_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn syntactically_incorrect_url() -> serde_json::Value {
  let (code, name, desc) = syntactically_incorrect_url_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn lost_connection() -> serde_json::Value {
  let (code, name, desc) = lost_connection_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn write_timeout() -> serde_json::Value {
  let (code, name, desc) = write_timeout_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn selection_failed() -> serde_json::Value {
  let (code, name, desc) = selection_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn write_error() -> serde_json::Value {
  let (code, name, desc) = write_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn incomplete_block_header() -> serde_json::Value {
  let (code, name, desc) = incomplete_block_header_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unexpected_error() -> serde_json::Value {
  let (code, name, desc) = unexpected_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
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
      "SSL Handshake Failed",
      "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols",
    ));
  }

  #[test]
  fn test_another_reading_error() {
    let (code, name, response) = another_reading_error_tuple();
    assert_eq!(code, 615);
    assert_eq!(name, "Another Reading Error");
    assert_eq!(response, "A generic error occurred while reading the response or data");
  }

  #[test]
  fn test_fba_anomaly() {
    let response = fba_anomaly();
    assert_eq!(response["status"], 616);
    assert_eq!(
      response["description"],
      "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input"
    );
  }
}
