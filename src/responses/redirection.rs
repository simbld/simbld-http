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
    (code, description)
  }
}

/// Functions return raw data as a tuple for further processing or formats containing HTTP status code, status message and description of various client error responses.
pub fn multiple_choices_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::MultipleChoices;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Multiple Choices", description)
}

pub fn moved_permanently_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::MovedPermanently;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Moved ", description)
}

pub fn found_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::Found;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Found", description)
}

pub fn see_other_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::SeeOther;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "See Other", description)
}

pub fn not_modified_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::NotModified;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Not Modified", description)
}

pub fn use_proxy_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::UseProxy;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Use Proxy", description)
}

pub fn switch_proxy_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::SwitchProxy;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Switch Proxy", description)
}

pub fn temporary_redirect_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::TemporaryRedirect;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Temporary Redirect", description)
}

pub fn permanent_redirect_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::PermanentRedirect;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Permanent Redirect", description)
}

pub fn too_many_redirects_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::TooManyRedirects;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Too Many Redirects", description)
}

pub fn redirect_method_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::RedirectMethod;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Redirect Method", description)
}

pub fn unassigned_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::Unassigned;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unassigned", description)
}

pub fn moved_permanently_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::MovedPermanentlyRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Moved Permanently Redirected", description)
}

pub fn moved_temporarily_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::MovedTemporarilyRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Moved Temporarily Redirected", description)
}

pub fn see_other_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::SeeOtherRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "See Other Redirected", description)
}

pub fn not_modified_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::NotModifiedRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Not Modified Redirected", description)
}

pub fn use_proxy_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::UseProxyRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Use Proxy Redirected", description)
}

pub fn unused_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::UnusedRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unused Redirected", description)
}

pub fn temporary_redirect_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::TemporaryRedirectRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Temporary Redirect Redirected", description)
}

pub fn permanent_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::PermanentRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Permanent Redirected", description)
}

pub fn too_many_redirects_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::TooManyRedirectsRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Too Many Redirects Redirected", description)
}

pub fn redirect_method_redirected_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::RedirectMethodRedirected;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Redirect Method Redirected", description)
}

pub fn user_name_ok_password_needed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::UserNameOkPasswordNeeded;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "User Name Ok Password Needed", description)
}

pub fn no_need_account_for_login_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::NoNeedAccountForLogin;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "No Need Account For Login", description)
}

pub fn session_key_not_present_in_header_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::SessionKeyNotPresentInHeader;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Session Key Not Present In Header", description)
}

pub fn session_key_present_and_not_decryptable_parsable_tuple() -> (u16, &'static str, &'static str)
{
  let code = ResponsesRedirectionCodes::SessionKeyPresentAndNotDecryptableParsable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Session Key Present And Not Decryptable Parsable", description)
}

pub fn server_is_unwilling_to_process_the_request_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::ServerIsUnwillingToProcessTheRequest;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Server Is Unwilling To Process The Request", description)
}

pub fn challenge_response_authentication_ok_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::ChallengeResponseAuthenticationOk;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Challenge Response Authentication Ok", description)
}

pub fn challenge_response_authentication_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::ChallengeResponseAuthenticationFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Challenge Response Authentication Failed", description)
}

pub fn length_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::LengthRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Length Required", description)
}

pub fn precondition_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::PreconditionFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Precondition Failed", description)
}

pub fn request_entity_too_large_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::RequestEntityTooLarge;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Request Entity Too Large", description)
}

pub fn unsupported_media_type_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::UnsupportedMediaType;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Unsupported Media Type", description)
}

pub fn requested_range_not_satisfiable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::RequestedRangeNotSatisfiable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Requested Range Not Satisfiable", description)
}

pub fn expectation_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::ExpectationFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Expectation Failed", description)
}

pub fn im_a_teapot_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::ImATeapot;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "I'm A Teapot", description)
}

pub fn error_accessing_url_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::ErrorAccessingURL;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Error Accessing URL", description)
}

pub fn trigger_not_found_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::TriggerNotFound;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Trigger Not Found", description)
}

pub fn access_denied_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::AccessDenied;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Access Denied", description)
}

pub fn condition_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::ConditionFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Condition Failed", description)
}

pub fn mandatory_parameter_is_null_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::MandatoryParameterIsNull;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Mandatory Parameter Is Null", description)
}

pub fn the_parameter_does_not_exist_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::TheParameterDoesNotExist;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "The Parameter Does Not Exist", description)
}

pub fn data_blob_should_not_be_null_for_post_method_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesRedirectionCodes::DataBLOBShouldNotBeNullForPostMethod;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Data BLOB Should Not Be Null For Post Method", description)
}

/// Functions return formatted data as JSON containing HTTP status code, status message and description of various informational responses.
pub fn multiple_choices() -> serde_json::Value {
  let (code, name, desc) = multiple_choices_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn moved_permanently() -> serde_json::Value {
  let (code, name, desc) = moved_permanently_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn found() -> serde_json::Value {
  let (code, name, desc) = found_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn see_other() -> serde_json::Value {
  let (code, name, desc) = see_other_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn not_modified() -> serde_json::Value {
  let (code, name, desc) = not_modified_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn use_proxy() -> serde_json::Value {
  let (code, name, desc) = use_proxy_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn switch_proxy() -> serde_json::Value {
  let (code, name, desc) = switch_proxy_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn temporary_redirect() -> serde_json::Value {
  let (code, name, desc) = temporary_redirect_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn permanent_redirect() -> serde_json::Value {
  let (code, name, desc) = permanent_redirect_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn too_many_redirects() -> serde_json::Value {
  let (code, name, desc) = too_many_redirects_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn redirect_method() -> serde_json::Value {
  let (code, name, desc) = redirect_method_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unassigned() -> serde_json::Value {
  let (code, name, desc) = unassigned_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn moved_permanently_redirected() -> serde_json::Value {
  let (code, name, desc) = moved_permanently_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn moved_temporarily_redirected() -> serde_json::Value {
  let (code, name, desc) = moved_temporarily_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn see_other_redirected() -> serde_json::Value {
  let (code, name, desc) = see_other_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn not_modified_redirected() -> serde_json::Value {
  let (code, name, desc) = not_modified_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn use_proxy_redirected() -> serde_json::Value {
  let (code, name, desc) = use_proxy_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unused_redirected() -> serde_json::Value {
  let (code, name, desc) = unused_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn temporary_redirect_redirected() -> serde_json::Value {
  let (code, name, desc) = temporary_redirect_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn permanent_redirected() -> serde_json::Value {
  let (code, name, desc) = permanent_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn too_many_redirects_redirected() -> serde_json::Value {
  let (code, name, desc) = too_many_redirects_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn redirect_method_redirected() -> serde_json::Value {
  let (code, name, desc) = redirect_method_redirected_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn user_name_ok_password_needed() -> serde_json::Value {
  let (code, name, desc) = user_name_ok_password_needed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn no_need_account_for_login() -> serde_json::Value {
  let (code, name, desc) = no_need_account_for_login_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn session_key_not_present_in_header() -> serde_json::Value {
  let (code, name, desc) = session_key_not_present_in_header_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn session_key_present_and_not_decryptable_parsable() -> serde_json::Value {
  let (code, name, desc) = session_key_present_and_not_decryptable_parsable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn server_is_unwilling_to_process_the_request() -> serde_json::Value {
  let (code, name, desc) = server_is_unwilling_to_process_the_request_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn challenge_response_authentication_ok() -> serde_json::Value {
  let (code, name, desc) = challenge_response_authentication_ok_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn challenge_response_authentication_failed() -> serde_json::Value {
  let (code, name, desc) = challenge_response_authentication_failed_tuple();
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

pub fn request_entity_too_large() -> serde_json::Value {
  let (code, name, desc) = request_entity_too_large_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn unsupported_media_type() -> serde_json::Value {
  let (code, name, desc) = unsupported_media_type_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn requested_range_not_satisfiable() -> serde_json::Value {
  let (code, name, desc) = requested_range_not_satisfiable_tuple();
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

pub fn error_accessing_url() -> serde_json::Value {
  let (code, name, desc) = error_accessing_url_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn trigger_not_found() -> serde_json::Value {
  let (code, name, desc) = trigger_not_found_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn access_denied() -> serde_json::Value {
  let (code, name, desc) = access_denied_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn condition_failed() -> serde_json::Value {
  let (code, name, desc) = condition_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn mandatory_parameter_is_null() -> serde_json::Value {
  let (code, name, desc) = mandatory_parameter_is_null_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn the_parameter_does_not_exist() -> serde_json::Value {
  let (code, name, desc) = the_parameter_does_not_exist_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn data_blob_should_not_be_null_for_post_method() -> serde_json::Value {
  let (code, name, desc) = data_blob_should_not_be_null_for_post_method_tuple();
  json!({ "status": code, "name": name, "description": desc })
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
    assert_eq!(moved_permanently_tuple(), (301, "Moved Permanently", "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection"));
  }

  #[test]
  fn test_from_u16_too_many_redirects() {
    let code = 310;
    let response = ResponsesRedirectionCodes::from_u16(code);
    assert_eq!(response, Some(ResponsesRedirectionCodes::TooManyRedirects));
  }

  #[test]
  fn test_multiple_choices() {
    let (code, name, description) = multiple_choices_tuple();
    assert_eq!(code, 300);
    assert_eq!(name, "Multiple Choices");
    assert_eq!(
      description,
      "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually"
    );
  }

  #[test]
  fn test_moved_permanently() {
    let response = moved_permanently();
    assert_eq!(
      response,
      json!({ "status": 301, "name": "Moved Permanently", "description": "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection" })
    );
  }
}
