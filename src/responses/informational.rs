/// The above Rust code defines an enum representing informational HTTP response codes with associated descriptions and provides functions to retrieve code-description pairs.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
#[repr(u16)]
pub enum ResponsesInformationalCodes {
  #[strum(props(
    Description = "The server has received the initial part of the request, the headers, and asks the client to continue request, proceed to send the body of the request, a POST request"
  ))]
  ContinueRequest = 100,
  #[strum(props(
    Description = "The server is complying with a request to switch protocols, used in WebSocket connections"
  ))]
  SwitchingProtocols = 101,
  #[strum(props(
    Description = "Indicates the server is processing the request but has not yet finished, used to prevent timeout errors in asynchronous operations, webdav RFC 2518"
  ))]
  Processing = 102,
  #[strum(props(
    Description = "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared"
  ))]
  EarlyHints = 103,
  #[strum(props(
    Description = "The connection was forcibly closed by a peer, possibly due to a protocol error, a timeout, or a network issue"
  ))]
  ConnectionResetByPeer = 104,
  #[strum(props(
    Description = "The server could not resolve the domain name provided in the request, indicating a DNS lookup failure, The requested hostname cannot be resolved to an IP address"
  ))]
  NameNotResolved = 105,
  #[strum(props(
    Description = "The server did not provide a response, possibly due to a timeout or a connection issue, The server didn’t send any response within the timeout period. This status code is not specified in any RFCs, but it is used in some scenarios to indicate that the server closed the connection without sending any response"
  ))]
  NoResponse = 106,
  #[strum(props(
    Description = "The server indicates that the client should retry the request with appropriate changes or additional information, new or different credentials, use a different protocol or in a different location"
  ))]
  RetryWith = 107,
  #[strum(props(
    Description = "The response returned by the server is stale and should be revalidated, indicating that the cached response is outdated or expired"
  ))]
  ResponseIsStale = 108,
  #[strum(props(
    Description = "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired"
  ))]
  RevalidationFailed = 109,
}

impl ToU16 for ResponsesInformationalCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesInformationalCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

impl Into<(u16, &'static str)> for ResponsesInformationalCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

pub fn continue_request() -> (u16, &'static str) {
  (100, "The server has received the initial part of the request, the headers, and asks the client to continue request, proceed to send the body of the request, a POST request")
}

pub fn switching_protocols() -> (u16, &'static str) {
  (101, "The server is complying with a request to switch protocols, used in WebSocket connections")
}

pub fn processing() -> (u16, &'static str) {
  (102, "Indicates the server is processing the request but has not yet finished, used to prevent timeout errors in asynchronous operations, webdav RFC 2518")
}

pub fn early_hints() -> (u16, &'static str) {
  (103, "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared")
}

pub fn connection_reset_by_peer() -> (u16, &'static str) {
  (104, "The connection was forcibly closed by a peer, possibly due to a protocol error, a timeout, or a network issue")
}

pub fn name_not_resolved() -> (u16, &'static str) {
  (105, "The server could not resolve the domain name provided in the request, indicating a DNS lookup failure, The requested hostname cannot be resolved to an IP address")
}

pub fn no_response() -> (u16, &'static str) {
  (106, "The server did not provide a response, possibly due to a timeout or a connection issue, The server didn’t send any response within the timeout period. This status code is not specified in any RFCs, but it is used in some scenarios to indicate that the server closed the connection without sending any response")
}

pub fn retry_with() -> (u16, &'static str) {
  (107, "The server indicates that the client should retry the request with appropriate changes or additional information, new or different credentials, use a different protocol or in a different location")
}

pub fn response_is_stale() -> (u16, &'static str) {
  (108, "The response returned by the server is stale and should be revalidated, indicating that the cached response is outdated or expired")
}

pub fn revalidation_failed() -> (u16, &'static str) {
  (109, "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired")
}

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
  fn test_early_hints() {
    assert_eq!(early_hints(), (103, "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared"))
  }

  #[test]
  fn test_from_u16_processing() {
    let response = ResponsesInformationalCodes::from_u16(102);
    assert_eq!(response, Some(ResponsesInformationalCodes::Processing));
  }
}
