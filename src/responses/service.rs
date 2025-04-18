use crate::generate_responses_functions;
use crate::responses::CustomResponse;
use crate::traits::get_code_trait::GetCode;
use strum_macros::EnumIter;

generate_responses_functions! {
  "Service responses",
  ResponsesServiceCodes,
  ReadingError => (500, "Internal Server Error", "An error occurred while reading the response or data from the server", 611, "Reading Error"),
  ConnectionError => (500, "Internal Server Error", "A connection issue occurred, preventing successful communication with the server", 612, "Connection Error"),
  ReadingTimeExpired => (500, "Internal Server Error", "The reading operation exceeded the allowed time limit, resulting in a timeout", 613, "Reading Time Expired"),
  SSLHandshakeFailed => (500, "Internal Server Error", "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols", 614, "SSL Handshake Failed"),
  AnotherReadingError => (500, "Internal Server Error", "A generic error occurred while reading the response or data", 615, "Another Reading Error"),
  FBAAnomaly => (500, "Internal Server Error", "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input", 616, "FBA Anomaly"),
  CodingError => (500, "Internal Server Error", "An error in the implementation or logic caused the request to fail", 617, "Coding Error"),
  RedirectWithoutRedirectURL => (500, "Internal Server Error", "The server issued a redirect response but did not provide a valid redirect URL", 618, "Redirect Without Redirect URL"),
  DNSLookupFailed => (500, "Internal Server Error", "The DNS lookup for the specified domain failed, indicating a potential network or configuration issue", 680, "DNS Lookup Failed"),
  SyntacticallyIncorrectURL => (500, "Internal Server Error", "The provided URL is syntactically incorrect and cannot be processed", 690, "Syntactically Incorrect URL"),
  LostConnection => (500, "Internal Server Error", "The connection to the server was lost unexpectedly during communication", 691, "Lost Connection"),
  WriteTimeout => (500, "Internal Server Error", "The operation timed out while attempting to write data to the server", 692, "Write Timeout"),
  SelectionFailed => (500, "Internal Server Error", "The requested operation failed during a selection or matching process", 693, "Selection Failed"),
  WriteError => (500, "Internal Server Error", "An error occurred while attempting to write data to the destination", 694, "Write Error"),
  IncompleteBlockHeader => (500, "Internal Server Error", "A block header was incomplete or malformed, preventing further processing", 695, "Incomplete Block Header"),
  UnexpectedError => (500, "Internal Server Error", "An unexpected error occurred, often indicative of an unforeseen issue or bug", 699, "Unexpected Error"),
}

#[cfg(test)]
mod tests {
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesServiceCodes;
    use crate::traits::tuple_traits::IntoTwoFieldsTuple;
    use serde_json::json;
    use serde_json::to_value;

    #[test]
    fn test_service_codes_get_code() {
        assert_eq!(ResponsesServiceCodes::ReadingError.get_code(), 500);
        assert_eq!(ResponsesServiceCodes::ConnectionError.get_code(), 500);
        assert_eq!(ResponsesServiceCodes::ReadingTimeExpired.get_code(), 500);
        assert_eq!(ResponsesServiceCodes::SSLHandshakeFailed.get_code(), 500);
    }

    #[test]
    fn test_service_codes_from_u16() {
        assert_eq!(ResponsesServiceCodes::from_u16(611), Some(ResponsesServiceCodes::ReadingError));
        assert_eq!(
            ResponsesServiceCodes::from_u16(613),
            Some(ResponsesServiceCodes::ReadingTimeExpired)
        );
        assert_eq!(
            ResponsesServiceCodes::from_u16(614),
            Some(ResponsesServiceCodes::SSLHandshakeFailed)
        );
        assert_eq!(ResponsesServiceCodes::from_u16(9999), None);
    }

    #[test]
    fn test_connection_error_codes_as_tuple() {
        let code = ResponsesServiceCodes::ConnectionError;
        let tuple = UnifiedTuple {
            standard_code: 500,
            standard_name: "Internal Server Error",
            unified_description:
                "A connection issue occurred, preventing successful communication with the server",
            internal_code: Some(612),
            internal_name: Option::from("Connection Error"),
        };
        let code_as_tuple = code.as_tuple();
        assert_eq!(code_as_tuple, tuple);
    }

    #[test]
    fn test_service_codes_as_json() {
        let response_code = ResponsesServiceCodes::ReadingTimeExpired;
        let json_result = response_code.as_json();
        let expected_json = json!({
            "type": "Service responses",
            "details": {
                "standard http code": {
                    "code": 500,
                    "name": "Internal Server Error"
                },
                "description": "The reading operation exceeded the allowed time limit, resulting in a timeout",
                "internal http code": {
                    "code": 613,
                    "name": "Reading Time Expired"
                }
            }
        });

        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_service_codes_into_two_fields_tuple() {
        let response_code = ResponsesServiceCodes::IncompleteBlockHeader;
        let tuple = response_code.into_two_fields_tuple();
        let json_result = to_value(&tuple).unwrap();

        let expected_json = json!({
            "code": 500,
            "name": "Internal Server Error"
        });

        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_internal_server_error_duplicate_standard_codes() {
        // These two codes have the same standard HTTP code (400) but different internal codes
        assert_eq!(
            ResponsesServiceCodes::from_u16(695),
            Some(ResponsesServiceCodes::IncompleteBlockHeader)
        );
        assert_eq!(
            ResponsesServiceCodes::from_u16(699),
            Some(ResponsesServiceCodes::UnexpectedError)
        );
    }
}
