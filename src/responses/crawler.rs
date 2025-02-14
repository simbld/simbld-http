use crate::generate_responses_functions;

generate_responses_functions! {
"",
    ResponsesCrawlerCodes,
    ParsingErrorUnfinishedHeader => (400, "Bad Request", "Parsing error: unfinished header.", 700, "Parsing Error: Unfinished Header"),
    ParsingErrorHeader => (400, "Bad Request", "Parsing error in the header.", 710, "Parsing Error: Header"),
    ParsingErrorMissingHTTPCode => (400, "Bad Request", "Parsing error: missing HTTP code.", 720, "Parsing Error: Missing HTTP Code"),
    ParsingErrorBody => (400, "Bad Request", "Parsing error in the body.", 730, "Parsing Error: Body"),
    ExcludedByRobotsTxtFile => (403, "Forbidden", "Excluded by robots.txt file.", 740, "Excluded by Robots.txt file"),
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
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesCrawlerCodes;
    use serde_json::json;
    
    #[test]
    fn test_crawler_codes_to_u16() {
        assert_eq!(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader.to_u16(), 700);
        assert_eq!(ResponsesCrawlerCodes::ParsingErrorHeader.to_u16(), 710);
        assert_eq!(ResponsesCrawlerCodes::InvalidURL.to_u16(), 786);
        assert_eq!(ResponsesCrawlerCodes::ProgrammableRedirection.to_u16(), 3020);
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
            code: 400,
            name: "Bad Request",
            description: "Parsing error: missing HTTP code.",
            internal_code: Some(720),
            internal_name: Option::from("Parsing Error: Missing HTTP Code"),
        };
        let code_as_tuple = code.as_tuple();
        assert_eq!(code_as_tuple, tuple);
    }

    #[test]
    fn test_as_json() {
        let response_code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
        let result = response_code.as_json();
        let expected_json = json!({
            "standard_http_code": {
                "code": 503,
                "name": "Service Unavailable"
            },
            "internal_http_code": {
                "code": 741,
                "name": "Robots Temporarily Unavailable"
            },
            "description": "Robots temporarily unavailable."
        });
        assert_eq!(result, expected_json);
    }

    #[test]
    fn test_into_tuple() {
        let (std_code, std_name): (u16, &'static str) =
            ResponsesCrawlerCodes::ProgrammableRedirection.into();
        assert_eq!(std_code, 302);
        assert_eq!(std_name, "Found");
    }
}

#[test]
fn test_duplicate_standard_codes() {
    // Ces deux codes ont le même code HTTP standard (400) mais des codes internes différents
    assert_eq!(
        ResponsesCrawlerCodes::from_u16(700),
        Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader)
    );
    assert_eq!(
        ResponsesCrawlerCodes::from_u16(710),
        Some(ResponsesCrawlerCodes::ParsingErrorHeader)
    );
}
