use crate::helpers::generate_responses_functions;
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

/// The above Rust code defines an enum representing informational HTTP response codes with associated descriptions and provides functions to retrieve code-description pairs.
generate_responses_functions! {
  #[derive(Debug, Clone, PartialEq)]
  #[repr(u16)]
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

// Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_continue_request() {
    let response = ResponsesInformationalCodes::ContinueRequest;
    let (code, description) = response.into();
    assert_eq!(code, 100);
    assert_eq!(description, "The server has received the initial part of the request, the headers, and asks the client to continue request, proceed to send the body of the request, a POST request");
  }

  #[test]
  fn test_to_16_switching_protocols() {
    let response = ResponsesInformationalCodes::SwitchingProtocols;
    let code = response.to_u16();
    assert_eq!(code, 101);
  }

  #[test]
  fn test_from_u16_processing() {
    let response = ResponsesInformationalCodes::from_u16(102);
    assert_eq!(response, Some(ResponsesInformationalCodes::Processing));
  }

  #[test]
  fn test_switching_protocols_tuple() {
    assert_eq!(
      switching_protocols_tuple(),
      (
        101,
        "Switching Protocols",
        "The server is complying with a request to switch protocols, used in WebSocket connections"
      )
    );
  }

  #[test]
  fn test_revalidation_failed() {
    let (code, name, description) = revalidation_failed_tuple();
    assert_eq!(code, 109);
    assert_eq!(name, "Revalidation Failed");
    assert_eq!(description, "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired");
  }
}
