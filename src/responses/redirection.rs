use crate::generate_responses_functions;
use crate::traits::to_u16_trait::ToU16;
use strum_macros::EnumIter;

generate_responses_functions! {
  "Redirection responses",
  ResponsesRedirectionCodes,
  MultipleChoices => (300, "Multiple Choices", "The request has more than one possible response. The user-agent or user should choose one of them. There is no standardized way of choosing one of the responses, but HTML links to the possibilities are recommended so the user can pick manually", 300, "Multiple Choices"),
  MovedPermanently => (301, "Moved Permanently", "The resource has been permanently moved to a new URI. Future requests should use the new URI. This status code is typically used for URL redirection", 301, "Moved Permanently"),
  Found => (302, "Found", "The resource is temporarily available at a different URI. The client should continue using the original URI for future requests. This status code is often used for URL redirection", 302, "Found"),
  SeeOther => (303, "See Other", "The response to the request can be found under another URI, and the client should use GET to retrieve it. This status code is used to direct the client to retrieve the resource from a different URI", 303, "See Other"),
  NotModified => (304, "Not Modified", "The resource has not been modified since the version specified in the request headers. This status code is used for caching purposes to reduce unnecessary network traffic", 304, "Not Modified"),
  UseProxy => (305, "Use Proxy", "The requested resource must be accessed through a specified proxy. This status code is used to inform the client that it should use a proxy server to access the resource", 305, "Use Proxy"),
  SwitchProxy => (306, "Unused", "Originally 'Switch Proxy', no longer used.", 306, "Switch Proxy"),
  TemporaryRedirect => (307, "Temporary Redirect", "The resource is temporarily located at a different URI. The client should use the same method to access it. This status code is used for temporary URL redirection", 307, "Temporary Redirect"),
  PermanentRedirect => (308, "Permanent Redirect", "The resource has been permanently moved to a new URI. The client should update its references. This status code is used for permanent URL redirection", 308, "Permanent Redirect"),
  TooManyRedirects => (300, "Multiple Choices", "The client has been redirected too many times, possibly causing a redirection loop. This status code is used to prevent infinite redirection loops", 310, "Too Many Redirects"),
  RedirectMethod => (300, "Multiple Choices", "The client should use a different method to access the resource. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST", 311, "Redirect Method"),
  Unassigned => (300, "Multiple Choices", "This code is currently unassigned and reserved for future use. It may be used for a new feature or status code in the future", 312, "Unassigned"),
  MovedPermanentlyRedirected => (300, "Multiple Choices", "The requested resource has been permanently moved to a new URI, but the client should continue to use the original URI. This status code is used for special cases of permanent redirection", 321, "Moved Permanently Redirected"),
  MovedTemporarilyRedirected => (300, "Multiple Choices", "The requested resource is temporarily available at a new URI but the client should not update its original URI. This status code is used for special cases of temporary redirection", 322, "Moved Temporarily Redirected"),
  SeeOtherRedirected => (300, "Multiple Choices", "The requested resource can be accessed at a different URI using the GET method. This status code is used to direct the client to retrieve the resource from a different URI using GET", 323, "See Other Redirected"),
  NotModifiedRedirected => (300, "Multiple Choices", "The requested resource has not been modified and can be retrieved from the cache. This status code is used for caching purposes to reduce unnecessary network traffic", 324, "Not Modified Redirected"),
  UseProxyRedirected => (300, "Multiple Choices", "The resource must be accessed through a proxy, and the proxy details are provided. This status code is used to inform the client that it should use a proxy server to access the resource", 325, "Use Proxy Redirected"),
  UnusedRedirected => (300, "Multiple Choices", "This status code is reserved and not used anymore. It was previously used for a proposed feature that was never implemented", 326, "Unused Redirected"),
  TemporaryRedirectRedirected => (300, "Multiple Choices", "The requested resource is temporarily located at a new URI. The client should not update its reference. This status code is used for special cases of temporary redirection", 327, "Temporary Redirect Redirected"),
  PermanentRedirected => (300, "Multiple Choices", "The resource has been permanently moved to a new URI, and future requests should use the new URI. This status code is used for special cases of permanent redirection", 328, "Permanent Redirected"),
  TooManyRedirectsRedirected => (300, "Multiple Choices", "The client has been redirected too many times during a redirection loop. This status code is used to prevent infinite redirection loops", 329, "Too Many Redirects Redirected"),
  RedirectMethodRedirected => (300, "Multiple Choices", "The redirection requires the client to use a different request method. This status code is used to inform the client that it should use a different HTTP method, such as GET or POST", 330, "Redirect Method Redirected"),
  UserNameOkPasswordNeeded => (300, "Multiple Choices", "The username is valid, but the client must provide a password to proceed. This status code is used for authentication purposes", 331, "User Name Ok Password Needed"),
  NoNeedAccountForLogin => (300, "Multiple Choices", "The requested resource does not require a user account for access. This status code is used to inform the client that no login is necessary", 332, "No Need Account For Login"),
  SessionKeyNotPresentInHeader => (300, "Multiple Choices", "The request is missing a session key in the header. This status code is used for session management purposes", 333, "Session Key Not Present In Header"),
  SessionKeyPresentAndNotDecryptableParsable => (300, "Multiple Choices", "The session key provided in the request cannot be decrypted or parsed. This status code is used for session management purposes", 334, "Session Key Present And Not Decryptable Parsable"),
  ServerIsUnwillingToProcessTheRequest => (300, "Multiple Choices", "The server refuses to process the request, often due to policy restrictions. This status code is used to inform the client that the server is unwilling to process the request", 335, "Server Is Unwilling To Process The Request"),
  ChallengeResponseAuthenticationOk => (300, "Multiple Choices", "Challenge-response authentication was successfully completed. This status code is used to inform the client that authentication was successful", 336, "Challenge Response Authentication Ok"),
  ChallengeResponseAuthenticationFailed => (300, "Multiple Choices", "Challenge-response authentication failed due to invalid credentials or other issues. This status code is used to inform the client that authentication failed", 337, "Challenge Response Authentication Failed"),
  LengthRequired => (300, "Multiple Choices", "The request did not specify the length of its content, which is required by the server. This status code is used to inform the client that the length is required", 342, "Length Required"),
  PreconditionFailed => (300, "Multiple Choices", "The server does not meet the preconditions set by the client in its request. This status code is used to inform the client that the preconditions failed", 343, "Precondition Failed"),
  RequestEntityTooLarge => (300, "Multiple Choices", "The request is larger than the server is willing or able to process. This status code is used to inform the client that the request entity is too large", 344, "Request Entity Too Large"),
  UnsupportedMediaType => (300, "Multiple Choices", "The media type of the request is not supported by the server. This status code is used to inform the client that the media type is unsupported", 346, "Unsupported Media Type"),
  RequestedRangeNotSatisfiable => (300, "Multiple Choices", "The server cannot supply the portion of the file requested by the client. This status code is used to inform the client that the requested range is not satisfiable", 347, "Requested Range Not Satisfiable"),
  ExpectationFailed => (300, "Multiple Choices", "The server cannot meet the requirements specified in the Expect header of the request. This status code is used to inform the client that the expectation failed", 348, "Expectation Failed"),
  ImATeapot => (300, "Multiple Choices", "A humorous response indicating the server is a teapot and refuses to brew coffee. This status code is used as an April Fools' joke", 349, "I'm A Teapot"),
  ErrorAccessingURL => (300, "Multiple Choices", "The server encountered an error while attempting to access the specified URL. This status code is used to inform the client that there was an error accessing the URL", 350, "Error Accessing URL"),
  TriggerNotFound => (300, "Multiple Choices", "The requested redirection trigger could not be found on the server. This status code is used to inform the client that the trigger was not found", 351, "Trigger Not Found"),
  AccessDenied => (300, "Multiple Choices", "The server refuses to fulfill the request due to access restrictions. This status code is used to inform the client that access is denied", 352, "Access Denied"),
  ConditionFailed => (300, "Multiple Choices", "A condition required to complete the redirection was not satisfied. This status code is used to inform the client that the condition failed", 353, "Condition Failed"),
  MandatoryParameterIsNull => (300, "Multiple Choices", "A required parameter for the request is missing or null. This status code is used to inform the client that a mandatory parameter is null", 354, "Mandatory Parameter Is Null"),
  TheParameterDoesNotExist => (300, "Multiple Choices", "A parameter specified in the request does not exist. This status code is used to inform the client that the parameter does not exist", 355, "The Parameter Does Not Exist"),
  DataBLOBShouldNotBeNullForPostMethod => (300, "Multiple Choices", "The data payload for a POST request must not be null. This status code is used to inform the client that the data BLOB should not be null for POST method", 356, "Data BLOB Should Not Be Null For Post Method"),
}

#[cfg(test)]
mod tests {
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesRedirectionCodes;
    use crate::traits::tuple_traits::IntoTwoFieldsTuple;
    use serde_json::json;
    use serde_json::to_value;

    #[test]
    fn test_redirection_codes_to_u16() {
        assert_eq!(ResponsesRedirectionCodes::MultipleChoices.to_u16(), 300);
        assert_eq!(ResponsesRedirectionCodes::MovedPermanently.to_u16(), 301);
        assert_eq!(ResponsesRedirectionCodes::SeeOther.to_u16(), 303);
        assert_eq!(ResponsesRedirectionCodes::TemporaryRedirect.to_u16(), 307);
    }

    #[test]
    fn test_redirection_codes_from_u16() {
        assert_eq!(
            ResponsesRedirectionCodes::from_u16(301),
            Some(ResponsesRedirectionCodes::MovedPermanently)
        );
        assert_eq!(
            ResponsesRedirectionCodes::from_u16(303),
            Some(ResponsesRedirectionCodes::SeeOther)
        );
        assert_eq!(
            ResponsesRedirectionCodes::from_u16(321),
            Some(ResponsesRedirectionCodes::MovedPermanentlyRedirected)
        );
        assert_eq!(ResponsesRedirectionCodes::from_u16(9999), None);
    }

    #[test]
    fn test_server_is_unwilling_to_process_the_request_codes_as_tuple() {
        let code = ResponsesRedirectionCodes::ServerIsUnwillingToProcessTheRequest;
        let tuple = UnifiedTuple {
            standard_code: 300,
            standard_name: "Multiple Choices",
            unified_description: "The server refuses to process the request, often due to policy restrictions. This status code is used to inform the client that the server is unwilling to process the request",
            internal_code: Some(335),
            internal_name: Some("Server Is Unwilling To Process The Request")
        };
        let code_as_tuple = code.as_tuple();
        assert_eq!(code_as_tuple, tuple);
    }

    #[test]
    fn test_redirection_codes_as_json() {
        let response_code = ResponsesRedirectionCodes::UserNameOkPasswordNeeded;
        let json_result = response_code.as_json();
        let expected_json = json!({
            "type": "Redirection responses",
            "details": {
                "standard http code": {
                    "code": 300,
                    "name": "Multiple Choices"
                },
                "description": "The username is valid, but the client must provide a password to proceed. This status code is used for authentication purposes",
                "internal http code": {
                    "code": 331,
                    "name": "User Name Ok Password Needed"
                }
            }
        });

        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_temporary_redirect_codes_into_two_fields_tuple() {
        let response_code = ResponsesRedirectionCodes::TemporaryRedirect;
        let tuple = response_code.into_two_fields_tuple();
        let json_result = to_value(&tuple).unwrap();

        let expected_json = json!({
            "code": "307",
            "name": "Temporary Redirect"
        });

        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_multiples_choices_standard_codes() {
        // These two codes have the same standard HTTP code (400) but different internal codes
        assert_eq!(
            ResponsesRedirectionCodes::from_u16(355),
            Some(ResponsesRedirectionCodes::TheParameterDoesNotExist)
        );
        assert_eq!(
            ResponsesRedirectionCodes::from_u16(356),
            Some(ResponsesRedirectionCodes::DataBLOBShouldNotBeNullForPostMethod)
        );
    }
}
