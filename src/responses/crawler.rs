use crate::generate_responses_functions;
generate_responses_functions! {
// Converts the `ResponsesCrawlerCodes` variant to a tuple representation.
// The tuple contains the standard HTTP code, standard name, description, internal code, and internal name.
//
// * Example:
// ```rust
// use simbld_http::responses::ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader;
// let response = ParsingErrorUnfinishedHeader;
// let tuple = response.as_tuple();
// println!("{:?}", tuple);
// ```
//
// The output will be `(400, "Bad Request", "Parsing error: unfinished header.", 700, "Parsing Error: Unfinished Header")`.
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

// This file defines the `ResponsesCrawlerCodes` enum and provides five main functionalities:
// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
// 4. `as_json()` - converts the variant to a JSON object.
// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::UnifiedTuple;
  use serde_json::json;

  #[test]
  fn test_crawler_codes_to_u16() {
    let response = ResponsesCrawlerCodes::ParsingErrorHeader;
    assert_eq!(response.to_u16(), 400);
  }

  #[test]
  fn test_crawler_codes_from_u16() {
    let status = ResponsesCrawlerCodes::from_u16(400);
    assert_eq!(status, Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader));
  }

  #[test]
  fn test_crawler_codes_as_tuple() {
    let code = ResponsesCrawlerCodes::InvalidURL;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::FiveFields(
        400,
        "Bad Request",
        "Invalid URL encountered by crawler.",
        786,
        "Invalid URL"
      )
    );
  }

  #[test]
  fn test_crawler_codes_as_json() {
    let code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
    let json_result = code.as_json();
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
    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_crawler_codes_into_tuple() {
    let code = ResponsesCrawlerCodes::ProgrammableRedirection;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 302);
    assert_eq!(std_name, "Found");
  }
}
