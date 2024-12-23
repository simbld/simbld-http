/// The above Rust code defines an enum representing HTTP server response codes with associated descriptions and provides functions to retrieve specific response code details.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
#[repr(u16)]

pub enum ResponsesServerCodes {
  #[strum(props(
    Description = "The server encountered an unexpected condition that prevented it from fulfilling the request. This could be due to a misconfiguration, an unhandled exception, or resource exhaustion"
  ))]
  InternalServerError = 500,
  #[strum(props(
    Description = "The server does not support the functionality required to fulfill the request. This might be because the server does not recognize the request method or lacks the capability to process it"
  ))]
  NotImplemented = 501,
  #[strum(props(
    Description = "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured"
  ))]
  BadGateway = 502, // WARNING: can affect the rate at which Googlebot explanation :: https://http.dev/502
  #[strum(props(
    Description = "The server is currently unable to handle the request due to temporary overloading or maintenance. This is usually a temporary state"
  ))]
  ServiceUnavailable = 503,
  #[strum(props(
    Description = "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. This could be due to network congestion or the upstream server being overloaded"
  ))]
  GatewayTimeout = 504,
  #[strum(props(
    Description = "The server does not support the HTTP protocol version used in the request. This prevents the server from processing the request"
  ))]
  HTTPVersionNotSupported = 505,
  #[strum(props(
    Description = "The server encountered a configuration error in transparent content negotiation. This resulted in a circular reference that prevents the server from serving the requested content"
  ))]
  VariantAlsoNegotiates = 506,
  #[strum(props(
    Description = "The server is unable to store the representation needed to complete the request. This could be due to storage limits being reached or allocation constraints"
  ))]
  InsufficientStorage = 507,
  #[strum(props(
    Description = "The server detected an infinite loop while processing a request. This is often due to circular references or recursive function calls in WebDAV configurations, RFC 5842"
  ))]
  LoopDetected = 508,
  #[strum(props(
    Description = "The server's bandwidth limit has been exceeded. This limit is typically set by the administrator and prevents further data transfer until the limit resets, often used by hosting providers to prevent abuse, apache, unofficial, Cpanel"
  ))]
  BandwidthLimitExceeded = 509,
  #[strum(props(
    Description = "The server requires further extensions to fulfill the request. This could mean additional client conditions or protocol extensions are necessary before the server can process the request"
  ))]
  NotExtended = 510,
  #[strum(props(
    Description = "The network connection requires authentication before accessing the requested resources. This is often used by captive portals to redirect users to a login page"
  ))]
  NetworkAuthenticationRequired = 511,
  #[strum(props(
    Description = "An unspecified error occurred, and the server was unable to provide more details. This is a catch-all for unexpected conditions"
  ))]
  UnknownError = 520,
  #[strum(props(
    Description = "Cloudflare, unofficial is currently unreachable, likely due to downtime or maintenance. This prevents the server from processing the request, and the client should try again later"
  ))]
  WebServerIsDown = 521,
  #[strum(props(
    Description = "The connection to the server timed out before a response could be received. This could be due to network issues or server overload"
  ))]
  ConnectionTimedOut = 522,
  #[strum(props(
    Description = "The origin server could not be contacted. This might be due to network issues or misconfiguration"
  ))]
  OriginIsUnreachable = 523,
  #[strum(props(
    Description = "The operation timed out while waiting for a response from the server. This could be due to network congestion or server overload"
  ))]
  TimeoutOccurred = 524,
  #[strum(props(
    Description = "The SSL/TLS handshake failed, preventing a secure connection from being established. This could be due to certificate issues or network problems"
  ))]
  SSLHandshakeFailed = 525,
  #[strum(props(
    Description = "The SSL/TLS certificate provided by the server is invalid, expired, or does not match the requested domain. This prevents the secure connection from being established"
  ))]
  InvalidSSLCertificate = 526,
  #[strum(props(
    Description = "An error occurred in the Railgun service, which accelerates connections between Cloudflare and the origin server. This may indicate a misconfiguration or temporary service unavailability"
  ))]
  RailgunError = 527,
  #[strum(props(
    Description = "Indicates the Qualys server cannot process the request, likely due to high traffic or resource constraints. This is a Qualys-specific status code, unofficial"
  ))]
  SiteIsOverloaded = 529,
  #[strum(props(
    Description = "Indicates the Pantheon server has been frozen due to inactivity, preventing further requests from being processed. This is a Pantheon-specific status code, unofficial"
  ))]
  SiteIsFrozen = 530,
  #[strum(props(
    Description = "The origin server encountered a DNS resolution error while attempting to process the request. This typically occurs when the domain name cannot be resolved to an IP address, possibly due to a misconfiguration or network issue"
  ))]
  OriginDNSError = 531,
  #[strum(props(
    Description = "This error is specific to certain hosting environments. For AWS, it indicates an HTTP Authentication failure, whereas for Pantheon, it means there is a problem with the site configuration"
  ))]
  NoSiteDetected = 561,
  #[strum(props(
    Description = "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network returned read timeouts"
  ))]
  NetworkReadTimeoutError = 598,
  #[strum(props(
    Description = "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network timed out"
  ))]
  NetworkConnectTimeoutError = 599,
}

/// implementation of a custom trait `ToU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “to_u16” method which converts a value from the enumeration into a “u16” type. Self accesses the value of the enum In the implementation, it calls the `into()` method to perform the conversion, which relies on the `Into<u16>` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl ToU16 for ResponsesServerCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

/// implementation of a custom trait `FromU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “from_u16” method which converts a value from the enumeration into an `Option<Self>` type. The method uses the `try_from` method to perform the conversion, which relies on the `TryFromPrimitive` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl FromU16 for ResponsesServerCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

/// implementation of a custom trait `Into` for the `ResponsesLocalApiCodes` enumeration. We provide an “into” method which converts a value from the enumeration into a tuple containing a `u16` and a `&'static str`. The method calls the `to_u16` method to get the status code and the `get_str` method to get the description. The `unwrap_or` method is used to provide a default value in case the description is not found. The method returns the tuple containing the status code and the description
impl Into<(u16, &'static str)> for ResponsesServerCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

/// The functions returns a tuple containing an unsigned 16-bit integer and a static string indicating that the operation was approved with no further action required.
pub fn internal_server_error_tuple() -> (u16, &'static str) {
  (500, "The server encountered an unexpected condition that prevented it from fulfilling the request. This could be due to a misconfiguration, an unhandled exception, or resource exhaustion")
}

pub fn not_implemented_tuple() -> (u16, &'static str) {
  (501, "The server does not support the functionality required to fulfill the request. This might be because the server does not recognize the request method or lacks the capability to process it")
}

pub fn bad_gateway_tuple() -> (u16, &'static str) {
  (502, "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured")
}

pub fn service_unavailable_tuple() -> (u16, &'static str) {
  (503, "The server is currently unable to handle the request due to temporary overloading or maintenance. This is usually a temporary state")
}

pub fn gateway_timeout_tuple() -> (u16, &'static str) {
  (504, "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. This could be due to network congestion or the upstream server being overloaded")
}

pub fn http_version_not_supported_tuple() -> (u16, &'static str) {
  (505, "The server does not support the HTTP protocol version used in the request. This prevents the server from processing the request")
}

pub fn variant_also_negotiates_tuple() -> (u16, &'static str) {
  (506, "The server encountered a configuration error in transparent content negotiation. This resulted in a circular reference that prevents the server from serving the requested content")
}

pub fn insufficient_storage_tuple() -> (u16, &'static str) {
  (507, "The server is unable to store the representation needed to complete the request. This could be due to storage limits being reached or allocation constraints")
}

pub fn loop_detected_tuple() -> (u16, &'static str) {
  (508, "The server detected an infinite loop while processing a request. This is often due to circular references or recursive function calls in WebDAV configurations, RFC 5842")
}

pub fn bandwidth_limit_exceeded_tuple() -> (u16, &'static str) {
  (509, "The server's bandwidth limit has been exceeded. This limit is typically set by the administrator and prevents further data transfer until the limit resets, often used by hosting providers to prevent abuse, apache, unofficial, Cpanel")
}

pub fn not_extended_tuple() -> (u16, &'static str) {
  (510, "The server requires further extensions to fulfill the request. This could mean additional client conditions or protocol extensions are necessary before the server can process the request")
}

pub fn network_authentication_required_tuple() -> (u16, &'static str) {
  (511, "The network connection requires authentication before accessing the requested resources. This is often used by captive portals to redirect users to a login page")
}

pub fn unknown_error_tuple() -> (u16, &'static str) {
  (520, "An unspecified error occurred, and the server was unable to provide more details. This is a catch-all for unexpected conditions")
}

pub fn web_server_is_down_tuple() -> (u16, &'static str) {
  (521, "Cloudflare, unofficial is currently unreachable, likely due to downtime or maintenance. This prevents the server from processing the request, and the client should try again later")
}

pub fn connection_timed_out_tuple() -> (u16, &'static str) {
  (522, "The connection to the server timed out before a response could be received. This could be due to network issues or server overload")
}

pub fn origin_is_unreachable_tuple() -> (u16, &'static str) {
  (523, "The origin server could not be contacted. This might be due to network issues or misconfiguration")
}

pub fn timeout_occurred_tuple() -> (u16, &'static str) {
  (524, "The operation timed out while waiting for a response from the server. This could be due to network congestion or server overload")
}

pub fn ssl_handshake_failed_tuple() -> (u16, &'static str) {
  (525, "The SSL/TLS handshake failed, preventing a secure connection from being established. This could be due to certificate issues or network problems")
}

pub fn invalid_ssl_certificate_tuple() -> (u16, &'static str) {
  (526, "The SSL/TLS certificate provided by the server is invalid, expired, or does not match the requested domain. This prevents the secure connection from being established")
}

pub fn railgun_error_tuple() -> (u16, &'static str) {
  (527, "An error occurred in the Railgun service, which accelerates connections between Cloudflare and the origin server. This may indicate a misconfiguration or temporary service unavailability")
}

pub fn site_is_overloaded_tuple() -> (u16, &'static str) {
  (529, "Indicates the Qualys server cannot process the request, likely due to high traffic or resource constraints. This is a Qualys-specific status code, unofficial")
}

pub fn site_is_frozen_tuple() -> (u16, &'static str) {
  (530, "Indicates the Pantheon server has been frozen due to inactivity, preventing further requests from being processed. This is a Pantheon-specific status code, unofficial")
}

pub fn origin_dns_error_tuple() -> (u16, &'static str) {
  (531, "The origin server encountered a DNS resolution error while attempting to process the request. This typically occurs when the domain name cannot be resolved to an IP address, possibly due to a misconfiguration or network issue")
}

pub fn no_site_detected_tuple() -> (u16, &'static str) {
  (561, "This error is specific to certain hosting environments. For AWS, it indicates an HTTP Authentication failure, whereas for Pantheon, it means there is a problem with the site configuration")
}

pub fn network_read_timeout_error_tuple() -> (u16, &'static str) {
  (598, "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network returned read timeouts")
}

pub fn network_connect_timeout_error_tuple() -> (u16, &'static str) {
  (599, "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network timed out")
}

/// The functions returns a tuple containing a status code and a JSON value with status and description fields.
pub fn gateway_timeout() -> (u16, serde_json::Value) {
  let (code, desc) = gateway_timeout_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn http_version_not_supported() -> (u16, serde_json::Value) {
  let (code, desc) = http_version_not_supported_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn variant_also_negotiates() -> (u16, serde_json::Value) {
  let (code, desc) = variant_also_negotiates_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn insufficient_storage() -> (u16, serde_json::Value) {
  let (code, desc) = insufficient_storage_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn loop_detected() -> (u16, serde_json::Value) {
  let (code, desc) = loop_detected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn bandwidth_limit_exceeded() -> (u16, serde_json::Value) {
  let (code, desc) = bandwidth_limit_exceeded_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn not_extended() -> (u16, serde_json::Value) {
  let (code, desc) = not_extended_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn network_authentication_required() -> (u16, serde_json::Value) {
  let (code, desc) = network_authentication_required_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn unknown_error() -> (u16, serde_json::Value) {
  let (code, desc) = unknown_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn web_server_is_down() -> (u16, serde_json::Value) {
  let (code, desc) = web_server_is_down_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn connection_timed_out() -> (u16, serde_json::Value) {
  let (code, desc) = connection_timed_out_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn origin_is_unreachable() -> (u16, serde_json::Value) {
  let (code, desc) = origin_is_unreachable_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn timeout_occurred() -> (u16, serde_json::Value) {
  let (code, desc) = timeout_occurred_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn ssl_handshake_failed() -> (u16, serde_json::Value) {
  let (code, desc) = ssl_handshake_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn invalid_ssl_certificate() -> (u16, serde_json::Value) {
  let (code, desc) = invalid_ssl_certificate_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn railgun_error() -> (u16, serde_json::Value) {
  let (code, desc) = railgun_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn site_is_overloaded() -> (u16, serde_json::Value) {
  let (code, desc) = site_is_overloaded_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn site_is_frozen() -> (u16, serde_json::Value) {
  let (code, desc) = site_is_frozen_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn origin_dns_error() -> (u16, serde_json::Value) {
  let (code, desc) = origin_dns_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn no_site_detected() -> (u16, serde_json::Value) {
  let (code, desc) = no_site_detected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn network_read_timeout_error() -> (u16, serde_json::Value) {
  let (code, desc) = network_read_timeout_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn network_connect_timeout_error() -> (u16, serde_json::Value) {
  let (code, desc) = network_connect_timeout_error_tuple();
  (code, json!({ "status": code, "description": desc }))
}

// Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_internal_server_error() {
    let response = ResponsesServerCodes::InternalServerError;
    let (code, description) = response.into();
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
    assert_eq!(bad_gateway_tuple(), (502, "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured"));
  }

  #[test]
  fn test_from_u16_service_unavailable() {
    let response = ResponsesServerCodes::from_u16(503);
    assert_eq!(response, Some(ResponsesServerCodes::ServiceUnavailable));
  }

  #[test]
  fn test_gateway_timeout() {
    let (code, response) = gateway_timeout_tuple();
    assert_eq!(code, 504);
    assert_eq!(response, "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. This could be due to network congestion or the upstream server being overloaded");
  }

  #[test]
  fn test_http_version_not_supported() {
    let (code, response) = http_version_not_supported();
    assert_eq!(code, 505);
    assert_eq!(
      response,
      json!({ "status": 505, "description": "The server does not support the HTTP protocol version used in the request. This prevents the server from processing the request" })
    );
  }
}
