use crate::helpers::generate_responses_functions::generate_responses_functions;
use serde_json::json;
/// Enum representing HTTP response status codes and descriptions.
/// Each variant corresponds to a specific HTTP status code.
///
/// * Example:
/// ```rust
///
/// use simbld_http::responses::server::ResponsesServerCodes::InternalServerError;
///
/// let response = InternalServerError;
/// let json = response.as_json();
/// println!("{:?}", json);
/// ```

#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum ResponsesServerCodes {
  InternalServerError,
  NotImplemented,
  BadGateway,
  ServiceUnavailable,
  GatewayTimeout,
  HTTPVersionNotSupported,
  VariantAlsoNegotiates,
  InsufficientStorage,
  LoopDetected,
  BandwidthLimitExceeded,
  NotExtended,
  NetworkAuthenticationRequired,
  UnknownError,
  WebServerIsDown,
  ConnectionTimedOut,
  OriginIsUnreachable,
  TimeoutOccurred,
  SSLHandshakeFailed,
  InvalidSSLCertificate,
  RailgunError,
  SiteIsOverloaded,
  SiteIsFrozen,
  OriginDNSError,
  NoSiteDetected,
  NetworkReadTimeoutError,
  NetworkConnectTimeoutError,
}

generate_responses_functions! {
  InternalServerError => (500, "Internal Server Error", "The server encountered an unexpected condition that prevented it from fulfilling the request. This could be due to a misconfiguration, an unhandled exception, or resource exhaustion", 500, "Internal Server Error", 0, "", "", ""),
  NotImplemented => (501, "Not Implemented", "The server does not support the functionality required to fulfill the request. This might be because the server does not recognize the request method or lacks the capability to process it", 501, "Not Implemented", 0, "", "", ""),
  BadGateway => (502, "Bad Gateway", "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured", 502, "Bad Gateway", 0, "", "", ""),
  ServiceUnavailable => (503, "Service Unavailable", "The server is currently unable to handle the request due to temporary overloading or maintenance. This is usually a temporary state", 503, "Service Unavailable", 0, "", "", ""),
  GatewayTimeout => (504, "Gateway Timeout", "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. This could be due to network congestion or the upstream server being overloaded", 504, "Gateway Timeout", 0, "", "", ""),
  HTTPVersionNotSupported => (505, "HTTP Version Not Supported", "The server does not support the HTTP protocol version used in the request. This prevents the server from processing the request", 505, "HTTP Version Not Supported", 0, "", "", ""),
  VariantAlsoNegotiates => (506, "Variant Also Negotiates", "The server encountered a configuration error in transparent content negotiation. This resulted in a circular reference that prevents the server from serving the requested content", 506, "Variant Also Negotiates", 0, "", "", ""),
  InsufficientStorage => (507, "Insufficient Storage", "The server is unable to store the representation needed to complete the request. This could be due to storage limits being reached or allocation constraints", 507, "Insufficient Storage", 0, "", "", ""),
  LoopDetected => (508, "Loop Detected", "The server detected an infinite loop while processing a request. This is often due to circular references or recursive function calls in WebDAV configurations, RFC 5842", 508, "Loop Detected", 0, "", "", ""),
  BandwidthLimitExceeded => (509, "Bandwidth Limit Exceeded", "The server's bandwidth limit has been exceeded. This limit is typically set by the administrator and prevents further data transfer until the limit resets, often used by hosting providers to prevent abuse, apache, unofficial, Cpanel", 509, "Bandwidth Limit Exceeded", 0, "", "", ""),
  NotExtended => (510, "Not Extended", "The server requires further extensions to fulfill the request. This could mean additional client conditions or protocol extensions are necessary before the server can process the request", 510, "Not Extended", 0, "", "", ""),
  NetworkAuthenticationRequired => (511, "Network Authentication Required", "The network connection requires authentication before accessing the requested resources. This is often used by captive portals to redirect users to a login page", 511, "Network Authentication Required", 0, "", "", ""),
  UnknownError => (500, "Internal Server Error", "An unspecified error occurred, and the server was unable to provide more details. This is a catch-all for unexpected conditions, (Cloudflare extension)", 520, "Unknown Error", 0, "", "", ""),
  WebServerIsDown => (502, "Bad Gateway", "Cloudflare, unofficial is currently unreachable, likely due to downtime or maintenance. This prevents the server from processing the request, and the client should try again later", 521, "Web Server Is Down", 0, "", "", ""),
  ConnectionTimedOut => (504, "Gateway Timeout", "The connection to the server timed out before a response could be received. This could be due to network issues or server overload", 522, "Connection Timed Out", 0, "", "", ""),
  OriginIsUnreachable => (502, "Bad Gateway", "The origin server could not be contacted. This might be due to network issues or misconfiguration", 523, "Origin Is Unreachable", 0, "", "", ""),
  TimeoutOccurred => (504, "Gateway Timeout", "The operation timed out while waiting for a response from the server. This could be due to network congestion or server overload", 524, "Timeout Occurred", 0, "", "", ""),
  SSLHandshakeFailed => (525, "SSL Handshake Failed", "The SSL/TLS handshake failed, preventing a secure connection from being established. This could be due to certificate issues or network problems", 525, "SSL Handshake Failed", 0, "", "", ""),
  InvalidSSLCertificate => (526, "Invalid SSL Certificate", "The SSL/TLS certificate provided by the server is invalid, expired, or does not match the requested domain. This prevents the secure connection from being established", 526, "Invalid SSL Certificate", 0, "", "", ""),
  RailgunError => (527, "Railgun Error", "An error occurred in the Railgun service, which accelerates connections between Cloudflare and the origin server. This may indicate a misconfiguration or temporary service unavailability", 527, "Railgun Error", 0, "", "", ""),
  SiteIsOverloaded => (529, "Site Is Overloaded", "Indicates the Qualys server cannot process the request, likely due to high traffic or resource constraints. This is a Qualys-specific status code, unofficial", 529, "Site Is Overloaded", 0, "", "", ""),
  SiteIsFrozen => (530, "Site Is Frozen", "Indicates the Pantheon server has been frozen due to inactivity, preventing further requests from being processed. This is a Pantheon-specific status code, unofficial", 530, "Site Is Frozen", 0, "", "", ""),
  OriginDNSError => (531, "Origin DNS Error", "The origin server encountered a DNS resolution error while attempting to process the request. This typically occurs when the domain name cannot be resolved to an IP address, possibly due to a misconfiguration or network issue", 531, "Origin DNS Error", 0, "", "", ""),
  NoSiteDetected => (500, "Internal Server Error", "This error is specific to certain hosting environments. For AWS, it indicates an HTTP Authentication failure, whereas for Pantheon, it means there is a problem with the site configuration, no site detected / AWS or Pantheon config error.", 561, "No Site Detected", 0, "", "", ""),
  NetworkReadTimeoutError => (598, "Network Read Timeout Error", "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network returned read timeouts", 598, "Network Read Timeout Error", 0, "", "", ""),
  NetworkConnectTimeoutError => (599, "Network Connect Timeout Error", "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network timed out", 599, "Network Connect Timeout Error", 0, "", "", ""),
}

/// This file defines the `ResponsesServerCodes` enum and provides five main functionalities:
/// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
/// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
/// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
/// 4. `as_json()` - converts the variant to a JSON object.
/// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::ResponsesServerCodes;

  #[test]
  fn test_server_codes_to_u16() {
    let response = ResponsesServerCodes::InternalServerError;
    let code = response.to_u16();
    assert_eq!(code, 500);
  }

  #[test]
  fn test_server_codes_from_u16() {
    let status = ResponsesServerCodes::from_u16(501);
    assert_eq!(status, Some(ResponsesServerCodes::NotImplemented));
  }

  #[test]
  fn test_server_codes_as_tuple() {
    let code = ResponsesServerCodes::BadGateway;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::NineFields(
        502,
        "Bad Gateway",
        "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured",
        502,
        "Bad Gateway",
        110,
        "req-13",
        "user-13",
        "status-13"
      )
    );
  }

  #[test]
  fn test_server_codes_as_json() {
    let code = ResponsesServerCodes::ServiceUnavailable;
    let json_result = code.as_json();
    let expected_json = json!({
        "standard http code": {
            "code": 503,
            "name": "Service Unavailable"
        },
        "internal http code": {
            "code": 503,
            "name": "Service Unavailable"
        },
        "description": "The server is currently unable to handle the request due to temporary overloading or maintenance. This is usually a temporary state",
        "metadata": {
            "meta1": 103,
            "meta2": "req-6",
            "meta3": "user-6",
            "meta4": "status-6"
        }
    });
    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_server_codes_into_tuple() {
    let code = ResponsesServerCodes::GatewayTimeout;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 504);
    assert_eq!(std_name, "Gateway Timeout");
  }
}
