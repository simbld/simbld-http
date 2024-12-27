/// The above Rust code defines an enum representing informational HTTP response codes with associated descriptions and provides functions to retrieve code-description pairs.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
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

/// implementation of a custom trait `ToU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “to_u16” method which converts a value from the enumeration into a “u16” type. Self accesses the value of the enum In the implementation, it calls the `into()` method to perform the conversion, which relies on the `Into<u16>` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl ToU16 for ResponsesInformationalCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

/// implementation of a custom trait `FromU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “from_u16” method which converts a value from the enumeration into an `Option<Self>` type. The method uses the `try_from` method to perform the conversion, which relies on the `TryFromPrimitive` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl FromU16 for ResponsesInformationalCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

/// implementation of a custom trait `Into` for the `ResponsesLocalApiCodes` enumeration. We provide an “into” method which converts a value from the enumeration into a tuple containing a `u16` and a `&'static str`. The method calls the `to_u16` method to get the status code and the `get_str` method to get the description. The `unwrap_or` method is used to provide a default value in case the description is not found. The method returns the tuple containing the status code and the description
impl Into<(u16, &'static str)> for ResponsesInformationalCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description)
  }
}

/// Functions return raw data as a tuple for further processing or formats containing HTTP status code, status message and description of various client error responses.
pub fn continue_request_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::ContinueRequest;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Continue Request", description)
}

pub fn switching_protocols_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::SwitchingProtocols;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Switching Protocols", description)
}

pub fn processing_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::Processing;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Processing", description)
}

pub fn early_hints_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::EarlyHints;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Early Hints", description)
}

pub fn connection_reset_by_peer_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::ConnectionResetByPeer;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Connection Reset By Peer", description)
}

pub fn name_not_resolved_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::NameNotResolved;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Name Not Resolved", description)
}

pub fn no_response_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::NoResponse;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "No Response", description)
}

pub fn retry_with_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::RetryWith;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Retry With", description)
}

pub fn response_is_stale_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::ResponseIsStale;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Response Is Stale", description)
}

pub fn revalidation_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesInformationalCodes::RevalidationFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Revalidation Failed", description)
}

/// Functions return formatted data as JSON containing HTTP status code, status message and description of various informational responses.
pub fn continue_request() -> serde_json::Value {
  let (code, name, desc) = continue_request_tuple();
  json!({ "status": code, "name": name, "description": desc })
}
pub fn switching_protocols() -> serde_json::Value {
  let (code, name, desc) = switching_protocols_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn processing() -> serde_json::Value {
  let (code, name, desc) = processing_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn early_hints() -> serde_json::Value {
  let (code, name, desc) = early_hints_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn connection_reset_by_peer() -> serde_json::Value {
  let (code, name, desc) = connection_reset_by_peer_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn name_not_resolved() -> serde_json::Value {
  let (code, name, desc) = name_not_resolved_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn no_response() -> serde_json::Value {
  let (code, name, desc) = no_response_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn retry_with() -> serde_json::Value {
  let (code, name, desc) = retry_with_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn response_is_stale() -> serde_json::Value {
  let (code, name, desc) = response_is_stale_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn revalidation_failed() -> serde_json::Value {
  let (code, name, desc) = revalidation_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
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
