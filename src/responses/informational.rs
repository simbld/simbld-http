use crate::generate_responses_functions;
use crate::responses::CustomResponse;
use crate::traits::get_code_trait::GetCode;
use strum_macros::EnumIter;

generate_responses_functions! {
"Informational responses",
  ResponsesInformationalCodes,
  ContinueRequest => (100, "Continue", "The server has received the initial part of the request, the headers, and asks the client to continue request, proceed to send the body of the request, a POST request", 100, "Continue Request"),
  SwitchingProtocols => (101, "Switching Protocols", "The server is complying with a request to switch protocols, used in WebSocket connections", 101, "Switching Protocols"),
  Processing => (102, "Processing", "Indicates the server is processing the request but has not yet finished, used to prevent timeout errors in asynchronous operations, webdav RFC 2518", 102, "Processing"),
  EarlyHints => (103, "Early Hints", "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared", 103, "Early Hints"),
  ConnectionResetByPeer => (100, "Continue", "The connection was forcibly closed by a peer, possibly due to a protocol error, a timeout, or a network issue", 104, "Connection Reset By Peer"),
  NameNotResolved => (100, "Continue", "The server could not resolve the domain name provided in the request, indicating a DNS lookup failure, The requested hostname cannot be resolved to an IP address", 105, "Name Not Resolved"),
  NoResponse => (100, "Continue", "The server did not provide a response, possibly due to a timeout or a connection issue, The server didn’t send any response within the timeout period. This status code is not specified in any RFCs, but it is used in some scenarios to indicate that the server closed the connection without sending any response", 106, "No Response"),
  RetryWith => (100, "Continue", "The server indicates that the client should retry the request with appropriate changes or additional information, new or different credentials, use a different protocol or in a different location", 107, "Retry With"),
  ResponseIsStale => (100, "Continue", "The response returned by the server is stale and should be revalidated, indicating that the cached response is outdated or expired", 108, "Response Is Stale"),
  RevalidationFailed => (100, "Continue", "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired", 109, "Revalidation Failed"),
}

#[cfg(test)]
mod tests {
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesInformationalCodes;
    use crate::traits::tuple_traits::IntoTwoFieldsTuple;
    use serde_json::json;
    use serde_json::to_value;

    #[test]
    fn test_to_16_switching_protocols() {
        assert_eq!(ResponsesInformationalCodes::ContinueRequest.get_code(), 100);
        assert_eq!(ResponsesInformationalCodes::SwitchingProtocols.get_code(), 101);
        assert_eq!(ResponsesInformationalCodes::Processing.get_code(), 102);
        assert_eq!(ResponsesInformationalCodes::EarlyHints.get_code(), 103);
    }

    #[test]
    fn test_processing_codes_from_u16() {
        assert_eq!(
            ResponsesInformationalCodes::from_u16(102),
            Some(ResponsesInformationalCodes::Processing)
        );
        assert_eq!(
            ResponsesInformationalCodes::from_u16(103),
            Some(ResponsesInformationalCodes::EarlyHints)
        );
        assert_eq!(
            ResponsesInformationalCodes::from_u16(104),
            Some(ResponsesInformationalCodes::ConnectionResetByPeer)
        );
        assert_eq!(ResponsesInformationalCodes::from_u16(9999), None);
    }

    #[test]
    fn test_response_is_stale_codes_as_tuple() {
        let code = ResponsesInformationalCodes::ResponseIsStale;
        let tuple = UnifiedTuple {
            standard_code: 100,
            standard_name: "Continue",
            unified_description: "The response returned by the server is stale and should be revalidated, indicating that the cached response is outdated or expired",
            internal_code: Some(108),
            internal_name: Option::from("Response Is Stale"),
        };
        let code_as_tuple = code.as_tuple();
        assert_eq!(code_as_tuple, tuple);
    }

    #[test]
    fn test_revalidation_failed_codes_as_json() {
        let response_code = ResponsesInformationalCodes::RevalidationFailed;
        let json_result = response_code.as_json();
        let expected_json = json!({
            "type": "Informational responses",
            "details": {
                "standard http code": {
                    "code": 100,
                    "name": "Continue"
                },
                "description": "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired",
                "internal http code": {
                    "code": 109,
                    "name": "Revalidation Failed"
                }
        }});

        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_continue_request_codes_into_two_fields_tuple() {
        let response_code = ResponsesInformationalCodes::ContinueRequest;
        let tuple = response_code.into_two_fields_tuple();
        let json_result = to_value(&tuple).unwrap();

        let expected_json = json!({
            "code": 100,
            "name": "Continue"
        });

        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_continue_duplicate_standard_codes() {
        assert_eq!(
            ResponsesInformationalCodes::from_u16(108),
            Some(ResponsesInformationalCodes::ResponseIsStale)
        );
        assert_eq!(
            ResponsesInformationalCodes::from_u16(109),
            Some(ResponsesInformationalCodes::RevalidationFailed)
        );
    }
}
