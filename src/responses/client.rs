use crate::helpers::generate_responses_functions::generate_responses_functions;
use serde_json::json;
/// Enum representing HTTP response status codes and descriptions.
/// Each variant corresponds to a specific HTTP status code.
///
/// * Example:
/// ```rust
///
/// use simbld_http::responses::client::ResponsesClientCodes::BadRequest;
///
/// let response = BadRequest;
/// let json = response.as_json();
/// println!("{:?}", json);
/// ```
#[derive(Debug, Clone, PartialEq)]
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

generate_responses_functions! {
  BadRequest => (400, "Bad Request", "The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).", 400, "Bad Request", 0, "", "", ""),
  Unauthorized => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 401, "Unauthorized", 0, "", "", ""),
  PaymentRequired => (402, "Payment Required", "The initial purpose of this code was for digital payment systems, however this status code is rarely used and no standard convention exists.", 402, "Payment Required", 0, "", "", ""),
  Forbidden => (403, "Forbidden", "The client does not have access rights to the content; that is, it is unauthorized, so the server is refusing to give the requested resource. Unlike 401 Unauthorized, the client's identity is known to the server.", 403, "Forbidden", 0, "", "", ""),
  NotFound => (404, "Not Found", "The server cannot find the requested resource. In the browser, this means the URL is not recognized. In an API, this can also mean that the endpoint is valid but the resource itself does not exist. Servers may also send this response instead of 403 Forbidden to hide the existence of a resource from an unauthorized client.", 404, "Not Found", 0, "", "", ""),
  MethodNotAllowed => (405, "Method Not Allowed", "The HTTP method is not supported for the target resource", 405, "Method Not Allowed", 0, "", "", ""),
  NotAcceptable => (406, "Not Acceptable", "This response is sent when the web server, after performing server-driven content negotiation, doesn't find any content that conforms to the criteria given by the user agent.", 406, "Not Acceptable", 0, "", "", ""),
  ProxyAuthenticationRequired => (407, "Proxy Authentication Required", "The client must authenticate with a proxy first", 407, "Proxy Authentication Required", 0, "", "", ""),
  RequestTimeout => (408, "Request Timeout", "This response is sent on an idle connection by some servers, even without any previous request by the client. It means that the server would like to shut down this unused connection. This response is used much more since some browsers use HTTP pre-connection mechanisms to speed up browsing. Some servers may shut down a connection without sending this message.", 408, "Request Timeout", 0, "", "", ""),
  Conflict => (409, "Conflict", "This response is sent when a request conflicts with the current state of the server. In WebDAV remote web authoring, 409 responses are errors sent to the client so that a user might be able to resolve a conflict and resubmit the request.", 409, "Conflict", 0, "", "", ""),
  Gone => (410, "Gone", "This response is sent when the requested content has been permanently deleted from server, with no forwarding address. Clients are expected to remove their caches and links to the resource. The HTTP specification intends this status code to be used for 'limited-time, promotional services'. APIs should not feel compelled to indicate resources that have been deleted with this status code.", 410, "Gone", 0, "", "", ""),
  LengthRequired => (411, "Length Required", "Server rejected the request because the Content-Length header field is not defined and the server requires it.", 411, "Length Required", 0, "", "", ""),
  PreconditionFailed => (412, "Precondition Failed", "In conditional requests, the client has indicated preconditions in its headers which the server does not meet.", 412, "Precondition Failed", 0, "", "", ""),
  ContentTooLarge => (413, "Payload Too Large", "The request or resource is too large for the server to handle", 413, "Content Too Large", 0, "", "", ""),
  URITooLong => (414, "URI Too Long", "The URI requested by the client is longer than the server is willing to interpret.", 414, "URI Too Long", 0, "", "", ""),
  UnsupportedMediaType => (415, "Unsupported Media Type", "The media format of the requested data is not supported by the server, so the server is rejecting the request.", 415, "Unsupported Media Type", 0, "", "", ""),
  RangeNotSatisfiable => (416, "Range Not Satisfiable", "The range specified by the request's Range header field cannot be satisfied; the range may exceed the size of the data coming from the targeted URI.", 416, "Range Not Satisfiable", 0, "", "", ""),
  ExpectationFailed => (417, "Expectation Failed", "This response code means that the expectations indicated by the Expect request header field could not be met by the server.", 417, "Expectation Failed", 0, "", "", ""),
  ImATeapot => (418, "I'm a teapot", "The waiter refuses to brew coffee with a teapot, RFC 2324.", 418, "I'm a teapot", 0, "", "", ""),
  PageExpired => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 419, "PageExpired", 0, "", "", ""),
  MethodFailure => (405, "Method Not Allowed", "The HTTP method is not supported for the target resource", 420, "MethodFailure", 0, "", "", ""),
  MisdirectedRequest => (421, "Misdirected Request", "The request was sent to a server unable to produce a response. This code may be sent by a server that has not been configured to produce responses subject to the combination of schemas and identities included in the request URI.", 421, "Misdirected Request", 0, "", "", ""),
  UnprocessableEntity => (422, "Unprocessable Entity", "The request was successfully created but could not be processed due to semantic errors, WebDAV RFC 4918.", 422, "Unprocessable Entity", 0, "", "", ""),
  Locked => (423, "Locked", "The resource that is currently being viewed is locked.", 423, "Locked", 0, "", "", ""),
  FailedDependency => (424, "Failed Dependency", "The query failed because a previous query failed.", 424, "Failed Dependency", 0, "", "", ""),
  TooEarly => (422, "Unprocessable Entity", "Indicate that the server does not want to process a request that could be replayed.", 425, "Too Early", 0, "", "", ""),
  UpgradeRequired => (426, "Upgrade Required", "The server refuses to process the request using the current protocol but may agree to do so if the client opts for another protocol. The server must send an Upgrade header in the 426 response to indicate the requested protocol(s) (Section 6.7 of [RFC7230]).", 426, "Upgrade Required", 0, "", "", ""),
  PreconditionRequired => (428, "Precondition Required", "The origin server requires the request to be conditional. This is intended to prevent the 'loss of update' problem, where a client retrieves the state of a resource with GET, modifies it, and returns it to the server with PUT while a third party modifies the state of the resource. server, which leads to a conflict.", 428, "Precondition Required", 0, "", "", ""),
  TooManyRequests => (429, "Too Many Requests", "The user has sent too many requests in a given amount of time (rate limiting).", 429, "Too Many Requests", 0, "", "", ""),
  RequestHeaderFieldsTooLarge => (431, "Request Header Fields Too Large", "The server is unwilling to process the request because the header fields are too long. The request can be returned after reducing the size of the headers.", 431, "Request Header Fields Too Large", 0, "", "", ""),
  LoginRequired => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 432, "Login Required", 0, "", "", ""),
  OriginError => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 433, "Origin Error", 0, "", "", ""),
  DestinationError => (400, "Bad Request", "The request was rejected due to a destination server/config issue", 434, "DestinationError", 0, "", "", ""),
  TooLarge => (413, "Payload Too Large", "The request or resource is too large for the server to handle", 435, "TooLarge", 0, "", "", ""),
  SSLCertificateError => (400, "Bad Request", "An invalid or untrusted SSL certificate was encountered", 436, "SSLCertificateError", 0, "", "", ""),
  SSLCertificateRequired => (400, "Bad Request", "The server requires a valid SSL certificate for secure connections", 437, "SSLCertificateRequired", 0, "", "", ""),
  NoCertificate => (400, "Bad Request", "No SSL certificate was provided by the client", 438, "NoCertificate", 0, "", "", ""),
  LoginTimeout => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 440, "LoginTimeout", 0, "", "", ""),
  OverDataQuota => (413, "Payload Too Large", "The request or resource is too large for the server to handle", 441, "OverDataQuota", 0, "", "", ""),
  NoResponse => (400, "Bad Request", "The server closed the connection without sending a response", 444, "NoResponse", 0, "", "", ""),
  RetryWith => (428, "Precondition Required", "The user has sent too many requests in a given amount of time (rate limiting).", 449, "RetryWith", 0, "", "", ""),
  BlockedByWindowsParentalControls => (403, "Forbidden", "A Microsoft extension. This error is given when Windows Parental Controls are turned on and are blocking access to the given webpage.", 450, "BlockedByWindowsParentalControls", 0, "", "", ""),
  UnavailableForLegalReasons => (451, "Unavailable For Legal Reasons", "A server operator has received a legal demand to deny access to a resource or to a set of resources that includes the requested resource.", 451, "UnavailableForLegalReasons", 0, "", "", ""),
  TooManyRecipients => (429, "Too Many Requests",  "The user has sent too many requests in a given amount of time (rate limiting) or too many recipients or addresses used", 452, "TooManyRecipients", 0, "", "", ""),
  MethodNotValidInThisState => (405, "Method Not Allowed", "The HTTP method is not supported for the target resource", 453, "MethodNotValidInThisState", 0, "", "", ""),
  UnrecoverableError => (400, "Bad Request", "The server has encountered a situation it doesn't know how to handle.", 456, "UnrecoverableError", 0, "", "", ""),
  ClientClosedConnexionPrematurely => (400, "Bad Request", "The client closed the connection before the server could send a response.", 499, "ClientClosedConnexionPrematurely", 0, "", "", ""),
  TooManyForwardedIPAddresses => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 445, "TooManyForwardedIPAddresses", 0, "", "", ""),
  InternetSecurityError => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 446, "InternetSecurityError", 0, "", "", ""),
  RequestHeaderTooLarge => (431, "Request Header Fields Too Large", "The server is unwilling to process the request because the header fields are too long. The request can be returned after reducing the size of the headers.", 494, "RequestHeaderTooLarge", 0, "", "", ""),
  CertError => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 495, "CertError", 0, "", "", ""),
  NoCert => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 496, "NoCert", 0, "", "", ""),
  HTTPToHTTPS => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 497, "HTTPToHTTPS", 0, "", "", ""),
  InvalidToken => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 498, "InvalidToken", 0, "", "", ""),
  ClientClosedRequest => (400, "Bad Request", "The client closed the connection before the server could send a response.", 499, "ClientClosedRequest", 0, "", "", ""),
}

/// This file defines the `ResponsesClientCodes` enum and provides five main functionalities:
/// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
/// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
/// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
/// 4. `as_json()` - converts the variant to a JSON object.
/// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::UnifiedTuple;

  #[test]
  fn test_unauthorized_codes_to_u16() {
    let response = ResponsesClientCodes::Unauthorized;
    let code = response.to_u16();
    assert_eq!(code, 401);
  }

  #[test]
  fn test_not_found_codes_from_u16() {
    let status = ResponsesClientCodes::from_u16(404);
    assert_eq!(status, Some(ResponsesClientCodes::NotFound));
  }

  #[test]
  fn test_forbidden_codes_as_tuple() {
    let code = ResponsesClientCodes::Forbidden;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::NineFields(
        403,
        "Forbidden",
        "The client does not have access rights to the content; that is, it is unauthorized, so the server is refusing to give the requested resource. Unlike 401 Unauthorized, the client's identity is known to the server.",
        403,
        "Forbidden",
        403,
        "req-3",
        "user-3",
        "status-3"
      )
    );
  }

  #[test]
  fn test_not_found_codes_as_json() {
    let code = ResponsesClientCodes::NotFound;
    let json_result = code.as_json();
    let expected_json = json!({
      "standard http code": {
        "code": 404,
        "name": "Not Found"
      },
      "internal http code": {
        "code": 404,
        "name": "Not Found"
      },
      "description": "The server cannot find the requested resource. In the browser, this means the URL is not recognized. In an API, this can also mean that the endpoint is valid but the resource itself does not exist. Servers may also send this response instead of 403 Forbidden to hide the existence of a resource from an unauthorized client.",
      "metadata": {
        "internal code": 404,
        "request_id": "req-4",
        "user_id": "user-4",
        "status_id": "status-4"
      }
    });
    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_bad_request_codes_into_tuple() {
    let code = ResponsesClientCodes::BadRequest;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 400);
    assert_eq!(std_name, "Bad Request");
  }
}
