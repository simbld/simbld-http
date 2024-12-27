/// The code defines an enum representing HTTP response status codes with corresponding descriptions and provides helper functions to retrieve code-description pairs.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
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

/// implementation of a custom trait `ToU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “to_u16” method which converts a value from the enumeration into a “u16” type. Self accesses the value of the enum In the implementation, it calls the `into()` method to perform the conversion, which relies on the `Into<u16>` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl ToU16 for ResponsesClientCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

/// implementation of a custom trait `FromU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “from_u16” method which converts a value from the enumeration into an `Option<Self>` type. The method uses the `try_from` method to perform the conversion, which relies on the `TryFromPrimitive` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl FromU16 for ResponsesClientCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

/// implementation of a custom trait `Into` for the `ResponsesLocalApiCodes` enumeration. We provide an “into” method which converts a value from the enumeration into a tuple containing a `u16` and a `&'static str`. The method calls the `to_u16` method to get the status code and the `get_str` method to get the description. The `unwrap_or` method is used to provide a default value in case the description is not found. The method returns the tuple containing the status code and the description
impl Into<(u16, &'static str)> for ResponsesClientCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

/// Functions return raw data as a tuple for further processing or formats containing HTTP status code, status message and description of various client error responses.
pub fn bad_request_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::BadRequest;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Bad Request", description)
}

pub fn unauthorized_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::Unauthorized;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unauthorized", description)
}

pub fn payment_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::PaymentRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Payment Required", description)
}

pub fn forbidden_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::Forbidden;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Forbidden", description)
}

pub fn not_found_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::NotFound;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Not Found", description)
}

pub fn method_not_allowed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::MethodNotAllowed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Method Not Allowed", description)
}

pub fn not_acceptable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::NotAcceptable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Not Acceptable", description)
}

pub fn proxy_authentication_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::ProxyAuthenticationRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Proxy Authentication Required", description)
}

pub fn request_timeout_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::RequestTimeout;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Request Timeout", description)
}

pub fn conflict_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::Conflict;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Conflict", description)
}

pub fn gone_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::Gone;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Gone", description)
}

pub fn length_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::LengthRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Length Required", description)
}

pub fn precondition_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::PreconditionFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Precondition Failed", description)
}

pub fn payload_too_large_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::PayloadTooLarge;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Payload Too Large", description)
}

pub fn request_uri_too_long_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::RequestUriTooLong;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Request URI Too Long", description)
}

pub fn unsupported_media_type_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::UnsupportedMediaType;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unsupported Media Type", description)
}

pub fn requested_range_unsatisfiable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::RequestedRangeUnsatisfiable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Requested Range Unsatisfiable", description)
}

pub fn expectation_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::ExpectationFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Expectation Failed", description)
}

pub fn im_a_teapot_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::ImATeapot;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "I'm a teapot", description)
}

pub fn page_expired_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::PageExpired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Page Expired", description)
}

pub fn method_failure_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::MethodFailure;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Method Failure", description)
}

pub fn misdirected_request_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::MisdirectedRequest;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Misdirected Request", description)
}

pub fn unprocessable_entity_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::UnprocessableEntity;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unprocessable Entity", description)
}

pub fn locked_temporarily_unavailable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::LockedTemporarilyUnavailable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Locked Temporarily Unavailable", description)
}

pub fn failed_dependency_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::FailedDependency;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Failed Dependency", description)
}

pub fn too_early_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::TooEarly;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Too Early", description)
}

pub fn upgrade_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::UpgradeRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Upgrade Required", description)
}

pub fn precondition_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::PreconditionRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Precondition Required", description)
}

pub fn too_many_requests_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::TooManyRequests;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Too Many Requests", description)
}

pub fn request_header_fields_too_large_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::RequestHeaderFieldsTooLarge;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Request Header Fields Too Large", description)
}

pub fn login_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::LoginRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Login Required", description)
}

pub fn origin_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::OriginError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Origin Error", description)
}

pub fn destination_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::DestinationError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Destination Error", description)
}

pub fn too_large_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::TooLarge;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Too Large", description)
}

pub fn ssl_certificate_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::SSLCertificateError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "SSL Certificate Error", description)
}

pub fn ssl_certificate_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::SSLCertificateRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "SSL Certificate Required", description)
}

pub fn no_certificate_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::NoCertificate;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "No Certificate", description)
}

pub fn login_timeout_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::LoginTimeout;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Login Timeout", description)
}

pub fn over_data_quota_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::OverDataQuota;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Over Data Quota", description)
}

pub fn no_response_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::NoResponse;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "No Response", description)
}

pub fn retry_with_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::RetryWith;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Retry With", description)
}

pub fn blocked_by_windows_parental_controls_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::BlockedByWindowsParentalControls;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Blocked By Windows Parental Controls", description)
}

pub fn unavailable_for_legal_reasons_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::UnavailableForLegalReasons;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unavailable For Legal Reasons", description)
}

pub fn too_many_recipients_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::TooManyRecipients;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Too Many Recipients", description)
}

pub fn method_not_valid_in_this_state_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::MethodNotValidInThisState;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Method Not Valid In This State", description)
}

pub fn unrecoverable_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::UnrecoverableError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unrecoverable Error", description)
}

pub fn client_closed_connexion_prematurely_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::ClientClosedConnexionPrematurely;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Client Closed Connexion Prematurely", description)
}

pub fn too_many_forwarded_ip_addresses_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::TooManyForwardedIPAddresses;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Too Many Forwarded IP Addresses", description)
}

pub fn internet_security_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::InternetSecurityError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Internet Security Error", description)
}

pub fn temporary_unavailable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::TemporaryUnavailable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Temporary Unavailable", description)
}

pub fn request_header_too_large_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::RequestHeaderTooLarge;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Request Header Too Large", description)
}

pub fn cert_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::CertError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Cert Error", description)
}

pub fn no_cert_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::NoCert;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "No Cert", description)
}

pub fn http_to_https_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::HTTPToHTTPS;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "HTTP To HTTPS", description)
}

pub fn invalid_token_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::InvalidToken;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Token", description)
}

pub fn client_closed_request_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesClientCodes::ClientClosedRequest;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Client Closed Request", description)
}

/// Functions return formatted data as JSON containing HTTP status code, status message and description of various informational responses.
pub fn bad_request() -> serde_json::Value {
  let (code, name, desc) = bad_request_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unauthorized() -> serde_json::Value {
  let (code, name, desc) = unauthorized_tuple();
  json!({ "status": code, "name": name, "description": desc })
}
pub fn payment_required() -> serde_json::Value {
  let (code, name, desc) = payment_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn forbidden() -> serde_json::Value {
  let (code, name, desc) = forbidden_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn not_found() -> serde_json::Value {
  let (code, name, desc) = not_found_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn method_not_allowed() -> serde_json::Value {
  let (code, name, desc) = method_not_allowed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn not_acceptable() -> serde_json::Value {
  let (code, name, desc) = not_acceptable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn proxy_authentication_required() -> serde_json::Value {
  let (code, name, desc) = proxy_authentication_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn request_timeout() -> serde_json::Value {
  let (code, name, desc) = request_timeout_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn conflict() -> serde_json::Value {
  let (code, name, desc) = conflict_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn gone() -> serde_json::Value {
  let (code, name, desc) = gone_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn length_required() -> serde_json::Value {
  let (code, name, desc) = length_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn precondition_failed() -> serde_json::Value {
  let (code, name, desc) = precondition_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn payload_too_large() -> serde_json::Value {
  let (code, name, desc) = payload_too_large_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn request_uri_too_long() -> serde_json::Value {
  let (code, name, desc) = request_uri_too_long_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unsupported_media_type() -> serde_json::Value {
  let (code, name, desc) = unsupported_media_type_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn requested_range_unsatisfiable() -> serde_json::Value {
  let (code, name, desc) = requested_range_unsatisfiable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn expectation_failed() -> serde_json::Value {
  let (code, name, desc) = expectation_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn im_a_teapot() -> serde_json::Value {
  let (code, name, desc) = im_a_teapot_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn page_expired() -> serde_json::Value {
  let (code, name, desc) = page_expired_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn method_failure() -> serde_json::Value {
  let (code, name, desc) = method_failure_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn misdirected_request() -> serde_json::Value {
  let (code, name, desc) = misdirected_request_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unprocessable_entity() -> serde_json::Value {
  let (code, name, desc) = unprocessable_entity_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn locked_temporarily_unavailable() -> serde_json::Value {
  let (code, name, desc) = locked_temporarily_unavailable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn failed_dependency() -> serde_json::Value {
  let (code, name, desc) = failed_dependency_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn too_early() -> serde_json::Value {
  let (code, name, desc) = too_early_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn upgrade_required() -> serde_json::Value {
  let (code, name, desc) = upgrade_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn precondition_required() -> serde_json::Value {
  let (code, name, desc) = precondition_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn too_many_requests() -> serde_json::Value {
  let (code, name, desc) = too_many_requests_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn request_header_fields_too_large() -> serde_json::Value {
  let (code, name, desc) = request_header_fields_too_large_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn login_required() -> serde_json::Value {
  let (code, name, desc) = login_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn origin_error() -> serde_json::Value {
  let (code, name, desc) = origin_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn destination_error() -> serde_json::Value {
  let (code, name, desc) = destination_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn too_large() -> serde_json::Value {
  let (code, name, desc) = too_large_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn ssl_certificate_error() -> serde_json::Value {
  let (code, name, desc) = ssl_certificate_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn ssl_certificate_required() -> serde_json::Value {
  let (code, name, desc) = ssl_certificate_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn no_certificate() -> serde_json::Value {
  let (code, name, desc) = no_certificate_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn login_timeout() -> serde_json::Value {
  let (code, name, desc) = login_timeout_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn over_data_quota() -> serde_json::Value {
  let (code, name, desc) = over_data_quota_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn no_response() -> serde_json::Value {
  let (code, name, desc) = no_response_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn retry_with() -> serde_json::Value {
  let (code, name, desc) = retry_with_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn blocked_by_windows_parental_controls() -> serde_json::Value {
  let (code, name, desc) = blocked_by_windows_parental_controls_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unavailable_for_legal_reasons() -> serde_json::Value {
  let (code, name, desc) = unavailable_for_legal_reasons_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn too_many_recipients() -> serde_json::Value {
  let (code, name, desc) = too_many_recipients_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn method_not_valid_in_this_state() -> serde_json::Value {
  let (code, name, desc) = method_not_valid_in_this_state_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unrecoverable_error() -> serde_json::Value {
  let (code, name, desc) = unrecoverable_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn client_closed_connexion_prematurely() -> serde_json::Value {
  let (code, name, desc) = client_closed_connexion_prematurely_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn too_many_forwarded_ip_addresses() -> serde_json::Value {
  let (code, name, desc) = too_many_forwarded_ip_addresses_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn internet_security_error() -> serde_json::Value {
  let (code, name, desc) = internet_security_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn temporary_unavailable() -> serde_json::Value {
  let (code, name, desc) = temporary_unavailable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn request_header_too_large() -> serde_json::Value {
  let (code, name, desc) = request_header_too_large_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn cert_error() -> serde_json::Value {
  let (code, name, desc) = cert_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn no_cert() -> serde_json::Value {
  let (code, name, desc) = no_cert_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn http_to_https() -> serde_json::Value {
  let (code, name, desc) = http_to_https_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_token() -> serde_json::Value {
  let (code, name, desc) = invalid_token_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn client_closed_request() -> serde_json::Value {
  let (code, name, desc) = client_closed_request_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

// Unit tests
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
  fn tes_from_u16_not_found() {
    let response = ResponsesClientCodes::from_u16(404);
    assert_eq!(response, Some(ResponsesClientCodes::NotFound));
  }

  #[test]
  fn test_login_required_tuple() {
    assert_eq!(
          login_required_tuple(),
          (
              432,
              "Login Required",
              "Authentication is required to access the requested resource, typically in web applications"
          )
      );
  }

  #[test]
  fn test_internet_security_error() {
    let (code, name, description) = internet_security_error_tuple();
    assert_eq!(code, 467);
    let response = json!({
        "status": code,
        "name": name,
        "description": description
    });
    assert_eq!(response["status"], 467);
    assert_eq!(response["name"], "Internet Security Error");
    assert_eq!(
        response["description"],
        "An internet security policy violation or configuration issue occurred, often related to SSL/TLS settings, certificates, or protocol mismatches"
    );
  }
}
