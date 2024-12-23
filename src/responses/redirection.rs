/// The above Rust code defines an enum representing HTTP responses redirection codes with associated descriptions and provides functions to retrieve specific code descriptions.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
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

/// implementation of a custom trait `ToU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “to_u16” method which converts a value from the enumeration into a “u16” type. Self accesses the value of the enum In the implementation, it calls the `into()` method to perform the conversion, which relies on the `Into<u16>` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl ToU16 for ResponsesRedirectionCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

/// implementation of a custom trait `FromU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “from_u16” method which converts a value from the enumeration into an `Option<Self>` type. The method uses the `try_from` method to perform the conversion, which relies on the `TryFromPrimitive` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl FromU16 for ResponsesRedirectionCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

/// implementation of a custom trait `Into` for the `ResponsesLocalApiCodes` enumeration. We provide an “into” method which converts a value from the enumeration into a tuple containing a `u16` and a `&'static str`. The method calls the `to_u16` method to get the status code and the `get_str` method to get the description. The `unwrap_or` method is used to provide a default value in case the description is not found. The method returns the tuple containing the status code and the description
impl Into<(u16, &'static str)> for ResponsesRedirectionCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

/// The functions returns a tuple containing an unsigned 16-bit integer and a static string indicating that the operation was approved with no further action required.
pub fn multiple_choices_tuple() -> (u16, &'static str) {
  (300, "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually")
}

pub fn moved_permanently_tuple() -> (u16, &'static str) {
  (301, "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection")
}

pub fn found_tuple() -> (u16, &'static str) {
  (302, "The resource is temporarily available at a different URI. The client should continue using the original URI for future requests. This status code is often used for URL redirection")
}

pub fn see_other_tuple() -> (u16, &'static str) {
  (303, "The response to the request can be found under another URI, and the client should use GET to retrieve it. This status code is used to direct the client to retrieve the resource from a different URI")
}

pub fn not_modified_tuple() -> (u16, &'static str) {
  (304, "The resource has not been modified since the version specified in the request headers. This status code is used for caching purposes to reduce unnecessary network traffic")
}

pub fn use_proxy_tuple() -> (u16, &'static str) {
  (305, "The requested resource must be accessed through a specified proxy. This status code is used to inform the client that it should use a proxy server to access the resource")
}

pub fn switch_proxy_tuple() -> (u16, &'static str) {
  (
    306,
    "Switch to the specified proxy. This is no longer used, and it is now considered deprecated",
  )
}

pub fn temporary_redirect_tuple() -> (u16, &'static str) {
  (307, "The resource is temporarily available at a different URI. The client should use the same method to access it. This status code is used for temporary URL redirection")
}

pub fn permanent_redirect_tuple() -> (u16, &'static str) {
  (308, "The resource has been permanently moved to a new URI. The client should update its references. This status code is used for permanent URL redirection")
}

pub fn too_many_redirects_tuple() -> (u16, &'static str) {
  (310, "The client has been redirected too many times, possibly causing a redirection loop. This status code is used to prevent infinite redirection loops")
}

pub fn redirect_method_tuple() -> (u16, &'static str) {
  (311, "The client should use a different method to access the resource. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST")
}

pub fn unassigned_tuple() -> (u16, &'static str) {
  (312, "This code is currently unassigned and reserved for future use. It may be used for a new feature or status code in the future")
}

pub fn moved_permanently_redirected_tuple() -> (u16, &'static str) {
  (321, "The requested resource has been permanently moved to a new URI, but the client should continue to use the original URI. This status code is used for special cases of permanent redirection")
}

pub fn moved_temporarily_redirected_tuple() -> (u16, &'static str) {
  (322, "The requested resource is temporarily available at a new URI but the client should not update its original URI. This status code is used for special cases of temporary redirection")
}

pub fn see_other_redirected_tuple() -> (u16, &'static str) {
  (323, "The requested resource can be accessed at a different URI using the GET method. This status code is used to direct the client to retrieve the resource from a different URI using GET")
}

pub fn not_modified_redirected_tuple() -> (u16, &'static str) {
  (324, "The requested resource has not been modified and can be retrieved from the cache. This status code is used for caching purposes to reduce unnecessary network traffic")
}

pub fn use_proxy_redirected_tuple() -> (u16, &'static str) {
  (325, "The resource must be accessed through a proxy, and the proxy details are provided. This status code is used to inform the client that it should use a proxy server to access the resource")
}

pub fn unused_redirected_tuple() -> (u16, &'static str) {
  (326, "This status code is reserved and not used anymore. It was previously used for a proposed feature that was never implemented")
}

pub fn temporary_redirect_redirected_tuple() -> (u16, &'static str) {
  (327, "The requested resource is temporarily located at a new URI. The client should not update its reference. This status code is used for special cases of temporary redirection")
}

pub fn permanent_redirected_tuple() -> (u16, &'static str) {
  (328, "The resource has been permanently moved to a new URI, and future requests should use the new URI. This status code is used for special cases of permanent redirection")
}

pub fn too_many_redirects_redirected_tuple() -> (u16, &'static str) {
  (329, "The client has been redirected too many times during a redirection loop. This status code is used to prevent infinite redirection loops")
}

pub fn redirect_method_redirected_tuple() -> (u16, &'static str) {
  (330, "The redirection requires the client to use a different request method. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST")
}

pub fn user_name_ok_password_needed_tuple() -> (u16, &'static str) {
  (331, "The username is valid, but the client must provide a password to proceed. This status code is used for authentication purposes")
}

pub fn no_need_account_for_login_tuple() -> (u16, &'static str) {
  (332, "The requested resource does not require a user account for access. This status code is used to inform the client that no login is necessary")
}

pub fn session_key_not_present_in_header_tuple() -> (u16, &'static str) {
  (333, "The request is missing a session key in the header. This status code is used for session management purposes")
}

pub fn session_key_present_and_not_decryptable_parsable_tuple() -> (u16, &'static str) {
  (334, "The session key provided in the request cannot be decrypted or parsed. This status code is used for session management purposes")
}

pub fn server_is_unwilling_to_process_the_request_tuple() -> (u16, &'static str) {
  (335, "The server refuses to process the request, often due to policy restrictions. This status code is used to inform the client that the server is unwilling to process the request")
}

pub fn challenge_response_authentication_ok_tuple() -> (u16, &'static str) {
  (336, "Challenge-response authentication was successfully completed. This status code is used to inform the client that authentication was successful")
}

pub fn challenge_response_authentication_failed_tuple() -> (u16, &'static str) {
  (337, "Challenge-response authentication failed due to invalid credentials or other issues. This status code is used to inform the client that authentication failed")
}

pub fn length_required_tuple() -> (u16, &'static str) {
  (342, "The request did not specify the length of its content, which is required by the server. This status code is used to inform the client that the length is required")
}

pub fn precondition_failed_tuple() -> (u16, &'static str) {
  (343, "The server does not meet the preconditions set by the client in its request. This status code is used to inform the client that the preconditions failed")
}

pub fn request_entity_too_large_tuple() -> (u16, &'static str) {
  (344, "The request is larger than the server is willing or able to process. This status code is used to inform the client that the request entity is too large")
}

pub fn unsupported_media_type_tuple() -> (u16, &'static str) {
  (346, "The media type of the request is not supported by the server. This status code is used to inform the client that the media type is unsupported")
}

pub fn requested_range_not_satisfiable_tuple() -> (u16, &'static str) {
  (347, "The server cannot supply the portion of the file requested by the client. This status code is used to inform the client that the requested range is not satisfiable")
}

pub fn expectation_failed_tuple() -> (u16, &'static str) {
  (348, "The server cannot meet the requirements specified in the Expect header of the request. This status code is used to inform the client that the expectation failed")
}

pub fn im_a_teapot_tuple() -> (u16, &'static str) {
  (349, "A humorous response indicating the server is a teapot and refuses to brew coffee. This status code is used as an April Fools' joke")
}

pub fn error_accessing_url_tuple() -> (u16, &'static str) {
  (350, "The server encountered an error while attempting to access the specified URL. This status code is used to inform the client that there was an error accessing the URL")
}

pub fn trigger_not_found_tuple() -> (u16, &'static str) {
  (351, "The requested redirection trigger could not be found on the server. This status code is used to inform the client that the trigger was not found")
}

pub fn access_denied_tuple() -> (u16, &'static str) {
  (352, "The server refuses to fulfill the request due to access restrictions. This status code is used to inform the client that access is denied")
}

pub fn condition_failed_tuple() -> (u16, &'static str) {
  (353, "A condition required to complete the redirection was not satisfied. This status code is used to inform the client that the condition failed")
}

pub fn mandatory_parameter_is_null_tuple() -> (u16, &'static str) {
  (354, "A required parameter for the request is missing or null. This status code is used to inform the client that a mandatory parameter is null")
}

pub fn the_parameter_does_not_exist_tuple() -> (u16, &'static str) {
  (355, "A parameter specified in the request does not exist. This status code is used to inform the client that the parameter does not exist")
}

pub fn data_blob_should_not_be_null_for_post_method_tuple() -> (u16, &'static str) {
  (356, "The data payload for a POST request must not be null. This status code is used to inform the client that the data BLOB should not be null for POST method")
}

/// The functions returns a tuple containing a status code and a JSON value with status and description fields.
pub fn multiple_choices() -> (u16, serde_json::Value) {
  let (code, desc) = multiple_choices_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn moved_permanently() -> (u16, serde_json::Value) {
  let (code, desc) = moved_permanently_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn found() -> (u16, serde_json::Value) {
  let (code, desc) = found_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn see_other() -> (u16, serde_json::Value) {
  let (code, desc) = see_other_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn not_modified() -> (u16, serde_json::Value) {
  let (code, desc) = not_modified_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn use_proxy() -> (u16, serde_json::Value) {
  let (code, desc) = use_proxy_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn switch_proxy() -> (u16, serde_json::Value) {
  let (code, desc) = switch_proxy_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn temporary_redirect() -> (u16, serde_json::Value) {
  let (code, desc) = temporary_redirect_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn permanent_redirect() -> (u16, serde_json::Value) {
  let (code, desc) = permanent_redirect_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn too_many_redirects() -> (u16, serde_json::Value) {
  let (code, desc) = too_many_redirects_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn redirect_method() -> (u16, serde_json::Value) {
  let (code, desc) = redirect_method_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn unassigned() -> (u16, serde_json::Value) {
  let (code, desc) = unassigned_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn moved_permanently_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = moved_permanently_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn moved_temporarily_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = moved_temporarily_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn see_other_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = see_other_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn not_modified_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = not_modified_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn use_proxy_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = use_proxy_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn unused_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = unused_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn temporary_redirect_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = temporary_redirect_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn permanent_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = permanent_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn too_many_redirects_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = too_many_redirects_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn redirect_method_redirected() -> (u16, serde_json::Value) {
  let (code, desc) = redirect_method_redirected_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn user_name_ok_password_needed() -> (u16, serde_json::Value) {
  let (code, desc) = user_name_ok_password_needed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn no_need_account_for_login() -> (u16, serde_json::Value) {
  let (code, desc) = no_need_account_for_login_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn session_key_not_present_in_header() -> (u16, serde_json::Value) {
  let (code, desc) = session_key_not_present_in_header_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn session_key_present_and_not_decryptable_parsable() -> (u16, serde_json::Value) {
  let (code, desc) = session_key_present_and_not_decryptable_parsable_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn server_is_unwilling_to_process_the_request() -> (u16, serde_json::Value) {
  let (code, desc) = server_is_unwilling_to_process_the_request_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn challenge_response_authentication_ok() -> (u16, serde_json::Value) {
  let (code, desc) = challenge_response_authentication_ok_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn challenge_response_authentication_failed() -> (u16, serde_json::Value) {
  let (code, desc) = challenge_response_authentication_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn length_required() -> (u16, serde_json::Value) {
  let (code, desc) = length_required_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn precondition_failed() -> (u16, serde_json::Value) {
  let (code, desc) = precondition_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn request_entity_too_large() -> (u16, serde_json::Value) {
  let (code, desc) = request_entity_too_large_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn unsupported_media_type() -> (u16, serde_json::Value) {
  let (code, desc) = unsupported_media_type_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn requested_range_not_satisfiable() -> (u16, serde_json::Value) {
  let (code, desc) = requested_range_not_satisfiable_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn expectation_failed() -> (u16, serde_json::Value) {
  let (code, desc) = expectation_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn im_a_teapot() -> (u16, serde_json::Value) {
  let (code, desc) = im_a_teapot_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn error_accessing_url() -> (u16, serde_json::Value) {
  let (code, desc) = error_accessing_url_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn trigger_not_found() -> (u16, serde_json::Value) {
  let (code, desc) = trigger_not_found_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn access_denied() -> (u16, serde_json::Value) {
  let (code, desc) = access_denied_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn condition_failed() -> (u16, serde_json::Value) {
  let (code, desc) = condition_failed_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn mandatory_parameter_is_null() -> (u16, serde_json::Value) {
  let (code, desc) = mandatory_parameter_is_null_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn the_parameter_does_not_exist() -> (u16, serde_json::Value) {
  let (code, desc) = the_parameter_does_not_exist_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn data_blob_should_not_be_null_for_post_method() -> (u16, serde_json::Value) {
  let (code, desc) = data_blob_should_not_be_null_for_post_method_tuple();
  (code, json!({ "status": code, "description": desc }))
}

// Unit tests
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
  fn test_moved_permanently_tuple() {
    assert_eq!(moved_permanently_tuple(), (301, "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection"));
  }

  #[test]
  fn test_from_u16_too_many_redirects() {
    let code = 310;
    let response = ResponsesRedirectionCodes::from_u16(code);
    assert_eq!(response, Some(ResponsesRedirectionCodes::TooManyRedirects));
  }

  #[test]
  fn test_multiple_choices() {
    let (code, response) = multiple_choices_tuple();
    assert_eq!(code, 300);
    assert_eq!(
      response,
      "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually"
    );
  }

  #[test]
  fn test_moved_permanently() {
    let (code, response) = moved_permanently();
    assert_eq! {code, 301};
    assert_eq!(
      response,
      json!({ "status": 301, "description": "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection" })
    );
  }
}
