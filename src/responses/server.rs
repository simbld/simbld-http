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
    (code, description)
  }
}

/// Functions return raw data as a tuple for further processing or formats containing HTTP status code, status message and description of various client error responses.
pub fn internal_server_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::InternalServerError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Internal Server Error", description)
}

pub fn not_implemented_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::NotImplemented;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Not Implemented", description)
}

pub fn bad_gateway_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::BadGateway;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Bad Gateway", description)
}

pub fn service_unavailable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::ServiceUnavailable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Service Unavailable", description)
}

pub fn gateway_timeout_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::GatewayTimeout;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Gateway Timeout", description)
}

pub fn http_version_not_supported_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::HTTPVersionNotSupported;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "HTTP Version Not Supported", description)
}

pub fn variant_also_negotiates_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::VariantAlsoNegotiates;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Variant Also Negotiates", description)
}

pub fn insufficient_storage_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::InsufficientStorage;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Insufficient Storage", description)
}

pub fn loop_detected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::LoopDetected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Loop Detected", description)
}

pub fn bandwidth_limit_exceeded_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::BandwidthLimitExceeded;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Bandwidth Limit Exceeded", description)
}

pub fn not_extended_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::NotExtended;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Not Extended", description)
}

pub fn network_authentication_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::NetworkAuthenticationRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Network Authentication Required", description)
}

pub fn unknown_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::UnknownError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unknown Error", description)
}

pub fn web_server_is_down_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::WebServerIsDown;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Web Server Is Down", description)
}

pub fn connection_timed_out_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::ConnectionTimedOut;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Connection Timed Out", description)
}

pub fn origin_is_unreachable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::OriginIsUnreachable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Origin Is Unreachable", description)
}

pub fn timeout_occurred_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::TimeoutOccurred;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Timeout Occurred", description)
}

pub fn ssl_handshake_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::SSLHandshakeFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "SSL Handshake Failed", description)
}

pub fn invalid_ssl_certificate_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::InvalidSSLCertificate;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid SSL Certificate", description)
}

pub fn railgun_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::RailgunError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Railgun Error", description)
}

pub fn site_is_overloaded_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::SiteIsOverloaded;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Site Is Overloaded", description)
}

pub fn site_is_frozen_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::SiteIsFrozen;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Site Is Frozen", description)
}

pub fn origin_dns_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::OriginDNSError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Origin DNS Error", description)
}

pub fn no_site_detected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::NoSiteDetected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "No Site Detected", description)
}

pub fn network_read_timeout_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::NetworkReadTimeoutError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Network Read Timeout Error", description)
}

pub fn network_connect_timeout_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesServerCodes::NetworkConnectTimeoutError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Network Connect Timeout Error", description)
}

/// Functions return formatted data as JSON containing HTTP status code, status message and description of various informational responses.
pub fn internal_server_error() -> serde_json::Value {
  let (code, name, desc) = internal_server_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn not_implemented() -> serde_json::Value {
  let (code, name, desc) = not_implemented_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn bad_gateway() -> serde_json::Value {
  let (code, name, desc) = bad_gateway_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn service_unavailable() -> serde_json::Value {
  let (code, name, desc) = service_unavailable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn gateway_timeout() -> serde_json::Value {
  let (code, name, desc) = gateway_timeout_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn http_version_not_supported() -> serde_json::Value {
  let (code, name, desc) = http_version_not_supported_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn variant_also_negotiates() -> serde_json::Value {
  let (code, name, desc) = variant_also_negotiates_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn insufficient_storage() -> serde_json::Value {
  let (code, name, desc) = insufficient_storage_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn loop_detected() -> serde_json::Value {
  let (code, name, desc) = loop_detected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn bandwidth_limit_exceeded() -> serde_json::Value {
  let (code, name, desc) = bandwidth_limit_exceeded_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn not_extended() -> serde_json::Value {
  let (code, name, desc) = not_extended_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn network_authentication_required() -> serde_json::Value {
  let (code, name, desc) = network_authentication_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unknown_error() -> serde_json::Value {
  let (code, name, desc) = unknown_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn web_server_is_down() -> serde_json::Value {
  let (code, name, desc) = web_server_is_down_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn connection_timed_out() -> serde_json::Value {
  let (code, name, desc) = connection_timed_out_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn origin_is_unreachable() -> serde_json::Value {
  let (code, name, desc) = origin_is_unreachable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn timeout_occurred() -> serde_json::Value {
  let (code, name, desc) = timeout_occurred_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn ssl_handshake_failed() -> serde_json::Value {
  let (code, name, desc) = ssl_handshake_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_ssl_certificate() -> serde_json::Value {
  let (code, name, desc) = invalid_ssl_certificate_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn railgun_error() -> serde_json::Value {
  let (code, name, desc) = railgun_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn site_is_overloaded() -> serde_json::Value {
  let (code, name, desc) = site_is_overloaded_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn site_is_frozen() -> serde_json::Value {
  let (code, name, desc) = site_is_frozen_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn origin_dns_error() -> serde_json::Value {
  let (code, name, desc) = origin_dns_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn no_site_detected() -> serde_json::Value {
  let (code, name, desc) = no_site_detected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn network_read_timeout_error() -> serde_json::Value {
  let (code, name, desc) = network_read_timeout_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn network_connect_timeout_error() -> serde_json::Value {
  let (code, name, desc) = network_connect_timeout_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
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
