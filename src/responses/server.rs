pub enum ResponsesServerCodes {
  InternalServerError = 500,
  NotImplemented = 501,
  BadGateway = 502, // WARNING: can affect the rate at which Googlebot explanation :: https://http.dev/502
  ServiceUnavailable = 503,
  GatewayTimeout = 504,
  HTTPVersionNotSupported = 505,
  VariantAlsoNegotiates = 506,
  InsufficientStorage = 507,
  LoopDetected = 508,
  BandwidthLimitExceeded = 509,
  NotExtended = 510,
  NetworkAuthenticationRequired = 511,
  UnknownError = 520,
  WebServerIsDown = 521,
  ConnectionTimedOut = 522,
  OriginIsUnreachable = 523,
  TimeoutOccurred = 524,
  SSLHandshakeFailed = 525,
  InvalidSSLCertificate = 526,
  RailgunError = 527,
  SiteIsOverloaded = 529,
  SiteIsFrozen = 530,
  OriginDNSError = 530,
  NoSiteDetected = 561,
  NetworkReadTimeoutError = 598,
  NetworkConnectTimeoutError = 599,
}

impl ResponsesServerCodes {
  pub fn to_u16(&self) -> u16 {
    *self as u16
  }

  pub fn description(&self) -> &'static str {
    match self {
      ResponsesServerCodes::InternalServerError => "The server encountered an unexpected condition that prevented it from fulfilling the request. This could be due to a misconfiguration, an unhandled exception, or resource exhaustion",
      ResponsesServerCodes::NotImplemented => "The server does not support the functionality required to fulfill the request. This might be because the server does not recognize the request method or lacks the capability to process it.",
      ResponsesServerCodes::BadGateway => "The server, while acting as a gateway or proxy, received an invalid response from an upstream server. This could be due to the upstream server being down or misconfigured",
      ResponsesServerCodes::ServiceUnavailable => "The server is currently unable to handle the request due to temporary overloading or maintenance. This is usually a temporary state.",
      ResponsesServerCodes::GatewayTimeout => "The server, while acting as a gateway or proxy, did not receive a timely response from the upstream server. This could be due to network congestion or the upstream server being overloaded",
      ResponsesServerCodes::HTTPVersionNotSupported => "The server does not support the HTTP protocol version used in the request. This prevents the server from processing the request",
      ResponsesServerCodes::VariantAlsoNegotiates => "The server encountered a configuration error in transparent content negotiation. This resulted in a circular reference that prevents the server from serving the requested content",
      ResponsesServerCodes::InsufficientStorage => "The server is unable to store the representation needed to complete the request. This could be due to storage limits being reached or allocation constraints",
      ResponsesServerCodes::LoopDetected => "The server detected an infinite loop while processing a request. This is often due to circular references or recursive function calls in WebDAV configurations, RFC 5842",
      ResponsesServerCodes::BandwidthLimitExceeded => "The server's bandwidth limit has been exceeded. This limit is typically set by the administrator and prevents further data transfer until the limit resets, often used by hosting providers to prevent abuse, apache, unofficial, Cpanel",
      ResponsesServerCodes::NotExtended => "The server requires further extensions to fulfill the request. This could mean additional client conditions or protocol extensions are necessary.",
      ResponsesServerCodes::NetworkAuthenticationRequired => "The network connection requires authentication before accessing the requested resources. This is often used by captive portals to redirect users to a login page",
      ResponsesServerCodes::UnknownError => "An unspecified error occurred, and the server was unable to provide more details. This is a catch-all for unexpected conditions",
      ResponsesServerCodes::WebServerIsDown => "Cloudflare, unofficial is currently unreachable, likely due to downtime or maintenance. This prevents the server from processing the request, ",
      ResponsesServerCodes::ConnectionTimedOut => "The connection to the server timed out before a response could be received. This could be due to network issues or server overload",
      ResponsesServerCodes::OriginIsUnreachable => "The origin server could not be contacted. This might be due to network issues or misconfiguration",
      ResponsesServerCodes::TimeoutOccurred => "The operation timed out while waiting for a response from the server. This could be due to network congestion or server overload",
      ResponsesServerCodes::SSLHandshakeFailed => "The SSL/TLS handshake failed, preventing a secure connection from being established. This could be due to certificate issues or network problems",
      ResponsesServerCodes::InvalidSSLCertificate => "The SSL/TLS certificate provided by the server is invalid, expired, or does not match the requested domain. This prevents the secure connection from being established",
      ResponsesServerCodes::RailgunError => "An error occurred in the Railgun service, which accelerates connections between Cloudflare and the origin server. This may indicate a misconfiguration or temporary service unavailability",
      ResponsesServerCodes::SiteIsOverloaded => "Indicates the Qualys server cannot process the request, likely due to high traffic or resource constraints. This is a Qualys-specific status code, unofficial",
      ResponsesServerCodes::SiteIsFrozen => "Indicates the Pantheon server has been frozen due to inactivity, preventing further requests from being processed. This is a Pantheon-specific status code, unofficial",
      ResponsesServerCodes::OriginDNSError => "The origin server encountered a DNS resolution error while attempting to process the request. This typically occurs when the domain name cannot be resolved to an IP address, possibly due to a misconfiguration or network issue",
      ResponsesServerCodes::NoSiteDetected => "This error is specific to certain hosting environments. For AWS, it indicates an HTTP Authentication failure, whereas for Pantheon, it means there is a problem with the site configuration",
      ResponsesServerCodes::NetworkReadTimeoutError => "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network returned read timeouts",
      ResponsesServerCodes::NetworkConnectTimeoutError => "This unofficial status code indicates that the HTTP requests executed by the code failed because no local network was found or the HTTP connections to the local network timed out",
    }
  }
}
