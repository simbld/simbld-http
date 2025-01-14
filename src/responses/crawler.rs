// src/responses/crawler.rs

use crate::helpers::generate_responses_functions::generate_responses_functions;
// Si vous vouliez la macro avec metadata, alors => generate_responses_functions_with_metadata
use crate::helpers::generate_responses_functions::{ToU16, FromU16};
use crate::utils::response_tuple::ResponseTuple;
use strum_macros::{Display, EnumIter, EnumProperty};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
#[repr(u16)]
generate_responses_functions! {
  ResponsesCrawlerCodes,

  ParsingErrorUnfinishedHeader => (400, "Bad Request", "Parsing error: unfinished header.", 700,  "Parsing Error: Unfinished Header"),
  ParsingErrorHeader           => (400, "Bad Request", "Parsing error in the header.",     710,  "Parsing Error: Header"),
  ParsingErrorMissingHTTPCode  => (400, "Bad Request", "Parsing error: missing HTTP code.", 720,  "Parsing Error: Missing HTTP Code"),
  ParsingErrorBody             => (400, "Bad Request", "Parsing error in the body.",        730,  "Parsing Error: Body"),
  ExcludedByRobotsTxtFile      => (403, "Forbidden",   "Excluded by robots.txt file.",      740,  "Excluded by Robots.txt file"),
  RobotsTemporarilyUnavailable => (503, "Service Unavailable", "Robots temporarily unavailable.", 741, "Robots Temporarily Unavailable"),
  ExcludedByDefinitionOfExplorationSpace => (403, "Forbidden", "Excluded by definition of exploration space.", 760, "Excluded by Definition of Exploration Space"),
  NotAllowedByLocalExplorationSpace       => (403, "Forbidden", "Not allowed by local exploration space.", 761, "Not Allowed by Local Exploration Space"),
  IncorrectProtocolOrNonStandardSystemPort => (400, "Bad Request", "Incorrect protocol or non-standard port used.", 770, "Incorrect Protocol or Non-Standard System Port"),
  ExcludedByFileTypeExclusions => (403, "Forbidden",   "Excluded by file type exclusions.", 780,  "Excluded by File Type Exclusions"),
  InvalidCard                  => (400, "Bad Request", "Invalid card - Not a physical card?", 781, "Invalid Card"),
  CannotDisablePhysicalCard    => (400, "Bad Request", "Cannot disable physical card or already requested print.", 782, "Cannot Disable Physical Card"),
  InvalidURL                   => (400, "Bad Request", "Invalid URL encountered by crawler.", 786, "Invalid URL"),
  NoIndexMetaTag               => (400, "Bad Request", "No index meta tag found (non-standard).", 2004, "No Index Meta Tag"),
  ProgrammableRedirection      => (302, "Found",       "Programmable redirection used (non-standard).", 3020, "Programmable Redirection"),
  RedirectedToAnotherURL       => (302, "Found",       "Redirected to another URL (crawler-based).",   3021, "Redirected to Another URL"),
}

// =====================
//   TESTS
// =====================
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_to_u16() {
      let code = ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader;
      assert_eq!(code.to_u16(), 400);
  }

  #[test]
  fn test_from_u16() {
      let maybe_code = ResponsesCrawlerCodes::from_u16(400);
      assert_eq!(maybe_code, Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader));
  }

  #[test]
  fn test_as_tuple() {
      let code = ResponsesCrawlerCodes::ParsingErrorBody;
      let tuple = code.as_tuple();
      assert_eq!(tuple.std_code, 400);
      assert_eq!(tuple.std_name, "Bad Request");
      assert_eq!(tuple.desc, "Parsing error in the body.");
      assert_eq!(tuple.int_code, Some(730));
      assert_eq!(tuple.int_name, Some("Parsing Error: Body"));
  }

  #[test]
  fn test_as_json() {
      let code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
      let json_val = code.as_json();
      let expected = serde_json::json!({
        "standard http code": {
            "code": 503,
            "name": "Service Unavailable"
        },
        "internal http code": {
            "code": 741,
            "name": "Robots Temporarily Unavailable"
        },
        "description": "Robots temporarily unavailable."
      });
      assert_eq!(json_val, expected);
  }

  #[test]
  fn test_into_tuple() {
      let code = ResponsesCrawlerCodes::InvalidURL;
      let res: (u16, &'static str) = code.into();
      assert_eq!(res, (400, "Bad Request"));
  }
}
