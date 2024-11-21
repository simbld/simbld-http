use crate::helpers::to_u16_helper::ToU16;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{EnumIter, EnumProperty};

#[derive(IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
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

impl ToU16 for ResponsesServerCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}
