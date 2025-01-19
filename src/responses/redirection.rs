use crate::helpers::generate_responses_functions::generate_responses_functions;
use serde_json::json;
/// Enum representing HTTP response status codes and descriptions.
/// Each variant corresponds to a specific HTTP status code.
///
/// * Example:
/// ```rust
///
/// use simbld_http::responses::redirection::ResponsesRedirectionCodes::MultipleChoices;
///
/// let response = MultipleChoices;
/// let json = response.as_json();
/// println!("{:?}", json);
/// ```

#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum ResponsesRedirectionCodes {
  MultipleChoices,
  MovedPermanently,
  Found,
  SeeOther,
  NotModified,
  UseProxy,
  SwitchProxy,
  TemporaryRedirect,
  PermanentRedirect,
  TooManyRedirects,
  RedirectMethod,
  Unassigned,
  MovedPermanentlyRedirected,
  MovedTemporarilyRedirected,
  SeeOtherRedirected,
  NotModifiedRedirected,
  UseProxyRedirected,
  UnusedRedirected,
  TemporaryRedirectRedirected,
  PermanentRedirected,
  TooManyRedirectsRedirected,
  RedirectMethodRedirected,
  UserNameOkPasswordNeeded,
  NoNeedAccountForLogin,
  SessionKeyNotPresentInHeader,
  SessionKeyPresentAndNotDecryptableParsable,
  ServerIsUnwillingToProcessTheRequest,
  ChallengeResponseAuthenticationOk,
  ChallengeResponseAuthenticationFailed,
  LengthRequired,
  PreconditionFailed,
  RequestEntityTooLarge,
  UnsupportedMediaType,
  RequestedRangeNotSatisfiable,
  ExpectationFailed,
  ImATeapot,
  ErrorAccessingURL,
  TriggerNotFound,
  AccessDenied,
  ConditionFailed,
  MandatoryParameterIsNull,
  TheParameterDoesNotExist,
  DataBLOBShouldNotBeNullForPostMethod,
}

generate_responses_functions! {
  MultipleChoices => (300, "Multiple Choices", "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually", 300, "Multiple Choices", 0, "", "", ""),
  MovedPermanently => (301, "Moved Permanently", "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection", 301, "Moved Permanently", 0, "", "", ""),
  Found => (302, "Found", "The resource is temporarily available at a different URI. The client should continue using the original URI for future requests. This status code is often used for URL redirection", 302, "Found", 0, "", "", ""),
  SeeOther => (303, "See Other", "The response to the request can be found under another URI, and the client should use GET to retrieve it. This status code is used to direct the client to retrieve the resource from a different URI", 303, "See Other", 0, "", "", ""),
  NotModified => (304, "Not Modified", "The resource has not been modified since the version specified in the request headers. This status code is used for caching purposes to reduce unnecessary network traffic", 304, "Not Modified", 0, "", "", ""),
  UseProxy => (305, "Use Proxy", "The requested resource must be accessed through a specified proxy. This status code is used to inform the client that it should use a proxy server to access the resource", 305, "Use Proxy", 0, "", "", ""),
  SwitchProxy => (306, "Unused", "Originally 'Switch Proxy', no longer used.", 306, "Switch Proxy", 0, "", "", ""),
  TemporaryRedirect => (307, "Temporary Redirect", "The resource is temporarily located at a different URI. The client should use the same method to access it. This status code is used for temporary URL redirection", 307, "Temporary Redirect", 0, "", "", ""),
  PermanentRedirect => (308, "Permanent Redirect", "The resource has been permanently moved to a new URI. The client should update its references. This status code is used for permanent URL redirection", 308, "Permanent Redirect", 0, "", "", ""),
  TooManyRedirects => (300, "Multiple Choices", "The client has been redirected too many times, possibly causing a redirection loop. This status code is used to prevent infinite redirection loops", 310, "Too Many Redirects", 0, "", "", ""),
  RedirectMethod => (300, "Multiple Choices", "The client should use a different method to access the resource. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST", 311, "Redirect Method", 0, "", "", ""),
  Unassigned => (300, "Multiple Choices", "This code is currently unassigned and reserved for future use. It may be used for a new feature or status code in the future", 312, "Unassigned", 0, "", "", ""),
  MovedPermanentlyRedirected => (300, "Multiple Choices", "The requested resource has been permanently moved to a new URI, but the client should continue to use the original URI. This status code is used for special cases of permanent redirection", 321, "Moved Permanently Redirected", 0, "", "", ""),
  MovedTemporarilyRedirected => (300, "Multiple Choices", "The requested resource is temporarily available at a new URI but the client should not update its original URI. This status code is used for special cases of temporary redirection", 322, "Moved Temporarily Redirected", 0, "", "", ""),
  SeeOtherRedirected => (300, "Multiple Choices", "The requested resource can be accessed at a different URI using the GET method. This status code is used to direct the client to retrieve the resource from a different URI using GET", 323, "See Other Redirected", 0, "", "", ""),
  NotModifiedRedirected => (300, "Multiple Choices", "The requested resource has not been modified and can be retrieved from the cache. This status code is used for caching purposes to reduce unnecessary network traffic", 324, "Not Modified Redirected", 0, "", "", ""),
  UseProxyRedirected => (300, "Multiple Choices", "The resource must be accessed through a proxy, and the proxy details are provided. This status code is used to inform the client that it should use a proxy server to access the resource", 325, "Use Proxy Redirected", 0, "", "", ""),
  UnusedRedirected => (300, "Multiple Choices", "This status code is reserved and not used anymore. It was previously used for a proposed feature that was never implemented", 326, "Unused Redirected", 0, "", "", ""),
  TemporaryRedirectRedirected => (300, "Multiple Choices", "The requested resource is temporarily located at a new URI. The client should not update its reference. This status code is used for special cases of temporary redirection", 327, "Temporary Redirect Redirected", 0, "", "", ""),
  PermanentRedirected => (300, "Multiple Choices", "The resource has been permanently moved to a new URI, and future requests should use the new URI. This status code is used for special cases of permanent redirection", 328, "Permanent Redirected", 0, "", "", ""),
  TooManyRedirectsRedirected => (300, "Multiple Choices", "The client has been redirected too many times during a redirection loop. This status code is used to prevent infinite redirection loops", 329, "Too Many Redirects Redirected", 0, "", "", ""),
  RedirectMethodRedirected => (300, "Multiple Choices", "The redirection requires the client to use a different request method. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST", 330, "Redirect Method Redirected", 0, "", "", ""),
  UserNameOkPasswordNeeded => (300, "Multiple Choices", "The username is valid, but the client must provide a password to proceed. This status code is used for authentication purposes", 331, "User Name Ok Password Needed", 0, "", "", ""),
  NoNeedAccountForLogin => (300, "Multiple Choices", "The requested resource does not require a user account for access. This status code is used to inform the client that no login is necessary", 332, "No Need Account For Login", 0, "", "", ""),
  SessionKeyNotPresentInHeader => (300, "Multiple Choices", "The request is missing a session key in the header. This status code is used for session management purposes", 333, "Session Key Not Present In Header", 0, "", "", ""),
  SessionKeyPresentAndNotDecryptableParsable => (300, "Multiple Choices", "The session key provided in the request cannot be decrypted or parsed. This status code is used for session management purposes", 334, "Session Key Present And Not Decryptable Parsable", 0, "", "", ""),
  ServerIsUnwillingToProcessTheRequest => (300, "Multiple Choices", "The server refuses to process the request, often due to policy restrictions. This status code is used to inform the client that the server is unwilling to process the request", 335, "Server Is Unwilling To Process The Request", 0, "", "", ""),
  ChallengeResponseAuthenticationOk => (300, "Multiple Choices", "Challenge-response authentication was successfully completed. This status code is used to inform the client that authentication was successful", 336, "Challenge Response Authentication Ok", 0, "", "", ""),
  ChallengeResponseAuthenticationFailed => (300, "Multiple Choices", "Challenge-response authentication failed due to invalid credentials or other issues. This status code is used to inform the client that authentication failed", 337, "Challenge Response Authentication Failed", 0, "", "", ""),
  LengthRequired => (300, "Multiple Choices", "The request did not specify the length of its content, which is required by the server. This status code is used to inform the client that the length is required", 342, "Length Required", 0, "", "", ""),
  PreconditionFailed => (300, "Multiple Choices", "The server does not meet the preconditions set by the client in its request. This status code is used to inform the client that the preconditions failed", 343, "Precondition Failed", 0, "", "", ""),
  RequestEntityTooLarge => (300, "Multiple Choices", "The request is larger than the server is willing or able to process. This status code is used to inform the client that the request entity is too large", 344, "Request Entity Too Large", 0, "", "", ""),
  UnsupportedMediaType => (300, "Multiple Choices", "The media type of the request is not supported by the server. This status code is used to inform the client that the media type is unsupported", 346, "Unsupported Media Type", 0, "", "", ""),
  RequestedRangeNotSatisfiable => (300, "Multiple Choices", "The server cannot supply the portion of the file requested by the client. This status code is used to inform the client that the requested range is not satisfiable", 347, "Requested Range Not Satisfiable", 0, "", "", ""),
  ExpectationFailed => (300, "Multiple Choices", "The server cannot meet the requirements specified in the Expect header of the request. This status code is used to inform the client that the expectation failed", 348, "Expectation Failed", 0, "", "", ""),
  ImATeapot => (300, "Multiple Choices", "A humorous response indicating the server is a teapot and refuses to brew coffee. This status code is used as an April Fools' joke", 349, "I'm A Teapot", 0, "", "", ""),
  ErrorAccessingURL => (300, "Multiple Choices", "The server encountered an error while attempting to access the specified URL. This status code is used to inform the client that there was an error accessing the URL", 350, "Error Accessing URL", 0, "", "", ""),
  TriggerNotFound => (300, "Multiple Choices", "The requested redirection trigger could not be found on the server. This status code is used to inform the client that the trigger was not found", 351, "Trigger Not Found", 0, "", "", ""),
  AccessDenied => (300, "Multiple Choices", "The server refuses to fulfill the request due to access restrictions. This status code is used to inform the client that access is denied", 352, "Access Denied", 0, "", "", ""),
  ConditionFailed => (300, "Multiple Choices", "A condition required to complete the redirection was not satisfied. This status code is used to inform the client that the condition failed", 353, "Condition Failed", 0, "", "", ""),
  MandatoryParameterIsNull => (300, "Multiple Choices", "A required parameter for the request is missing or null. This status code is used to inform the client that a mandatory parameter is null", 354, "Mandatory Parameter Is Null", 0, "", "", ""),
  TheParameterDoesNotExist => (300, "Multiple Choices", "A parameter specified in the request does not exist. This status code is used to inform the client that the parameter does not exist", 355, "The Parameter Does Not Exist", 0, "", "", ""),
  DataBLOBShouldNotBeNullForPostMethod => (300, "Multiple Choices", "The data payload for a POST request must not be null. This status code is used to inform the client that the data BLOB should not be null for POST method", 356, "Data BLOB Should Not Be Null For Post Method", 0, "", "", ""),
}

/// This file defines the `ResponsesRedirectionCodes` enum and provides five main functionalities:
/// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
/// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
/// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
/// 4. `as_json()` - converts the variant to a JSON object.
/// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::ResponsesRedirectionCodes;

  #[test]
  fn test_redirection_codes_to_u16() {
    let response = ResponsesRedirectionCodes::MultipleChoices;
    let code = response.to_u16();
    assert_eq!(code, 300);
  }

  #[test]
  fn test_redirection_codes_from_u16() {
    let status = ResponsesRedirectionCodes::from_u16(301);
    assert_eq!(status, Some(ResponsesRedirectionCodes::MovedPermanently));
  }

  #[test]
  fn test_redirection_codes_as_tuple() {
    let code = ResponsesRedirectionCodes::Found;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::NineFields(
        302,
        "Found",
        "The resource is temporarily available at a different URI. The client should continue using the original URI for future requests. This status code is often used for URL redirection",
        302,
        "Found",
        110,
        "req-13",
        "user-13",
        "status-13"
      )
    );
  }

  #[test]
  fn test_redirection_codes_as_json() {
    let code = ResponsesRedirectionCodes::SeeOther;
    let json_result = code.as_json();
    let expected_json = json!({
        "standard http code": {
            "code": 303,
            "name": "See Other"
        },
        "internal http code": {
            "code": 303,
            "name": "See Other"
        },
        "description": "The response to the request can be found under another URI, and the client should use GET to retrieve it. This status code is used to direct the client to retrieve the resource from a different URI",
        "metadata": {
            "meta1": 103,
            "meta2": "req-6",
            "meta3": "user-6",
            "meta4": "status-6"
        }
    });
    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_redirection_codes_into_tuple() {
    let code = ResponsesRedirectionCodes::TemporaryRedirect;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 307);
    assert_eq!(std_name, "Temporary Redirect");
  }
}
