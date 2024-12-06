use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
#[repr(u16)]

pub enum ResponsesRedirectionCodes {
  #[strum(props(
    Description = "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually"
  ))]
  MultipleChoices = 300,
  #[strum(props(
    Description = "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection"
  ))]
  MovedPermanently = 301,
  #[strum(props(
    Description = "The resource is temporarily available at a different URI. The client should continue using the original URI for future requests. This status code is often used for URL redirection"
  ))]
  Found = 302,
  #[strum(props(
    Description = "The response to the request can be found under another URI, and the client should use GET to retrieve it. This status code is used to direct the client to retrieve the resource from a different URI"
  ))]
  SeeOther = 303,
  #[strum(props(
    Description = "The resource has not been modified since the version specified in the request headers. This status code is used for caching purposes to reduce unnecessary network traffic"
  ))]
  NotModified = 304,
  #[strum(props(
    Description = "The requested resource must be accessed through a specified proxy. This status code is used to inform the client that it should use a proxy server to access the resource"
  ))]
  UseProxy = 305,
  #[strum(props(
    Description = "Switch to the specified proxy. This is no longer used, and it is now considered deprecated"
  ))]
  SwitchProxy = 306,
  #[strum(props(
    Description = "The resource is temporarily located at a different URI. The client should use the same method to access it. This status code is used for temporary URL redirection"
  ))]
  TemporaryRedirect = 307,
  #[strum(props(
    Description = "The resource has been permanently moved to a new URI. The client should update its references. This status code is used for permanent URL redirection"
  ))]
  PermanentRedirect = 308,
  #[strum(props(
    Description = "The client has been redirected too many times, possibly causing a redirection loop. This status code is used to prevent infinite redirection loops"
  ))]
  TooManyRedirects = 310,
  #[strum(props(
    Description = "The client should use a different method to access the resource. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST"
  ))]
  RedirectMethod = 311,
  #[strum(props(
    Description = "This code is currently unassigned and reserved for future use. It may be used for a new feature or status code in the future"
  ))]
  Unassigned = 312,
  #[strum(props(
    Description = "The requested resource has been permanently moved to a new URI, but the client should continue to use the original URI. This status code is used for special cases of permanent redirection"
  ))]
  MovedPermanentlyRedirected = 321,
  #[strum(props(
    Description = "The requested resource is temporarily available at a new URI but the client should not update its original URI. This status code is used for special cases of temporary redirection"
  ))]
  MovedTemporarilyRedirected = 322,
  #[strum(props(
    Description = "The requested resource can be accessed at a different URI using the GET method. This status code is used to direct the client to retrieve the resource from a different URI using GET"
  ))]
  SeeOtherRedirected = 323,
  #[strum(props(
    Description = "The requested resource has not been modified and can be retrieved from the cache. This status code is used for caching purposes to reduce unnecessary network traffic"
  ))]
  NotModifiedRedirected = 324,
  #[strum(props(
    Description = "The resource must be accessed through a proxy, and the proxy details are provided. This status code is used to inform the client that it should use a proxy server to access the resource"
  ))]
  UseProxyRedirected = 325,
  #[strum(props(
    Description = "This status code is reserved and not used anymore. It was previously used for a proposed feature that was never implemented"
  ))]
  UnusedRedirected = 326,
  #[strum(props(
    Description = "The requested resource is temporarily located at a new URI. The client should not update its reference. This status code is used for special cases of temporary redirection"
  ))]
  TemporaryRedirectRedirected = 327,
  #[strum(props(
    Description = "The resource has been permanently moved to a new URI, and future requests should use the new URI. This status code is used for special cases of permanent redirection"
  ))]
  PermanentRedirected = 328,
  #[strum(props(
    Description = "The client has been redirected too many times during a redirection loop. This status code is used to prevent infinite redirection loops"
  ))]
  TooManyRedirectsRedirected = 329,
  #[strum(props(
    Description = "The redirection requires the client to use a different request method. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST"
  ))]
  RedirectMethodRedirected = 330,
  #[strum(props(
    Description = "The username is valid, but the client must provide a password to proceed. This status code is used for authentication purposes"
  ))]
  UserNameOkPasswordNeeded = 331,
  #[strum(props(
    Description = "The requested resource does not require a user account for access. This status code is used to inform the client that no login is necessary"
  ))]
  NoNeedAccountForLogin = 332,
  #[strum(props(
    Description = "The request is missing a session key in the header. This status code is used for session management purposes"
  ))]
  SessionKeyNotPresentInHeader = 333,
  #[strum(props(
    Description = "The session key provided in the request cannot be decrypted or parsed. This status code is used for session management purposes"
  ))]
  SessionKeyPresentAndNotDecryptableParsable = 334,
  #[strum(props(
    Description = "The server refuses to process the request, often due to policy restrictions. This status code is used to inform the client that the server is unwilling to process the request"
  ))]
  ServerIsUnwillingToProcessTheRequest = 335,
  #[strum(props(
    Description = "Challenge-response authentication was successfully completed. This status code is used to inform the client that authentication was successful"
  ))]
  ChallengeResponseAuthenticationOk = 336,
  #[strum(props(
    Description = "Challenge-response authentication failed due to invalid credentials or other issues. This status code is used to inform the client that authentication failed"
  ))]
  ChallengeResponseAuthenticationFailed = 337,
  #[strum(props(
    Description = "The request did not specify the length of its content, which is required by the server. This status code is used to inform the client that the length is required"
  ))]
  LengthRequired = 342,
  #[strum(props(
    Description = "The server does not meet the preconditions set by the client in its request. This status code is used to inform the client that the preconditions failed"
  ))]
  PreconditionFailed = 343,
  #[strum(props(
    Description = "The request is larger than the server is willing or able to process. This status code is used to inform the client that the request entity is too large"
  ))]
  RequestEntityTooLarge = 344,
  #[strum(props(
    Description = "The media type of the request is not supported by the server. This status code is used to inform the client that the media type is unsupported"
  ))]
  UnsupportedMediaType = 346,
  #[strum(props(
    Description = "The server cannot supply the portion of the file requested by the client. This status code is used to inform the client that the requested range is not satisfiable"
  ))]
  RequestedRangeNotSatisfiable = 347,
  #[strum(props(
    Description = "The server cannot meet the requirements specified in the Expect header of the request. This status code is used to inform the client that the expectation failed"
  ))]
  ExpectationFailed = 348,
  #[strum(props(
    Description = "A humorous response indicating the server is a teapot and refuses to brew coffee. This status code is used as an April Fools' joke"
  ))]
  ImATeapot = 349,
  #[strum(props(
    Description = "The server encountered an error while attempting to access the specified URL. This status code is used to inform the client that there was an error accessing the URL"
  ))]
  ErrorAccessingURL = 350,
  #[strum(props(
    Description = "The requested redirection trigger could not be found on the server. This status code is used to inform the client that the trigger was not found"
  ))]
  TriggerNotFound = 351,
  #[strum(props(
    Description = "The server refuses to fulfill the request due to access restrictions. This status code is used to inform the client that access is denied"
  ))]
  AccessDenied = 352,
  #[strum(props(
    Description = "A condition required to complete the redirection was not satisfied. This status code is used to inform the client that the condition failed"
  ))]
  ConditionFailed = 353,
  #[strum(props(
    Description = "A required parameter for the request is missing or null. This status code is used to inform the client that a mandatory parameter is null"
  ))]
  MandatoryParameterIsNull = 354,
  #[strum(props(
    Description = "A parameter specified in the request does not exist. This status code is used to inform the client that the parameter does not exist"
  ))]
  TheParameterDoesNotExist = 355,
  #[strum(props(
    Description = "The data payload for a POST request must not be null. This status code is used to inform the client that the data BLOB should not be null for POST method"
  ))]
  DataBLOBShouldNotBeNullForPostMethod = 356,
}

impl ToU16 for ResponsesRedirectionCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesRedirectionCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}
