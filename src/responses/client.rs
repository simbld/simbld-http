/// The code defines an enum representing HTTP response status codes with corresponding descriptions and provides helper functions to retrieve code-description pairs.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
#[repr(u16)]

pub enum ResponsesClientCodes {
  #[strum(props(
    Description = "The server cannot process the request due to malformed syntax or invalid parameters in the client request"
  ))]
  BadRequest = 400,
  #[strum(props(
    Description = "The client must authenticate itself to get the requested resource, typically a 401 Unauthorized response"
  ))]
  Unauthorized = 401,
  #[strum(props(
    Description = "Payment is required to access the requested resource, though this is not widely used in practice"
  ))]
  PaymentRequired = 402,
  #[strum(props(
    Description = "The server understands the request but refuses to authorize it, indicating insufficient permissions"
  ))]
  Forbidden = 403,
  #[strum(props(
    Description = "The server cannot find the requested resource, indicating a non-existent or inaccessible URI"
  ))]
  NotFound = 404,
  #[strum(props(
    Description = "The HTTP method used in the request is not supported for the target resource"
  ))]
  MethodNotAllowed = 405,
  #[strum(props(
    Description = "The requested resource cannot be provided in a format acceptable according to the request's Accept headers"
  ))]
  NotAcceptable = 406,
  #[strum(props(
    Description = "The client must authenticate with a proxy server before accessing the resource"
  ))]
  ProxyAuthenticationRequired = 407,
  #[strum(props(
    Description = "The server timed out while waiting for the request from the client. This status code is used to inform the client that the server timed out."
  ))]
  RequestTimeout = 408,
  #[strum(props(
    Description = "The request could not be completed due to a conflict with the current state of the target resource"
  ))]
  Conflict = 409,
  #[strum(props(
    Description = "The requested resource is no longer available and has been permanently removed from the server and will not be available again"
  ))]
  Gone = 410,
  #[strum(props(Description = "The request does not include the required Content-Length header"))]
  LengthRequired = 411,
  #[strum(props(
    Description = "One or more conditions in the request headers are not met by the server"
  ))]
  PreconditionFailed = 412,
  #[strum(props(
    Description = "The size of the request payload exceeds the server's capacity or configuration limits"
  ))]
  PayloadTooLarge = 413,
  #[strum(props(
    Description = "The URI of the request is too long for the server to process. This status code is used to inform the client that the request URI is too long"
  ))]
  RequestUriTooLong = 414,
  #[strum(props(
    Description = "The media type of the request payload is not supported by the server or target resource"
  ))]
  UnsupportedMediaType = 415,
  #[strum(props(
    Description = "The client requested a range that is not satisfiable for the target resource"
  ))]
  RequestedRangeUnsatisfiable = 416,
  #[strum(props(
    Description = "The server cannot meet the requirements specified in the Expect header field of the request"
  ))]
  ExpectationFailed = 417,
  #[strum(props(
    Description = "A playful response indicating the server is a teapot and cannot brew coffee (RFC 2324)"
  ))]
  ImATeapot = 418,
  #[strum(props(
    Description = "Issued by Laravel when a CSRF token is missing or expired, unofficial"
  ))]
  PageExpired = 419,
  #[strum(props(
    Description = "The method specified in the request is known by the server but cannot be processed due to a failure in the server's implementation, Issued by Spring when a method has failed. Now deprecated and reserved for backward compatibility, unofficial"
  ))]
  MethodFailure = 420,
  #[strum(props(
    Description = "Used by Twitter to indicate that the client has sent too many requests in a given amount of time, unofficial"
  ))]
  MisdirectedRequest = 421,
  #[strum(props(
    Description = "The request is well-formed but cannot be processed due to semantic errors, commonly used in APIs, use in WebDav RFC 4918"
  ))]
  UnprocessableEntity = 422,
  #[strum(props(
    Description = "The resource is locked and cannot be accessed or modified, typically used in WebDav RFC 4918"
  ))]
  LockedTemporarilyUnavailable = 423,
  #[strum(props(
    Description = "The request failed because it depended on another operation that failed, often used in WebDav RFC 4918"
  ))]
  FailedDependency = 424,
  #[strum(props(
    Description = "The server is unwilling to process the request because it might be replayed"
  ))]
  TooEarly = 425, // Only for Firefox
  #[strum(props(
    Description = "The client must upgrade to a different protocol to continue with the request"
  ))]
  UpgradeRequired = 426,
  #[strum(props(
    Description = "The server requires the request to include specific preconditions to proceed"
  ))]
  PreconditionRequired = 428,
  #[strum(props(
    Description = "The resource is rate-limited and the client has sent too many requests in the allotted time"
  ))]
  TooManyRequests = 429,
  #[strum(props(
    Description = "Issued by Shopify to indicate a rate-limit effect. This is used instead of 429, unofficial"
  ))]
  RequestHeaderFieldsTooLarge = 430,
  #[strum(props(
    Description = "Authentication is required to access the requested resource, typically in web applications"
  ))]
  LoginRequired = 432,
  #[strum(props(
    Description = "The request was rejected due to an issue with the origin server or client IP"
  ))]
  OriginError = 433,
  #[strum(props(
    Description = "The request was rejected due to an issue with the destination server or target configuration"
  ))]
  DestinationError = 434,
  #[strum(props(
    Description = "The size of the requested resource or payload exceeds the allowable limit for the server"
  ))]
  TooLarge = 435,
  #[strum(props(
    Description = "An error occurred due to an invalid or untrusted SSL certificate"
  ))]
  SSLCertificateError = 436,
  #[strum(props(
    Description = "The server requires a valid SSL certificate for the connection to proceed securely"
  ))]
  SSLCertificateRequired = 437,
  #[strum(props(
    Description = "The client did not provide an SSL certificate required for secure communication"
  ))]
  NoCertificate = 438,
  #[strum(props(
    Description = "The client session timed out and must log in again, iis, unofficial"
  ))]
  LoginTimeout = 440,
  #[strum(props(
    Description = "The client has exceeded the allocated data quota for the requested operation"
  ))]
  OverDataQuota = 441,
  #[strum(props(
    Description = "The server closed the connection without sending any response, often used in scenarios where the server chooses to silently drop the request, nginx, unofficial"
  ))]
  NoResponse = 444,
  #[strum(props(
    Description = "The user has not provided the required information, iis, unofficial"
  ))]
  RetryWith = 449,
  #[strum(props(
    Description = "Issued by Microsoft when Windows Parental Controls are turned on and a resource is blocked, unofficial"
  ))]
  BlockedByWindowsParentalControls = 450,
  #[strum(props(
    Description = "The server is denying access to the resource due to legal reasons, such as censorship or compliance with local laws"
  ))]
  UnavailableForLegalReasons = 451,
  #[strum(props(
    Description = "The server is unable to process the request because it contains too many recipients"
  ))]
  TooManyRecipients = 452,
  #[strum(props(
    Description = "The method specified in the request is not valid for the current state of the resource or server"
  ))]
  MethodNotValidInThisState = 455,
  #[strum(props(
    Description = "The server encountered a critical error that prevents it from continuing to process the request"
  ))]
  UnrecoverableError = 456,
  #[strum(props(
    Description = "The client closed the connection before the server was able to send a response, often due to a timeout or network interruption"
  ))]
  ClientClosedConnexionPrematurely = 460,
  #[strum(props(
    Description = "The server rejected the request due to an excessive number of forwarded IP addresses in the request headers, potentially indicating a misconfiguration or a security concern"
  ))]
  TooManyForwardedIPAddresses = 463,
  #[strum(props(
    Description = "An internet security policy violation or configuration issue occurred, often related to SSL/TLS settings, certificates, or protocol mismatches"
  ))]
  InternetSecurityError = 467,
  #[strum(props(
    Description = "The server is temporarily unavailable, usually due to maintenance or overload"
  ))]
  TemporaryUnavailable = 480,
  #[strum(props(
    Description = "The server is unable to process the request because the headers are too large, often due to a misconfiguration or an attack, nginx, unofficial"
  ))]
  RequestHeaderTooLarge = 494,
  #[strum(props(
    Description = "The SSL certificate presented by the client is invalid or cannot be verified by the server, preventing a secure connection from being established, nginx, unofficial"
  ))]
  CertError = 495,
  #[strum(props(
    Description = "A required client certificate wasn't provided, preventing the server from establishing a secure connection, nginx, unofficial"
  ))]
  NoCert = 496,
  #[strum(props(
    Description = "The client sent an unencrypted HTTP request to a server that requires HTTPS, and the server is redirecting the client to the HTTPS version of the resource, nginx, unofficial"
  ))]
  HTTPToHTTPS = 497,
  #[strum(props(
    Description = "The provided token is invalid, expired, or malformed, and cannot be used for authentication or authorization, Issued by ArcGIS for Server, unofficial"
  ))]
  InvalidToken = 498,
  #[strum(props(
    Description = "The client closed the connection before the server could provide a response, often due to client timeout or network interruption, nginx, unofficial"
  ))]
  ClientClosedRequest = 499,
}

impl ToU16 for ResponsesClientCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesClientCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

impl Into<(u16, &'static str)> for ResponsesClientCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

pub fn bad_request() -> (u16, &'static str) {
  (400, "The server cannot process the request due to malformed syntax or invalid parameters in the client request")
}

pub fn unauthorized() -> (u16, &'static str) {
  (401, "The client must authenticate itself to get the requested resource, typically a 401 Unauthorized response")
}

pub fn payment_required() -> (u16, &'static str) {
  (402, "Payment is required to access the requested resource, though this is not widely used in practice")
}

pub fn forbidden() -> (u16, &'static str) {
  (403, "The server understands the request but refuses to authorize it, indicating insufficient permissions")
}

pub fn not_found() -> (u16, &'static str) {
  (
    404,
    "The server cannot find the requested resource, indicating a non-existent or inaccessible URI",
  )
}

pub fn method_not_allowed() -> (u16, &'static str) {
  (405, "The HTTP method used in the request is not supported for the target resource")
}

pub fn not_acceptable() -> (u16, &'static str) {
  (406, "The requested resource cannot be provided in a format acceptable according to the request's Accept headers")
}

pub fn proxy_authentication_required() -> (u16, &'static str) {
  (407, "The client must authenticate with a proxy server before accessing the resource")
}

pub fn request_timeout() -> (u16, &'static str) {
  (408, "The server timed out while waiting for the request from the client. This status code is used to inform the client that the server timed out.")
}

pub fn conflict() -> (u16, &'static str) {
  (409, "The request could not be completed due to a conflict with the current state of the target resource")
}

pub fn gone() -> (u16, &'static str) {
  (410, "The requested resource is no longer available and has been permanently removed from the server and will not be available again")
}

pub fn length_required() -> (u16, &'static str) {
  (411, "The request does not include the required Content-Length header")
}

pub fn precondition_failed() -> (u16, &'static str) {
  (412, "One or more conditions in the request headers are not met by the server")
}

pub fn payload_too_large() -> (u16, &'static str) {
  (413, "The size of the request payload exceeds the server's capacity or configuration limits")
}

pub fn request_uri_too_long() -> (u16, &'static str) {
  (414, "The URI of the request is too long for the server to process. This status code is used to inform the client that the request URI is too long")
}

pub fn unsupported_media_type() -> (u16, &'static str) {
  (415, "The media type of the request payload is not supported by the server or target resource")
}

pub fn requested_range_unsatisfiable() -> (u16, &'static str) {
  (416, "The client requested a range that is not satisfiable for the target resource")
}

pub fn expectation_failed() -> (u16, &'static str) {
  (
    417,
    "The server cannot meet the requirements specified in the Expect header field of the request",
  )
}

pub fn im_a_teapot() -> (u16, &'static str) {
  (418, "A playful response indicating the server is a teapot and cannot brew coffee (RFC 2324)")
}

pub fn page_expired() -> (u16, &'static str) {
  (419, "Issued by Laravel when a CSRF token is missing or expired, unofficial")
}

pub fn method_failure() -> (u16, &'static str) {
  (420, "The method specified in the request is known by the server but cannot be processed due to a failure in the server's implementation, Issued by Spring when a method has failed. Now deprecated and reserved for backward compatibility, unofficial")
}

pub fn misdirected_request() -> (u16, &'static str) {
  (421, "Used by Twitter to indicate that the client has sent too many requests in a given amount of time, unofficial")
}

pub fn unprocessable_entity() -> (u16, &'static str) {
  (422, "The request is well-formed but cannot be processed due to semantic errors, commonly used in APIs, use in WebDav RFC 4918")
}

pub fn locked_temporarily_unavailable() -> (u16, &'static str) {
  (
    423,
    "The resource is locked and cannot be accessed or modified, typically used in WebDav RFC 4918",
  )
}

pub fn failed_dependency() -> (u16, &'static str) {
  (424, "The request failed because it depended on another operation that failed, often used in WebDav RFC 4918")
}

pub fn too_early() -> (u16, &'static str) {
  (425, "The server is unwilling to process the request because it might be replayed")
}

pub fn upgrade_required() -> (u16, &'static str) {
  (426, "The client must upgrade to a different protocol to continue with the request")
}

pub fn precondition_required() -> (u16, &'static str) {
  (428, "The server requires the request to include specific preconditions to proceed")
}

pub fn too_many_requests() -> (u16, &'static str) {
  (
    429,
    "The resource is rate-limited and the client has sent too many requests in the allotted time",
  )
}

pub fn request_header_fields_too_large() -> (u16, &'static str) {
  (
    430,
    "Issued by Shopify to indicate a rate-limit effect. This is used instead of 429, unofficial",
  )
}

pub fn login_required() -> (u16, &'static str) {
  (
    432,
    "Authentication is required to access the requested resource, typically in web applications",
  )
}

pub fn origin_error() -> (u16, &'static str) {
  (433, "The request was rejected due to an issue with the origin server or client IP")
}

pub fn destination_error() -> (u16, &'static str) {
  (
    434,
    "The request was rejected due to an issue with the destination server or target configuration",
  )
}

pub fn too_large() -> (u16, &'static str) {
  (435, "The size of the requested resource or payload exceeds the allowable limit for the server")
}

pub fn ssl_certificate_error() -> (u16, &'static str) {
  (436, "An error occurred due to an invalid or untrusted SSL certificate")
}

pub fn ssl_certificate_required() -> (u16, &'static str) {
  (437, "The server requires a valid SSL certificate for the connection to proceed securely")
}

pub fn no_certificate() -> (u16, &'static str) {
  (438, "The client did not provide an SSL certificate required for secure communication")
}

pub fn login_timeout() -> (u16, &'static str) {
  (440, "The client session timed out and must log in again, iis, unofficial")
}

pub fn over_data_quota() -> (u16, &'static str) {
  (441, "The client has exceeded the allocated data quota for the requested operation")
}

pub fn no_response() -> (u16, &'static str) {
  (444, "The server closed the connection without sending any response, often used in scenarios where the server chooses to silently drop the request, nginx, unofficial")
}

pub fn retry_with() -> (u16, &'static str) {
  (449, "The user has not provided the required information, iis, unofficial")
}

pub fn blocked_by_windows_parental_controls() -> (u16, &'static str) {
  (450, "Issued by Microsoft when Windows Parental Controls are turned on and a resource is blocked, unofficial")
}

pub fn unavailable_for_legal_reasons() -> (u16, &'static str) {
  (451, "The server is denying access to the resource due to legal reasons, such as censorship or compliance with local laws")
}

pub fn too_many_recipients() -> (u16, &'static str) {
  (452, "The server is unable to process the request because it contains too many recipients")
}

pub fn method_not_valid_in_this_state() -> (u16, &'static str) {
  (455, "The method specified in the request is not valid for the current state of the resource or server")
}

pub fn unrecoverable_error() -> (u16, &'static str) {
  (456, "The server encountered a critical error that prevents it from continuing to process the request")
}

pub fn client_closed_connexion_prematurely() -> (u16, &'static str) {
  (460, "The client closed the connection before the server was able to send a response, often due to a timeout or network interruption")
}

pub fn too_many_forwarded_ip_addresses() -> (u16, &'static str) {
  (463, "The server rejected the request due to an excessive number of forwarded IP addresses in the request headers, potentially indicating a misconfiguration or a security concern")
}

pub fn internet_security_error() -> (u16, &'static str) {
  (467, "An internet security policy violation or configuration issue occurred, often related to SSL/TLS settings, certificates, or protocol mismatches")
}

pub fn temporary_unavailable() -> (u16, &'static str) {
  (480, "The server is temporarily unavailable, usually due to maintenance or overload")
}

pub fn request_header_too_large() -> (u16, &'static str) {
  (494, "The server is unable to process the request because the headers are too large, often due to a misconfiguration or an attack, nginx, unofficial")
}

pub fn cert_error() -> (u16, &'static str) {
  (495, "The SSL certificate presented by the client is invalid or cannot be verified by the server, preventing a secure connection from being established, nginx, unofficial")
}

pub fn no_cert() -> (u16, &'static str) {
  (496, "A required client certificate wasn't provided, preventing the server from establishing a secure connection, nginx, unofficial")
}

pub fn http_to_https() -> (u16, &'static str) {
  (497, "The client sent an unencrypted HTTP request to a server that requires HTTPS, and the server is redirecting the client to the HTTPS version of the resource, nginx, unofficial")
}

pub fn invalid_token() -> (u16, &'static str) {
  (498, "The provided token is invalid, expired, or malformed, and cannot be used for authentication or authorization, Issued by ArcGIS for Server, unofficial")
}

pub fn client_closed_request() -> (u16, &'static str) {
  (499, "The client closed the connection before the server could provide a response, often due to client timeout or network interruption, nginx, unofficial")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_bad_request() {
    let response = ResponsesClientCodes::BadRequest;
    let (code, description) = response.into();
    assert_eq!(code, 400);
    assert_eq!(description, "The server cannot process the request due to malformed syntax or invalid parameters in the client request");
  }

  #[test]
  fn test_to_u16_unauthorized() {
    let response = ResponsesClientCodes::Unauthorized;
    let code = response.to_u16();
    assert_eq!(code, 401);
  }

  #[test]
  fn test_payment_required() {
    assert_eq!(payment_required(), (402, "Payment is required to access the requested resource, though this is not widely used in practice"));
  }

  #[test]
  fn tes_from_u16_not_found() {
    let response = ResponsesClientCodes::from_u16(404);
    assert_eq!(response, Some(ResponsesClientCodes::NotFound));
  }
}
