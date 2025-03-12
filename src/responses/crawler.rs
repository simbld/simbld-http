use crate::generate_responses_functions;
use crate::helpers::traits::to_u16_trait::ToU16;
use strum_macros::EnumIter;

generate_responses_functions! {
"Crawler responses",
    ResponsesCrawlerCodes,
    ParsingErrorUnfinishedHeader => (400, "Bad Request", "Parsing error: unfinished header.", 700, "Parsing Error Unfinished Header"),
    ParsingErrorHeader => (400, "Bad Request", "Parsing error in the header.", 710, "Parsing Error: Header"),
    ParsingErrorMissingHTTPCode => (400, "Bad Request", "Parsing error: missing HTTP code.", 720, "Parsing Error: Missing HTTP Code"),
    ParsingErrorBody => (400, "Bad Request", "Parsing error in the body.", 730, "Parsing Error: Body"),
    ExcludedByRobotsTxtFile => (403, "Forbidden", "Excluded by robots.txt file", 740, "Excluded by Robots.txt file"),
    RobotsTemporarilyUnavailable => (503, "Service Unavailable", "Robots temporarily unavailable.", 741, "Robots Temporarily Unavailable"),
    ExcludedByDefinitionOfExplorationSpace => (403, "Forbidden", "Excluded by definition of exploration space.", 760, "Excluded by Definition of Exploration Space"),
    NotAllowedByLocalExplorationSpace => (403, "Forbidden", "Not allowed by local exploration space.", 761, "Not Allowed by Local Exploration Space"),
    IncorrectProtocolOrNonStandardSystemPort => (400, "Bad Request", "Incorrect protocol or non-standard port used.", 770, "Incorrect Protocol or Non-Standard System Port"),
    ExcludedByFileTypeExclusions => (403, "Forbidden", "Excluded by file type exclusions.", 780, "Excluded by File Type Exclusions"),
    InvalidCard => (400, "Bad Request", "Invalid card - Not a physical card?", 781, "Invalid Card"),
    CannotDisablePhysicalCard => (400, "Bad Request", "Cannot disable physical card or already requested print.", 782, "Cannot Disable Physical Card"),
    InvalidURL => (400, "Bad Request", "Invalid URL encountered by crawler.", 786, "Invalid URL"),
    NoIndexMetaTag => (400, "Bad Request", "No index meta tag found (non-standard).", 2004, "No Index Meta Tag"),
    ProgrammableRedirection => (302, "Found", "Programmable redirection used (non-standard).", 3020, "Programmable Redirection"),
    RedirectedToAnotherURL => (302, "Found", "Redirected to another URL (crawler-based).", 3021, "Redirected to Another URL"),
}

#[cfg(test)]
mod tests {
    use crate::helpers::traits::tuple_traits::IntoTwoFieldsTuple;
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesCrawlerCodes;
    use serde_json::{json, to_value};

    #[test]
    fn test_crawler_codes_to_u16() {
        assert_eq!(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader.to_u16(), 400);
        assert_eq!(ResponsesCrawlerCodes::ParsingErrorHeader.to_u16(), 400);
        assert_eq!(ResponsesCrawlerCodes::InvalidURL.to_u16(), 400);
        assert_eq!(ResponsesCrawlerCodes::ProgrammableRedirection.to_u16(), 302);
    }

    #[test]
    fn test_crawler_codes_from_u16() {
        assert_eq!(
            ResponsesCrawlerCodes::from_u16(700),
            Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader)
        );
        assert_eq!(
            ResponsesCrawlerCodes::from_u16(710),
            Some(ResponsesCrawlerCodes::ParsingErrorHeader)
        );
        assert_eq!(ResponsesCrawlerCodes::from_u16(786), Some(ResponsesCrawlerCodes::InvalidURL));
        assert_eq!(ResponsesCrawlerCodes::from_u16(9999), None);
    }

    #[test]
    fn test_parsing_error_missing_as_tuple() {
        let code = ResponsesCrawlerCodes::ParsingErrorMissingHTTPCode;
        let tuple = UnifiedTuple {
            standard_code: 400,
            standard_name: "Bad Request",
            unified_description: "Parsing error: missing HTTP code.",
            internal_code: Some(720),
            internal_name: Option::from("Parsing Error: Missing HTTP Code"),
        };
        let code_as_tuple = code.as_tuple();
        assert_eq!(code_as_tuple, tuple);
    }

    #[test]
    fn test_robots_temporarily_unavailable_as_json() {
        let response_code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
        let json_result = response_code.as_json();
        let expected_json = serde_json::json!({
            "type": "Crawler responses",
            "details": {
                "standard http code": {
                    "code": 503,
                    "name": "Service Unavailable"
                },
                "description": "Robots temporarily unavailable.",
                "internal http code": {
                    "code": 741,
                    "name": "Robots Temporarily Unavailable"
                }
            }
        });
        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_found_into_two_fields_tuple() {
        let response_code = ResponsesCrawlerCodes::RedirectedToAnotherURL;
        let tuple = response_code.into_two_fields_tuple();
        let json_result = to_value(&tuple).unwrap();

        let expected_json = json!({
            "code": 302,
            "name": "Found"
        });

        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_bad_request_duplicate_standard_codes() {
        // These two codes have the same standard HTTP code (400) but different internal codes
        assert_eq!(
            ResponsesCrawlerCodes::from_u16(700),
            Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader)
        );
        assert_eq!(
            ResponsesCrawlerCodes::from_u16(710),
            Some(ResponsesCrawlerCodes::ParsingErrorHeader)
        );
    }
}
