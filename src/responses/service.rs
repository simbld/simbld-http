/// The above Rust code defines an enum `ResponsesServiceCodes` with associated error codes and descriptions, along with conversion and helper functions for working with these error codes.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

generate_responses_functions! {
  #[derive(Debug, Clone, PartialEq)]
  #[repr(u16)]
  ResponsesServiceCodes,
  ReadingError => (500, "Internal Server Error", "An error occurred while reading the response or data from the server", 611, "Reading Error"),
  ConnectionError => (500, "Internal Server Error", "A connection issue occurred, preventing successful communication with the server", 612, "Connection Error"),
  ReadingTimeExpired => (500, "Internal Server Error", "The reading operation exceeded the allowed time limit, resulting in a timeout", 613, "Reading Time Expired"),
  SSLHandshakeFailed => (500, "Internal Server Error", "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols", 614, "SSL Handshake Failed"),
  AnotherReadingError => (500, "Internal Server Error", "A generic error occurred while reading the response or data", 615, "Another Reading Error"),
  FBAAnomaly => (500, "Internal Server Error", "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input", 616, "FBA Anomaly"),
  CodingError => (500, "Internal Server Error", "An error in the implementation or logic caused the request to fail", 617, "Coding Error"),
  RedirectWithoutRedirectURL => (500, "Internal Server Error", "The server issued a redirect response but did not provide a valid redirect URL", 618, "Redirect Without Redirect URL"),
  DNSLookupFailed => (500, "Internal Server Error", "The DNS lookup for the specified domain failed, indicating a potential network or configuration issue", 680, "DNS Lookup Failed"),
  SyntacticallyIncorrectURL => (500, "Internal Server Error", "The provided URL is syntactically incorrect and cannot be processed", 690, "Syntactically Incorrect URL"),
  LostConnection => (500, "Internal Server Error", "The connection to the server was lost unexpectedly during communication", 691, "Lost Connection"),
  WriteTimeout => (500, "Internal Server Error", "The operation timed out while attempting to write data to the server", 692, "Write Timeout"),
  SelectionFailed => (500, "Internal Server Error", "The requested operation failed during a selection or matching process", 693, "Selection Failed"),
  WriteError => (500, "Internal Server Error", "An error occurred while attempting to write data to the destination", 694, "Write Error"),
  IncompleteBlockHeader => (500, "Internal Server Error", "A block header was incomplete or malformed, preventing further processing", 695, "Incomplete Block Header"),
  UnexpectedError => (500, "Internal Server Error", "An unexpected error occurred, often indicative of an unforeseen issue or bug", 699, "Unexpected Error"),
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
