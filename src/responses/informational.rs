use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::helpers::to_u16_helper::ToU16;
use strum_macros::{EnumIter, EnumProperty};

#[derive(IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
#[repr(u16)]
pub enum ResponsesInformationalCodes {
  #[strum(props(
    Description = "The server has received the initial part of the request, the headers, and asks the client to continue, proceed to send the body of the request, a POST request"
  ))]
  Continue = 100,
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
