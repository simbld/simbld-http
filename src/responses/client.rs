use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{EnumIter, EnumProperty};

#[derive(IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
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
