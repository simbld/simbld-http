/// The above Rust code defines an enum `ResponsesServiceCodes` with associated error codes and descriptions, along with conversion and helper functions for working with these error codes.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
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

impl ToU16 for ResponsesServiceCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesServiceCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

impl Into<(u16, &'static str)> for ResponsesServiceCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

pub fn reading_error() -> (u16, &'static str) {
  (611, "An error occurred while reading the response or data from the server")
}

pub fn connection_error() -> (u16, &'static str) {
  (612, "A connection issue occurred, preventing successful communication with the server")
}

pub fn reading_time_expired() -> (u16, &'static str) {
  (613, "The reading operation exceeded the allowed time limit, resulting in a timeout")
}

pub fn ssl_handshake_failed() -> (u16, &'static str) {
  (
    614,
    "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols",
  )
}

pub fn another_reading_error() -> (u16, &'static str) {
  (615, "A generic error occurred while reading the response or data")
}

pub fn fba_anomaly() -> (u16, &'static str) {
  (616, "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input")
}

pub fn coding_error() -> (u16, &'static str) {
  (617, "An error in the implementation or logic caused the request to fail")
}

pub fn redirect_without_redirect_url() -> (u16, &'static str) {
  (618, "The server issued a redirect response but did not provide a valid redirect URL")
}

pub fn dns_lookup_failed() -> (u16, &'static str) {
  (680, "The DNS lookup for the specified domain failed, indicating a potential network or configuration issue")
}

pub fn syntactically_incorrect_url() -> (u16, &'static str) {
  (690, "The provided URL is syntactically incorrect and cannot be processed")
}

pub fn lost_connection() -> (u16, &'static str) {
  (691, "The connection to the server was lost unexpectedly during communication")
}

pub fn write_timeout() -> (u16, &'static str) {
  (692, "The operation timed out while attempting to write data to the server")
}

pub fn selection_failed() -> (u16, &'static str) {
  (693, "The requested operation failed during a selection or matching process")
}

pub fn write_error() -> (u16, &'static str) {
  (694, "An error occurred while attempting to write data to the destination")
}

pub fn incomplete_block_header() -> (u16, &'static str) {
  (695, "A block header was incomplete or malformed, preventing further processing")
}

pub fn unexpected_error() -> (u16, &'static str) {
  (699, "An unexpected error occurred, often indicative of an unforeseen issue or bug")
}

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
    (ssl_handshake_failed(), (
      614,
      "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols",
    ));
  }
}
