// Enum representing HTTP response status codes and descriptions.
// Each variant corresponds to a specific HTTP status code.
//
// * Example:
// ```rust
//
// use simbld_http::responses::ResponsesInformationalCodes::ContinueRequest;
//
// let response = ContinueRequest;
// let json = response.as_json();
// println!("{:?}", json);
// ```
use crate::generate_responses_functions;
use crate::UnifiedTuple;


generate_responses_functions! {
  ResponsesInformationalCodes,
  ContinueRequest => (100, "Continue", "The server has received the initial part of the request, the headers, and asks the client to continue request, proceed to send the body of the request, a POST request", 100, "Continue Request"),
  SwitchingProtocols => (101, "Switching Protocols", "The server is complying with a request to switch protocols, used in WebSocket connections", 101, "Switching Protocols"),
  Processing => (102, "Processing", "Indicates the server is processing the request but has not yet finished, used to prevent timeout errors in asynchronous operations, webdav RFC 2518", 102, "Processing"),
  EarlyHints => (103, "Early Hints", "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared", 103, "Early Hints"),
  ConnectionResetByPeer => (100, "Continue", "The connection was forcibly closed by a peer, possibly due to a protocol error, a timeout, or a network issue", 104, "Connection Reset By Peer"),
  NameNotResolved => (100, "Continue", "The server could not resolve the domain name provided in the request, indicating a DNS lookup failure, The requested hostname cannot be resolved to an IP address", 105, "Name Not Resolved"),
  NoResponse => (100, "Continue", "The server did not provide a response, possibly due to a timeout or a connection issue, The server didn’t send any response within the timeout period. This status code is not specified in any RFCs, but it is used in some scenarios to indicate that the server closed the connection without sending any response", 106, "No Response"),
  RetryWith => (100, "Continue", "The server indicates that the client should retry the request with appropriate changes or additional information, new or different credentials, use a different protocol or in a different location", 107, "Retry With"),
  ResponseIsStale => (100, "Continue", "The response returned by the server is stale and should be revalidated, indicating that the cached response is outdated or expired", 108, "Response Is Stale"),
  RevalidationFailed => (100, "Continue", "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired", 109, "Revalidation Failed"),
}

// This file defines the `ResponsesInformationalCodes` enum and provides five main functionalities:
// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
// 4. `as_json()` - converts the variant to a JSON object.
// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::{ResponsesInformationalCodes, UnifiedTuple};

  #[test]
  fn test_to_16_switching_protocols() {
    let response = ResponsesInformationalCodes::SwitchingProtocols;
    let code = response.to_u16();
    assert_eq!(code, 101);
  }

  #[test]
  fn test_processing_codes_from_u16() {
    let status = ResponsesInformationalCodes::from_u16(102);
    assert_eq!(status, Some(ResponsesInformationalCodes::Processing));
  }

  #[test]
  fn test_response_is_stale_codes_as_tuple() {
    let code = ResponsesInformationalCodes::ResponseIsStale;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::FiveFields(
        100,
        "Continue",
        "The response returned by the server is stale and should be revalidated, indicating that the cached response is outdated or expired",
        108,
        "Response Is Stale"
      )
    );
  }

  #[test]
  fn test_revalidation_failed_codes_as_json() {
    let code = ResponsesInformationalCodes::RevalidationFailed;
    let json_result = code.as_json();
    let expected_json = json!({
      "standard http code": {
        "code": 100,
        "name": "Continue"
      },
      "internal http code": {
        "code": 109,
        "name": "Revalidation Failed"
      },
      "description": "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired"
    });
    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_continue_request_codes_into_tuple() {
    let code = ResponsesInformationalCodes::ContinueRequest;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 100);
    assert_eq!(std_name, "Continue");
  }
}
