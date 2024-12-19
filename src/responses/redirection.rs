/// The above Rust code defines an enum representing HTTP responses redirection codes with associated descriptions and provides functions to retrieve specific code descriptions.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
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

impl Into<(u16, &'static str)> for ResponsesRedirectionCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

pub fn multiple_choices() -> (u16, &'static str) {
  (300, "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually")
}

pub fn moved_permanently() -> (u16, &'static str) {
  (301, "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection")
}

pub fn found() -> (u16, &'static str) {
  (302, "The resource is temporarily available at a different URI. The client should continue using the original URI for future requests. This status code is often used for URL redirection")
}

pub fn see_other() -> (u16, &'static str) {
  (303, "The response to the request can be found under another URI, and the client should use GET to retrieve it. This status code is used to direct the client to retrieve the resource from a different URI")
}

pub fn not_modified() -> (u16, &'static str) {
  (304, "The resource has not been modified since the version specified in the request headers. This status code is used for caching purposes to reduce unnecessary network traffic")
}

pub fn use_proxy() -> (u16, &'static str) {
  (305, "The requested resource must be accessed through a specified proxy. This status code is used to inform the client that it should use a proxy server to access the resource")
}

pub fn switch_proxy() -> (u16, &'static str) {
  (
    306,
    "Switch to the specified proxy. This is no longer used, and it is now considered deprecated",
  )
}

pub fn temporary_redirect() -> (u16, &'static str) {
  (307, "The resource is temporarily available at a different URI. The client should use the same method to access it. This status code is used for temporary URL redirection")
}

pub fn permanent_redirect() -> (u16, &'static str) {
  (308, "The resource has been permanently moved to a new URI. The client should update its references. This status code is used for permanent URL redirection")
}

pub fn too_many_redirects() -> (u16, &'static str) {
  (310, "The client has been redirected too many times, possibly causing a redirection loop. This status code is used to prevent infinite redirection loops")
}

pub fn redirect_method() -> (u16, &'static str) {
  (311, "The client should use a different method to access the resource. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST")
}

pub fn unassigned() -> (u16, &'static str) {
  (312, "This code is currently unassigned and reserved for future use. It may be used for a new feature or status code in the future")
}

pub fn moved_permanently_redirected() -> (u16, &'static str) {
  (321, "The requested resource has been permanently moved to a new URI, but the client should continue to use the original URI. This status code is used for special cases of permanent redirection")
}

pub fn moved_temporarily_redirected() -> (u16, &'static str) {
  (322, "The requested resource is temporarily available at a new URI but the client should not update its original URI. This status code is used for special cases of temporary redirection")
}

pub fn see_other_redirected() -> (u16, &'static str) {
  (323, "The requested resource can be accessed at a different URI using the GET method. This status code is used to direct the client to retrieve the resource from a different URI using GET")
}

pub fn not_modified_redirected() -> (u16, &'static str) {
  (324, "The requested resource has not been modified and can be retrieved from the cache. This status code is used for caching purposes to reduce unnecessary network traffic")
}

pub fn use_proxy_redirected() -> (u16, &'static str) {
  (325, "The resource must be accessed through a proxy, and the proxy details are provided. This status code is used to inform the client that it should use a proxy server to access the resource")
}

pub fn unused_redirected() -> (u16, &'static str) {
  (326, "This status code is reserved and not used anymore. It was previously used for a proposed feature that was never implemented")
}

pub fn temporary_redirect_redirected() -> (u16, &'static str) {
  (327, "The requested resource is temporarily located at a new URI. The client should not update its reference. This status code is used for special cases of temporary redirection")
}

pub fn permanent_redirected() -> (u16, &'static str) {
  (328, "The resource has been permanently moved to a new URI, and future requests should use the new URI. This status code is used for special cases of permanent redirection")
}

pub fn too_many_redirects_redirected() -> (u16, &'static str) {
  (329, "The client has been redirected too many times during a redirection loop. This status code is used to prevent infinite redirection loops")
}

pub fn redirect_method_redirected() -> (u16, &'static str) {
  (330, "The redirection requires the client to use a different request method. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST")
}

pub fn user_name_ok_password_needed() -> (u16, &'static str) {
  (331, "The username is valid, but the client must provide a password to proceed. This status code is used for authentication purposes")
}

pub fn no_need_account_for_login() -> (u16, &'static str) {
  (332, "The requested resource does not require a user account for access. This status code is used to inform the client that no login is necessary")
}

pub fn session_key_not_present_in_header() -> (u16, &'static str) {
  (333, "The request is missing a session key in the header. This status code is used for session management purposes")
}

pub fn session_key_present_and_not_decryptable_parsable() -> (u16, &'static str) {
  (334, "The session key provided in the request cannot be decrypted or parsed. This status code is used for session management purposes")
}

pub fn server_is_unwilling_to_process_the_request() -> (u16, &'static str) {
  (335, "The server refuses to process the request, often due to policy restrictions. This status code is used to inform the client that the server is unwilling to process the request")
}

pub fn challenge_response_authentication_ok() -> (u16, &'static str) {
  (336, "Challenge-response authentication was successfully completed. This status code is used to inform the client that authentication was successful")
}

pub fn challenge_response_authentication_failed() -> (u16, &'static str) {
  (337, "Challenge-response authentication failed due to invalid credentials or other issues. This status code is used to inform the client that authentication failed")
}

pub fn length_required() -> (u16, &'static str) {
  (342, "The request did not specify the length of its content, which is required by the server. This status code is used to inform the client that the length is required")
}

pub fn precondition_failed() -> (u16, &'static str) {
  (343, "The server does not meet the preconditions set by the client in its request. This status code is used to inform the client that the preconditions failed")
}

pub fn request_entity_too_large() -> (u16, &'static str) {
  (344, "The request is larger than the server is willing or able to process. This status code is used to inform the client that the request entity is too large")
}

pub fn unsupported_media_type() -> (u16, &'static str) {
  (346, "The media type of the request is not supported by the server. This status code is used to inform the client that the media type is unsupported")
}

pub fn requested_range_not_satisfiable() -> (u16, &'static str) {
  (347, "The server cannot supply the portion of the file requested by the client. This status code is used to inform the client that the requested range is not satisfiable")
}

pub fn expectation_failed() -> (u16, &'static str) {
  (348, "The server cannot meet the requirements specified in the Expect header of the request. This status code is used to inform the client that the expectation failed")
}

pub fn im_a_teapot() -> (u16, &'static str) {
  (349, "A humorous response indicating the server is a teapot and refuses to brew coffee. This status code is used as an April Fools' joke")
}

pub fn error_accessing_url() -> (u16, &'static str) {
  (350, "The server encountered an error while attempting to access the specified URL. This status code is used to inform the client that there was an error accessing the URL")
}

pub fn trigger_not_found() -> (u16, &'static str) {
  (351, "The requested redirection trigger could not be found on the server. This status code is used to inform the client that the trigger was not found")
}

pub fn access_denied() -> (u16, &'static str) {
  (352, "The server refuses to fulfill the request due to access restrictions. This status code is used to inform the client that access is denied")
}

pub fn condition_failed() -> (u16, &'static str) {
  (353, "A condition required to complete the redirection was not satisfied. This status code is used to inform the client that the condition failed")
}

pub fn mandatory_parameter_is_null() -> (u16, &'static str) {
  (354, "A required parameter for the request is missing or null. This status code is used to inform the client that a mandatory parameter is null")
}

pub fn the_parameter_does_not_exist() -> (u16, &'static str) {
  (355, "A parameter specified in the request does not exist. This status code is used to inform the client that the parameter does not exist")
}

pub fn data_blob_should_not_be_null_for_post_method() -> (u16, &'static str) {
  (356, "The data payload for a POST request must not be null. This status code is used to inform the client that the data BLOB should not be null for POST method")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_redirection() {
    let response = ResponsesRedirectionCodes::MultipleChoices;
    let (code, description): (u16, &str) = response.into();
    assert_eq!(code, 300);
    assert_eq!(
      description,
      "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually"
    );
  }

  #[test]
  fn test_to_u16_success() {
    let response = ResponsesRedirectionCodes::Found;
    let code = response.to_u16();
    assert_eq!(code, 302);
  }

  #[test]
  fn test_moved_permanently() {
    assert_eq!(moved_permanently(), (301, "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection"));
  }

  #[test]
  fn test_from_u16_too_many_redirects() {
    let code = 310;
    let response = ResponsesRedirectionCodes::from_u16(code);
    assert_eq!(response, Some(ResponsesRedirectionCodes::TooManyRedirects));
  }
}
