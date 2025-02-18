use crate::generate_responses_functions;

generate_responses_functions! {
  "",
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

#[cfg(test)]
mod tests {
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesServerCodes;
    use serde_json::json;
    
    #[test]
    fn test_server_codes_to_u16() {
        assert_eq!(ResponsesServerCodes::InternalServerError.to_u16(), 500);
        assert_eq!(ResponsesServerCodes::NotImplemented.to_u16(), 501);
        assert_eq!(ResponsesServerCodes::BadGateway.to_u16(), 502);
        assert_eq!(ResponsesServerCodes::ServiceUnavailable.to_u16(), 503);
    }

    #[test]
    fn test_server_codes_from_u16() {
        assert_eq!(ResponsesServerCodes::from_u16(501), Some(ResponsesServerCodes::NotImplemented));
        assert_eq!(ResponsesServerCodes::from_u16(502), Some(ResponsesServerCodes::BadGateway));
        assert_eq!(
            ResponsesServerCodes::from_u16(503),
            Some(ResponsesServerCodes::ServiceUnavailable)
        );
        assert_eq!(ResponsesServerCodes::from_u16(9999), None);
    }

    #[test]
    fn test_no_site_detected_codes_as_tuple() {
        let code = ResponsesServerCodes::NoSiteDetected;
        let tuple = UnifiedTuple {
            code: 500,
            name: "Internal Server Error",
            description: "This error is specific to certain hosting environments. For AWS, it indicates an HTTP Authentication failure, whereas for Pantheon, it means there is a problem with the site configuration, no site detected / AWS or Pantheon config error.",
            internal_code: Some(561),
            internal_name: Option::from("No Site Detected"),
        };
        let code_as_tuple = code.as_tuple();
        assert_eq!(code_as_tuple, tuple);
    }

    #[test]
    fn test_web_server_is_down_codes_as_json() {
        let response_code = ResponsesServerCodes::WebServerIsDown;
        let json_result = response_code.as_json();
        let expected_json = json!({
            "standard_http_code": {
                "code": 502,
                "name": "Bad Gateway"
            },
            "internal_http_code": {
                "code": 521,
                "name": "Web Server Is Down"
            },
            "description": "Cloudflare, unofficial is currently unreachable, likely due to downtime or maintenance. This prevents the server from processing the request, and the client should try again later"
        });
        assert_eq!(
            serde_json::to_string(&json_result).unwrap(),
            serde_json::to_string(&expected_json).unwrap()
        );
    }

    #[test]
    fn test_server_codes_into_tuple() {
        let (std_code, std_name): (u16, &'static str) = ResponsesServerCodes::GatewayTimeout.into();
        assert_eq!(std_code, 504);
        assert_eq!(std_name, "Gateway Timeout");
    }

    #[test]
    fn test_bad_gateway_duplicate_standard_codes() {
        // These two codes have the same standard HTTP code (400) but different internal codes
        assert_eq!(
            ResponsesServerCodes::from_u16(521),
            Some(ResponsesServerCodes::WebServerIsDown)
        );
        assert_eq!(
            ResponsesServerCodes::from_u16(523),
            Some(ResponsesServerCodes::OriginIsUnreachable)
        );
    }
}
