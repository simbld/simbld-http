use crate::helpers::generate_responses_functions::generate_responses_functions;
use serde_json::json;
/// Enum representing HTTP response status codes and descriptions.
/// Each variant corresponds to a specific HTTP status code.
///
/// * Example:
/// ```rust
/// use simbld_http::responses::crawler::ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader;
/// let response = ParsingErrorUnfinishedHeader;
/// let json = response.as_json();
/// println!("{:?}", json);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum ResponsesCrawlerCodes {
  ParsingErrorUnfinishedHeader,
  ParsingErrorHeader,
  ParsingErrorMissingHTTPCode,
  ParsingErrorBody,
  ExcludedByRobotsTxtFile,
  RobotsTemporarilyUnavailable,
  ExcludedByDefinitionOfExplorationSpace,
  NotAllowedByLocalExplorationSpace,
  IncorrectProtocolOrNonStandardSystemPort,
  ExcludedByFileTypeExclusions,
  InvalidCard,
  CannotDisablePhysicalCard,
  InvalidURL,
  NoIndexMetaTag,
  ProgrammableRedirection,
  RedirectedToAnotherURL,
}

generate_responses_functions! {
  ParsingErrorUnfinishedHeader => (400, "Bad Request", "Parsing error: unfinished header.", 700, "Parsing Error: Unfinished Header", 123, "req-1", "user-1", "status-1", 0, "", "", ""),
  ParsingErrorHeader => (400, "Bad Request", "Parsing error in the header.", 710, "Parsing Error: Header", 456, "req-2", "user-2", "status-2", 0, "", "", ""),
  ParsingErrorMissingHTTPCode => (400, "Bad Request", "Parsing error: missing HTTP code.", 720, "Parsing Error: Missing HTTP Code", 789, "req-3", "user-3", "status-3", 0, "", "", ""),
  ParsingErrorBody => (400, "Bad Request", "Parsing error in the body.", 730, "Parsing Error: Body", 101, "req-4", "user-4", "status-4", 0, "", "", ""),
  ExcludedByRobotsTxtFile => (403, "Forbidden", "Excluded by robots.txt file.", 740, "Excluded by Robots.txt file", 102, "req-5", "user-5", "status-5", 0, "", "", ""),
  RobotsTemporarilyUnavailable => (503, "Service Unavailable", "Robots temporarily unavailable.", 741, "Robots Temporarily Unavailable", 103, "req-6", "user-6", "status-6", 0, "", "", ""),
  ExcludedByDefinitionOfExplorationSpace => (403, "Forbidden", "Excluded by definition of exploration space.", 760, "Excluded by Definition of Exploration Space", 104, "req-7", "user-7", "status-7", 0, "", "", ""),
  NotAllowedByLocalExplorationSpace => (403, "Forbidden", "Not allowed by local exploration space.", 761, "Not Allowed by Local Exploration Space", 105, "req-8", "user-8", "status-8", 0, "", "", ""),
  IncorrectProtocolOrNonStandardSystemPort => (400, "Bad Request", "Incorrect protocol or non-standard port used.", 770, "Incorrect Protocol or Non-Standard System Port", 106, "req-9", "user-9", "status-9", 0, "", "", ""),
  ExcludedByFileTypeExclusions => (403, "Forbidden", "Excluded by file type exclusions.", 780, "Excluded by File Type Exclusions", 107, "req-10", "user-10", "status-10", 0, "", "", ""),
  InvalidCard => (400, "Bad Request", "Invalid card - Not a physical card?", 781, "Invalid Card", 108, "req-11", "user-11", "status-11", 0, "", "", ""),
  CannotDisablePhysicalCard => (400, "Bad Request", "Cannot disable physical card or already requested print.", 782, "Cannot Disable Physical Card", 109, "req-12", "user-12", "status-12", 0, "", "", ""),
  InvalidURL => (400, "Bad Request", "Invalid URL encountered by crawler.", 786, "Invalid URL", 110, "req-13", "user-13", "status-13", 0, "", "", ""),
  NoIndexMetaTag => (400, "Bad Request", "No index meta tag found (non-standard).", 2004, "No Index Meta Tag", 111, "req-14", "user-14", "status-14", 0, "", "", ""),
  ProgrammableRedirection => (302, "Found", "Programmable redirection used (non-standard).", 3020, "Programmable Redirection", 112, "req-15", "user-15", "status-15", 0, "", "", ""),
  RedirectedToAnotherURL => (302, "Found", "Redirected to another URL (crawler-based).", 3021, "Redirected to Another URL", 113, "req-16", "user-16", "status-16", 0, "", "", ""),
}

/// This file defines the `ResponsesCrawlerCodes` enum and provides five main functionalities:
/// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
/// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
/// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
/// 4. `as_json()` - converts the variant to a JSON object.
/// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::ResponsesCrawlerCodes;

  #[test]
  fn test_crawler_codes_to_u16() {
    let response = ResponsesCrawlerCodes::ParsingErrorHeader;
    let code = response.to_u16();
    assert_eq!(code, 400);
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
      UnifiedTuple::NineFields(
        400,
        "Bad Request",
        "Invalid URL encountered by crawler.",
        786,
        "Invalid URL",
        110,
        "req-13",
        "user-13",
        "status-13"
      )
    );
  }

  #[test]
  fn test_crawler_codes_as_json() {
    let code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
    let json_result = code.as_json();
    let expected_json = json!({
        "standard http code": {
            "code": 503,
            "name": "Service Unavailable"
        },
        "internal http code": {
            "code": 741,
            "name": "Robots Temporarily Unavailable"
        },
        "description": "Robots temporarily unavailable.",
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
  fn test_crawler_codes_into_tuple() {
    let code = ResponsesCrawlerCodes::ProgrammableRedirection;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 302);
    assert_eq!(std_name, "Found");
  }
}
