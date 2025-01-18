/// The above Rust code defines an enum representing HTTP server response codes with associated descriptions and provides functions to retrieve specific response code details.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

generate_responses_functions! {
  #[derive(Debug, Clone, PartialEq)]
  #[repr(u16)]
  ResponsesServerCodes,
  InternalServerError => (500, "Internal Server Error", "The server encountered an unexpected condition that prevented it from fulfilling the request. This could be due to a misconfiguration, an unhandled exception, or resource exhaustion", 500, "Internal Server Error"),
  NotImplemented => (501, "Not Implemented", "The server does not support the functionality required to fulfill the request. This might be because the server does not recognize the request method or lacks the capability to process it", 501, "Not Implemented"),
  BadGateway => (502, "Bad Gateway", "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured", 502, "Bad Gateway"),
  ServiceUnavailable => (503, "Service Unavailable", "The server is currently unable to handle the request due to temporary overloading or maintenance. This is usually a temporary state", 503, "Service Unavailable"),
  GatewayTimeout => (504, "Gateway Timeout", "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. This could be due to network congestion or the upstream server being overloaded", 504, "Gateway Timeout"),
  HTTPVersionNotSupported => (505, "HTTP Version Not Supported", "The server does not support the HTTP protocol version used in the request. This prevents the server from processing the request", 505, "HTTP Version Not Supported"),
  VariantAlsoNegotiates => (506, "Variant Also Negotiates", "The server encountered a configuration error in transparent content negotiation. This resulted in a circular reference that prevents the server from serving the requested content", 506, "Variant Also Negotiates"),
  InsufficientStorage => (507, "Insufficient Storage", "The server is unable to store the representation needed to complete the request. This could be due to storage limits being reached or allocation constraints", 507, "Insufficient Storage"),
  LoopDetected => (508, "Loop Detected", "The server detected an infinite loop while processing a request. This is often due to circular references or recursive function calls in WebDAV configurations, RFC 5842", 508, "Loop Detected"),
  BandwidthLimitExceeded => (509, "Bandwidth Limit Exceeded", "The server's bandwidth limit has been exceeded. This limit is typically set by the administrator and prevents further data transfer until the limit resets, often used by hosting providers to prevent abuse, apache, unofficial, Cpanel", 509, "Bandwidth Limit Exceeded"),
  NotExtended => (510, "Not Extended", "The server requires further extensions to fulfill the request. This could mean additional client conditions or protocol extensions are necessary before the server can process the request", 510, "Not Extended"),
  NetworkAuthenticationRequired => (511, "Network Authentication Required", "The network connection requires authentication before accessing the requested resources. This is often used by captive portals to redirect users to a login page", 511, "Network Authentication Required"),
  UnknownError => (500, "Internal Server Error", "An unspecified error occurred, and the server was unable to provide more details. This is a catch-all for unexpected conditions, (Cloudflare extension)", 520, "Unknown Error"),
  WebServerIsDown => (502, "Bad Gateway", "Cloudflare, unofficial is currently unreachable, likely due to downtime or maintenance. This prevents the server from processing the request, and the client should try again later", 521, "Web Server Is Down"),
  ConnectionTimedOut => (504, "Gateway Timeout", "The connection to the server timed out before a response could be received. This could be due to network issues or server overload", 522, "Connection Timed Out"),
  OriginIsUnreachable => (502, "Bad Gateway", "The origin server could not be contacted. This might be due to network issues or misconfiguration", 523, "Origin Is Unreachable"),
  TimeoutOccurred => (504, "Gateway Timeout", "The operation timed out while waiting for a response from the server. This could be due to network congestion or server overload", 524, "Timeout Occurred"),
  SSLHandshakeFailed => (525, "SSL Handshake Failed", "The SSL/TLS handshake failed, preventing a secure connection from being established. This could be due to certificate issues or network problems", 525, "SSL Handshake Failed"),
  InvalidSSLCertificate => (526, "Invalid SSL Certificate", "The SSL/TLS certificate provided by the server is invalid, expired, or does not match the requested domain. This prevents the secure connection from being established", 526, "Invalid SSL Certificate"),
  RailgunError => (527, "Railgun Error", "An error occurred in the Railgun service, which accelerates connections between Cloudflare and the origin server. This may indicate a misconfiguration or temporary service unavailability", 527, "Railgun Error"),
  SiteIsOverloaded => (529, "Site Is Overloaded", "Indicates the Qualys server cannot process the request, likely due to high traffic or resource constraints. This is a Qualys-specific status code, unofficial", 529, "Site Is Overloaded"),
  SiteIsFrozen => (530, "Site Is Frozen", "Indicates the Pantheon server has been frozen due to inactivity, preventing further requests from being processed. This is a Pantheon-specific status code, unofficial", 530, "Site Is Frozen"),
  OriginDNSError => (531, "Origin DNS Error", "The origin server encountered a DNS resolution error while attempting to process the request. This typically occurs when the domain name cannot be resolved to an IP address, possibly due to a misconfiguration or network issue", 531, "Origin DNS Error"),
  NoSiteDetected => (500, "Internal Server Error", "This error is specific to certain hosting environments. For AWS, it indicates an HTTP Authentication failure, whereas for Pantheon, it means there is a problem with the site configuration, no site detected / AWS or Pantheon config error.", 561, "No Site Detected"),
  NetworkReadTimeoutError => (598, "Network Read Timeout Error", "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network returned read timeouts", 598, "Network Read Timeout Error"),
  NetworkConnectTimeoutError => (599, "Network Connect Timeout Error", "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network timed out", 599, "Network Connect Timeout Error"),
}

// Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_internal_server_error() {
    let response = ResponsesServerCodes::InternalServerError;
    let (code, description): (u16, &str) = response.into();
    assert_eq!(code, 500);
    assert_eq!(description, "The server encountered an unexpected condition that prevented it from fulfilling the request. This could be due to a misconfiguration, an unhandled exception, or resource exhaustion");
  }

  #[test]
  fn test_to_16_not_implemented() {
    let response = ResponsesServerCodes::NotImplemented;
    let code = response.to_u16();
    assert_eq!(code, 501);
  }

  #[test]
  fn test_bad_gateway() {
    assert_eq!(bad_gateway_tuple(), (502, "Bad Gateway", "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured"));
  }

  #[test]
  fn test_from_u16_service_unavailable() {
    let response = ResponsesServerCodes::from_u16(503);
    assert_eq!(response, Some(ResponsesServerCodes::ServiceUnavailable));
  }

  #[test]
  fn test_gateway_timeout() {
    let (code, name, response) = gateway_timeout_tuple();
    assert_eq!(code, 504);
    assert_eq!(name, "Gateway Timeout");
    assert_eq!(response, "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. This could be due to network congestion or the upstream server being overloaded");
  }

  #[test]
  fn test_http_version_not_supported() {
    let response = http_version_not_supported();
    assert_eq!(
      response,
      json!({ "status": 505, "name": "Http Version Not Supported", "description": "The server does not support the HTTP protocol version used in the request. This prevents the server from processing the request" })
    );
  }
}
