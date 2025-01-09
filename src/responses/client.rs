use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use crate::utils::json_formatter::JsonFormatter;
use crate::utils::response_tuple::ResponseTuple;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::Value;
use strum_macros::{Display, EnumIter, EnumProperty};
/// Enum representing HTTP response status codes and descriptions.
/// Each variant corresponds to a specific HTTP status code.
///
/// Example usage:
/// ```
/// let response = ResponsesClientCodes::BadRequest;
/// let json = response.to_detailed_json();
/// println!("{:?}", json);
/// ```

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
#[repr(u16)]
pub enum ResponsesClientCodes {
  BadRequest,
  Unauthorized,
  PaymentRequired,
  Forbidden,
  NotFound,
  MethodNotAllowed,
  NotAcceptable,
  ProxyAuthenticationRequired,
  RequestTimeout,
  Conflict,
  Gone,
  LengthRequired,
  PreconditionFailed,
  ContentTooLarge,
  URITooLong,
  UnsupportedMediaType,
  RangeNotSatisfiable,
  ExpectationFailed,
  ImATeapot,
  PageExpired,
  MethodFailure,
  MisdirectedRequest,
  UnprocessableEntity,
  Locked,
  FailedDependency,
  TooEarly,
  UpgradeRequired,
  PreconditionRequired,
  TooManyRequests,
  RequestHeaderFieldsTooLarge,
  LoginRequired,
  OriginError,
  DestinationError,
  TooLarge,
  SSLCertificateError,
  SSLCertificateRequired,
  NoCertificate,
  LoginTimeout,
  OverDataQuota,
  NoResponse,
  RetryWith,
  BlockedByWindowsParentalControls,
  UnavailableForLegalReasons,
  TooManyRecipients,
  MethodNotValidInThisState,
  UnrecoverableError,
  ClientClosedConnexionPrematurely,
  TooManyForwardedIPAddresses,
  InternetSecurityError,
  RequestHeaderTooLarge,
  CertError,
  NoCert,
  HTTPToHTTPS,
  InvalidToken,
  ClientClosedRequest,
}

/// The above Rust code is implementing a method `as_tuple` for a struct `ResponsesClientCodes`. This method returns a tuple containing information related to HTTP response codes. It extracts the standard code, standard name, internal code (if available), internal name (if available), and description from the struct instance. It then constructs a tuple with this information and returns it.
impl ResponsesClientCodes {
  pub fn as_tuple(&self) -> ResponseTuple {
    match self {
      ResponsesClientCodes::BadRequest => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(400),
        int_name: Some("Bad Request"),
        desc: "The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).",
      },
      ResponsesClientCodes::Unauthorized => ResponseTuple {
        std_code: 401,
        std_name: "Unauthorized",
        int_code: Some(401),
        int_name: Some("Unauthorized"),
        desc: "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.",
      },
      ResponsesClientCodes::PaymentRequired => ResponseTuple {
        std_code: 402,
        std_name: "Payment Required",
        int_code: Some(402),
        int_name: Some("Payment Required"),
        desc: "The initial purpose of this code was for digital payment systems, however this status code is rarely used and no standard convention exists.",
      },
      ResponsesClientCodes::Forbidden => ResponseTuple {
        std_code: 403,
        std_name: "Forbidden",
        int_code: Some(403),
        int_name: Some("Forbidden"),
        desc: "The client does not have access rights to the content; that is, it is unauthorized, so the server is refusing to give the requested resource. Unlike 401 Unauthorized, the client's identity is known to the server.",
      },
      ResponsesClientCodes::NotFound => ResponseTuple {
        std_code: 404,
        std_name: "Not Found",
        int_code: Some(404),
        int_name: Some("Not Found"),
        desc: "The server cannot find the requested resource. In the browser, this means the URL is not recognized. In an API, this can also mean that the endpoint is valid but the resource itself does not exist. Servers may also send this response instead of 403 Forbidden to hide the existence of a resource from an unauthorized client.",
      },
      ResponsesClientCodes::MethodNotAllowed => ResponseTuple {
        std_code: 405,
        std_name: "Method Not Allowed",
        int_code: Some(405),
        int_name: Some("Method Not Allowed"),
        desc: "The HTTP method is not supported for the target resource",
      },
      ResponsesClientCodes::NotAcceptable => ResponseTuple {
        std_code: 406,
        std_name: "Not Acceptable",
        int_code: Some(406),
        int_name: Some("Not Acceptable"),
        desc: "This response is sent when the web server, after performing server-driven content negotiation, doesn't find any content that conforms to the criteria given by the user agent.",
      },
      ResponsesClientCodes::ProxyAuthenticationRequired => ResponseTuple {
        std_code: 407,
        std_name: "Proxy Authentication Required",
        int_code: Some(407),
        int_name: Some("Proxy Authentication Required"),
        desc: "The client must authenticate with a proxy first",
      },
      ResponsesClientCodes::RequestTimeout => ResponseTuple {
        std_code: 408,
        std_name: "Request Timeout",
        int_code: Some(408),
        int_name: Some("Request Timeout"),
        desc: "This response is sent on an idle connection by some servers, even without any previous request by the client. It means that the server would like to shut down this unused connection. This response is used much more since some browsers use HTTP pre-connection mechanisms to speed up browsing. Some servers may shut down a connection without sending this message.",
      },
      ResponsesClientCodes::Conflict => ResponseTuple {
        std_code: 409,
        std_name: "Conflict",
        int_code: Some(409),
        int_name: Some("Conflict"),
        desc: "This response is sent when a request conflicts with the current state of the server. In WebDAV remote web authoring, 409 responses are errors sent to the client so that a user might be able to resolve a conflict and resubmit the request.",
      },
      ResponsesClientCodes::Gone => ResponseTuple {
        std_code: 410,
        std_name: "Gone",
        int_code: Some(410),
        int_name: Some("Gone"),
        desc: "This response is sent when the requested content has been permanently deleted from server, with no forwarding address. Clients are expected to remove their caches and links to the resource. The HTTP specification intends this status code to be used for 'limited-time, promotional services'. APIs should not feel compelled to indicate resources that have been deleted with this status code.",
      },
      ResponsesClientCodes::LengthRequired => ResponseTuple {
        std_code: 411,
        std_name: "Length Required",
        int_code: Some(411),
        int_name: Some("Length Required"),
        desc: "Server rejected the request because the Content-Length header field is not defined and the server requires it.",
      },
      ResponsesClientCodes::PreconditionFailed => ResponseTuple {
        std_code: 412,
        std_name: "Precondition Failed",
        int_code: Some(412),
        int_name: Some("Precondition Failed"),
        desc: "In conditional requests, the client has indicated preconditions in its headers which the server does not meet.",
      },
      ResponsesClientCodes::ContentTooLarge => ResponseTuple {
        std_code: 413,
        std_name: "Content Too Large",
        int_code: Some(413),
        int_name: Some("Content Too Large"),
        desc: "The request body is larger than limits defined by server. The server might close the connection or return an Retry-After header field.",
      },
      ResponsesClientCodes::URITooLong => ResponseTuple {
        std_code: 414,
        std_name: "URI Too Long",
        int_code: Some(414),
        int_name: Some("URI Too Long"),
        desc: "The URI requested by the client is longer than the server is willing to interpret.",
      },
      ResponsesClientCodes::UnsupportedMediaType => ResponseTuple {
        std_code: 415,
        std_name: "Unsupported Media Type",
        int_code: Some(415),
        int_name: Some("Unsupported Media Type"),
        desc: "The media format of the requested data is not supported by the server, so the server is rejecting the request.",
      },
      ResponsesClientCodes::RangeNotSatisfiable => ResponseTuple {
        std_code: 416,
        std_name: "Range Not Satisfiable",
        int_code: Some(416),
        int_name: Some("Range Not Satisfiable"),
        desc: "The range specified by the request's Range header field cannot be satisfied; the range may exceed the size of the data coming from the targeted URI.",
      },
      ResponsesClientCodes::ExpectationFailed => ResponseTuple {
        std_code: 417,
        std_name: "Expectation Failed",
        int_code: Some(417),
        int_name: Some("Expectation Failed"),
        desc: "This response code means that the expectations indicated by the Expect request header field could not be met by the server.",
      },
      ResponsesClientCodes::ImATeapot => ResponseTuple {
        std_code: 418,
        std_name: "I'm a teapot",
        int_code: Some(418),
        int_name: Some("I'm a teapot"),
        desc: "The waiter refuses to brew coffee with a teapot, RFC 2324.",
      },
      ResponsesClientCodes::PageExpired => ResponseTuple {
        std_code: 401,
        std_name: "Unauthorized",
        int_code: Some(419),
        int_name: Some("PageExpired"),
        desc: "Used by Laravel for missing/expired CSRF token, unofficial",
      },
      ResponsesClientCodes::MethodFailure => ResponseTuple {
        std_code: 405,
        std_name: "Method Not Allowed",
        int_code: Some(420),
        int_name: Some("MethodFailure"),
        desc: "Method known by server but cannot be processed, unofficial",
      },
      ResponsesClientCodes::MisdirectedRequest => ResponseTuple {
        std_code: 421,
        std_name: "Misdirected Request",
        int_code: Some(421),
        int_name: Some("Misdirected Request"),
        desc: "The request was sent to a server unable to produce a response. This code may be sent by a server that has not been configured to produce responses subject to the combination of schemas and identities included in the request URI.",
      },
      ResponsesClientCodes::UnprocessableEntity => ResponseTuple {
        std_code: 422,
        std_name: "Unprocessable Entity",
        int_code: Some(422),
        int_name: Some("Unprocessable Entity"),
        desc: "The request was successfully created but could not be processed due to semantic errors, WebDAV RFC 4918.",
      },
      ResponsesClientCodes::Locked => ResponseTuple {
        std_code: 423,
        std_name: "Locked",
        int_code: Some(423),
        int_name: Some("Locked"),
        desc: "The resource that is currently being viewed is locked.",
      },
      ResponsesClientCodes::FailedDependency => ResponseTuple {
        std_code: 424,
        std_name: "Failed Dependency",
        int_code: Some(424),
        int_name: Some("Failed Dependency"),
        desc: "The query failed because a previous query failed.",
      },
      ResponsesClientCodes::TooEarly => ResponseTuple {
        std_code: 425,
        std_name: "Too Early",
        int_code: Some(425),
        int_name: Some("Too Early"),
        desc: "Indicate that the server does not want to process a request that could be replayed.",
      },
      ResponsesClientCodes::UpgradeRequired => ResponseTuple {
        std_code: 426,
        std_name: "Upgrade Required",
        int_code: Some(426),
        int_name: Some("Upgrade Required"),
        desc: "The server refuses to process the request using the current protocol but may agree to do so if the client opts for another protocol. The server must send an Upgrade header in the 426 response to indicate the requested protocol(s) (Section 6.7 of [RFC7230]).",
      },
      ResponsesClientCodes::PreconditionRequired => ResponseTuple {
        std_code: 428,
        std_name: "Precondition Required",
        int_code: Some(428),
        int_name: Some("Precondition Required"),
        desc: "The origin server requires the request to be conditional. This is intended to prevent the 'loss of update' problem, where a client retrieves the state of a resource with GET, modifies it, and returns it to the server with PUT while a third party modifies the state of the resource. server, which leads to a conflict.",
      },
      ResponsesClientCodes::TooManyRequests => ResponseTuple {
        std_code: 429,
        std_name: "Too Many Requests",
        int_code: Some(429),
        int_name: Some("Too Many Requests"),
        desc: "The user has sent too many requests in a given amount of time (rate limiting).",
      },
      ResponsesClientCodes::RequestHeaderFieldsTooLarge => ResponseTuple {
        std_code: 431,
        std_name: "Request Header Fields Too Large",
        int_code: Some(431),
        int_name: Some("Request Header Fields Too Large"),
        desc: "The server is unwilling to process the request because the header fields are too long. The request can be returned after reducing the size of the headers.",
      },
      ResponsesClientCodes::LoginRequired => ResponseTuple {
        std_code: 401,
        std_name: "Unauthorized",
        int_code: Some(432),
        int_name: Some("Login Required"),
        desc: "Authentication is required to access the resource",
      },
      ResponsesClientCodes::OriginError => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(433),
        int_name: Some("Origin Error"),
        desc: "The request was rejected due to an origin server/client IP issue",
      },
      ResponsesClientCodes::DestinationError => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(434),
        int_name: Some("DestinationError"),
        desc: "The request was rejected due to a destination server/config issue",
      },
      ResponsesClientCodes::TooLarge => ResponseTuple {
        std_code: 413,
        std_name: "Payload Too Large",
        int_code: Some(435),
        int_name: Some("TooLarge"),
        desc: "The request or resource is too large for the server to handle",
      },
      ResponsesClientCodes::SSLCertificateError => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(436),
        int_name: Some("SSLCertificateError"),
        desc: "An invalid or untrusted SSL certificate was encountered",
      },
      ResponsesClientCodes::SSLCertificateRequired => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(437),
        int_name: Some("SSLCertificateRequired"),
        desc: "The server requires a valid SSL certificate for secure connections",
      },
      ResponsesClientCodes::NoCertificate => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(438),
        int_name: Some("NoCertificate"),
        desc: "No SSL certificate was provided by the client",
      },
      ResponsesClientCodes::LoginTimeout => ResponseTuple {
        std_code: 401,
        std_name: "Unauthorized",
        int_code: Some(440),
        int_name: Some("LoginTimeout"),
        desc: "The client session timed out; must log in again, unofficial (IIS)",
      },
      ResponsesClientCodes::OverDataQuota => ResponseTuple {
        std_code: 413,
        std_name: "Payload Too Large",
        int_code: Some(441),
        int_name: Some("OverDataQuota"),
        desc: "The client exceeded the allocated data quota",
      },
      ResponsesClientCodes::NoResponse => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(444),
        int_name: Some("NoResponse"),
        desc: "The server closed the connection without sending a response, unofficial (Nginx)",
      },
      ResponsesClientCodes::RetryWith => ResponseTuple {
        std_code: 428,
        std_name: "Precondition Required",
        int_code: Some(449),
        int_name: Some("RetryWith"),
        desc: "The user has not provided the required information, unofficial (IIS)",
      },
      ResponsesClientCodes::BlockedByWindowsParentalControls => ResponseTuple {
        std_code: 403,
        std_name: "Forbidden",
        int_code: Some(450),
        int_name: Some("BlockedByWindowsParentalControls"),
        desc: "Resource blocked by Windows Parental Controls, unofficial",
      },
      ResponsesClientCodes::UnavailableForLegalReasons => ResponseTuple {
        std_code: 451,
        std_name: "Unavailable For Legal Reasons",
        int_code: Some(451),
        int_name: Some("UnavailableForLegalReasons"),
        desc: "Access denied for legal reasons (censorship, local laws)",
      },
      ResponsesClientCodes::TooManyRecipients => ResponseTuple {
        std_code: 429,
        std_name: "Too Many Requests",
        int_code: Some(452),
        int_name: Some("TooManyRecipients"),
        desc: "Unable to process the request because it contains too many recipients",
      },
      ResponsesClientCodes::MethodNotValidInThisState => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(455),
        int_name: Some("MethodNotValidInThisState"),
        desc: "The specified method is not valid for the current resource state",
      },
      ResponsesClientCodes::UnrecoverableError => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(456),
        int_name: Some("UnrecoverableError"),
        desc: "A critical server error prevents continuing processing",
      },
      ResponsesClientCodes::ClientClosedConnexionPrematurely => ResponseTuple {
        std_code: 408,
        std_name: "Request Timeout",
        int_code: Some(460),
        int_name: Some("ClientClosedConnexionPrematurely"),
        desc: "Client closed the connection prematurely, often due to timeout or interruption",
      },
      ResponsesClientCodes::TooManyForwardedIPAddresses => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(463),
        int_name: Some("TooManyForwardedIPAddresses"),
        desc: "Excessive forwarded IP addresses in the headers, possible misconfiguration",
      },
      ResponsesClientCodes::InternetSecurityError => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(467),
        int_name: Some("InternetSecurityError"),
        desc: "A violation or misconfiguration in internet security policies occurred",
      },
      ResponsesClientCodes::RequestHeaderTooLarge => ResponseTuple {
        std_code: 431,
        std_name: "Request Header Fields Too Large",
        int_code: Some(494),
        int_name: Some("RequestHeaderTooLarge"),
        desc: "Headers too large to process, unofficial (Nginx)",
      },
      ResponsesClientCodes::CertError => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(495),
        int_name: Some("CertError"),
        desc: "SSL cert from client invalid or cannot be verified, unofficial (Nginx)",
      },
      ResponsesClientCodes::NoCert => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(496),
        int_name: Some("NoCert"),
        desc: "A required client certificate was not provided, unofficial (Nginx)",
      },
      ResponsesClientCodes::HTTPToHTTPS => ResponseTuple {
        std_code: 400,
        std_name: "Bad Request",
        int_code: Some(497),
        int_name: Some("HTTPToHTTPS"),
        desc: "Client sent HTTP to server requiring HTTPS, unofficial (Nginx)",
      },
      ResponsesClientCodes::InvalidToken => ResponseTuple {
        std_code: 401,
        std_name: "Unauthorized",
        int_code: Some(498),
        int_name: Some("Invalid Token"),
        desc: "Invalid, expired, or malformed token, unofficial (ArcGIS)",
      },
      ResponsesClientCodes::ClientClosedRequest => ResponseTuple {
        std_code: 408,
        std_name: "Request Timeout",
        int_code: Some(499),
        int_name: Some("ClientClosedRequest"),
        desc: "Client closed the connection before server response, unofficial (Nginx)",
      },
    }
  }
}

/// Delegates to `as_tuple` to generate a detailed JSON response.

pub fn as_json(&self) -> Value {
  self.as_tuple().to_json_response()
}

// -------------------------
// Impl "ToU16" (custom trait) - must be at the **same level** as the main impl,
// not inside another impl
// -------------------------
impl ToU16 for ResponsesClientCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion `Into<u16>` (provided by num_enum::IntoPrimitive)
  }
}

// -------------------------
// Impl "FromU16" (custom trait) - same, outside the main impl
// -------------------------
impl FromU16 for ResponsesClientCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok()
  }
}

// -------------------------
// Impl "Into<(u16, &'static str)>" - same
// -------------------------
impl Into<(u16, &'static str)> for ResponsesClientCodes {
  fn into(self) -> (u16, &'static str) {
    let std_code: u16 = self.to_u16();
    // Here, we use `to_string()` which comes from `Display` => "BadRequest", "Unauthorized", ...
    let std_name = self.to_string();
    (std_code, std_name)
  }
}

// Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_as_json_not_found() {
    let response = ResponsesClientCodes::NotFound;
    let json_result = response.as_json();
    let expected_json = serde_json::json!({
        "standard http code": {
            "code": 404,
            "name": "Not Found"
        },
        "internal http code": {
            "code": 4041,
            "name": "Custom Not Found"
        },
        "description": "The requested resource could not be found."
    });

    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_generated_function_bad_request() {
    let response = ResponsesClientCodes::BadRequest;
    let (code, description) = response.into();
    assert_eq!(code, 400);
    assert_eq!(description, "Bad Request");
  }

  #[test]
  fn test_to_u16_unauthorized() {
    let response = ResponsesClientCodes::Unauthorized;
    let code = response.to_u16();
    assert_eq!(code, 401);
  }

  #[test]
  fn test_from_u16_not_found() {
    let response = ResponsesClientCodes::from_u16(404);
    assert_eq!(response, Some(ResponsesClientCodes::NotFound));
  }
}
