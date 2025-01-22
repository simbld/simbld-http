use crate::generate_responses_functions;
/// Enum representing HTTP response status codes and descriptions.
/// Each variant corresponds to a specific HTTP status code.
///
/// * Example:
/// ```rust
///
/// use simbld_http::responses::ResponsesServiceCodes::ReadingError;
///
/// let response = ReadingError;
/// let json = response.as_json();
/// println!("{:?}", json);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum ResponsesServiceCodes {
  ReadingError,
  ConnectionError,
  ReadingTimeExpired,
  SSLHandshakeFailed,
  AnotherReadingError,
  FBAAnomaly,
  CodingError,
  RedirectWithoutRedirectURL,
  DNSLookupFailed,
  SyntacticallyIncorrectURL,
  LostConnection,
  WriteTimeout,
  SelectionFailed,
  WriteError,
  IncompleteBlockHeader,
  UnexpectedError,
}

generate_responses_functions! {
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

/// This file defines the `ResponsesServiceCodes` enum and provides five main functionalities:
/// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
/// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
/// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
/// 4. `as_json()` - converts the variant to a JSON object.
/// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::{ResponsesServiceCodes, UnifiedTuple};

  #[test]
  fn test_service_codes_to_u16() {
    let response = ResponsesServiceCodes::ReadingError;
    let code = response.to_u16();
    assert_eq!(code, 611);
  }

  #[test]
  fn test_service_codes_from_u16() {
    let status = ResponsesServiceCodes::from_u16(611);
    assert_eq!(status, Some(ResponsesServiceCodes::ReadingError));
  }

  #[test]
  fn test_service_codes_as_tuple() {
    let code = ResponsesServiceCodes::ConnectionError;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::FiveFields(
        500,
        "Internal Server Error",
        "A connection issue occurred, preventing successful communication with the server",
        612,
        "Connection Error"
      )
    );
  }

  #[test]
  fn test_service_codes_as_json() {
    let code = ResponsesServiceCodes::ReadingTimeExpired;
    let json_result = code.as_json();
    let expected_json = json!({
        "standard http code": {
            "code": 500,
            "name": "Internal Server Error"
        },
        "internal http code": {
            "code": 613,
            "name": "Reading Time Expired"
        },
        "description": "The reading operation exceeded the allowed time limit, resulting in a timeout"
    });
    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_service_codes_into_tuple() {
    let code = ResponsesServiceCodes::SSLHandshakeFailed;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 500);
    assert_eq!(std_name, "Internal Server Error");
  }
}
