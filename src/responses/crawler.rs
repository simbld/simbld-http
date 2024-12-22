/// The code defines an enum `ResponsesCrawlerCodes` with associated descriptions and conversion methods in Rust.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
#[repr(u16)]

pub enum ResponsesCrawlerCodes {
  #[strum(props(Description = "Parsing error: unfinished header"))]
  ParsingErrorUnfinishedHeader = 700,
  #[strum(props(Description = "Parsing error: header"))]
  ParsingErrorHeader = 710,
  #[strum(props(Description = "Parsing error: missing HTTP code"))]
  ParsingErrorMissingHTTPCode = 720,
  #[strum(props(Description = "Parsing error: body"))]
  ParsingErrorBody = 730,
  #[strum(props(Description = "Excluded by robots.txt file"))]
  ExcludedByRobotsTxtFile = 740,
  #[strum(props(Description = "Robots temporarily unavailable"))]
  RobotsTemporarilyUnavailable = 741,
  #[strum(props(Description = "Excluded by definition of exploration space"))]
  ExcludedByDefinitionOfExplorationSpace = 760,
  #[strum(props(Description = "Not allowed by local exploration space"))]
  NotAllowedByLocalExplorationSpace = 761,
  #[strum(props(Description = "Incorrect protocol or non-standard system port"))]
  IncorrectProtocolOrNonStandardSystemPort = 770,
  #[strum(props(Description = "Excluded by file type exclusions"))]
  ExcludedByFileTypeExclusions = 780,
  #[strum(props(Description = "Invalid card - Not a physical card"))]
  InvalidCard = 781,
  #[strum(props(
    Description = "Cannot disable physical card OR Print card request already requested"
  ))]
  CannotDisablePhysicalCard = 782,
  #[strum(props(Description = "Invalid URL"))]
  InvalidURL = 786,
  #[strum(props(Description = "No index meta tag"))]
  NoIndexMetaTag = 2004,
  #[strum(props(Description = "Programmable redirection"))]
  ProgrammableRedirection = 3020,
  #[strum(props(Description = "Redirected to another URL"))]
  RedirectedToAnotherURL = 3021,
}

impl ToU16 for ResponsesCrawlerCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesCrawlerCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

impl Into<(u16, &'static str)> for ResponsesCrawlerCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description) // Tuple
  }
}

pub fn parsing_error_unfinished_header_tuple() -> (u16, &'static str) {
  (700, "Parsing error: unfinished header")
}

pub fn parsing_error_header_tuple() -> (u16, &'static str) {
  (710, "Parsing error: header")
}

pub fn parsing_error_missing_http_code_tuple() -> (u16, &'static str) {
  (720, "Parsing error: missing HTTP code")
}

pub fn parsing_error_body_tuple() -> (u16, &'static str) {
  (730, "Parsing error: body")
}

pub fn excluded_by_robots_txt_file_tuple() -> (u16, &'static str) {
  (740, "Excluded by robots.txt file")
}

pub fn robots_temporarily_unavailable_tuple() -> (u16, &'static str) {
  (741, "Robots temporarily unavailable")
}

pub fn excluded_by_definition_of_exploration_space_tuple() -> (u16, &'static str) {
  (760, "Excluded by definition of exploration space")
}

pub fn not_allowed_by_local_exploration_space_tuple() -> (u16, &'static str) {
  (761, "Not allowed by local exploration space")
}

pub fn incorrect_protocol_or_non_standard_system_port_tuple() -> (u16, &'static str) {
  (770, "Incorrect protocol or non-standard system port")
}

pub fn excluded_by_file_type_exclusions_tuple() -> (u16, &'static str) {
  (780, "Excluded by file type exclusions")
}

pub fn invalid_card_tuple() -> (u16, &'static str) {
  (781, "Invalid card - Not a physical card")
}

pub fn cannot_disable_physical_card_tuple() -> (u16, &'static str) {
  (782, "Cannot disable physical card OR Print card request already requested")
}

pub fn invalid_url_tuple() -> (u16, &'static str) {
  (786, "Invalid URL")
}

pub fn no_index_meta_tag_tuple() -> (u16, &'static str) {
  (2004, "No index meta tag")
}

pub fn programmable_redirection_tuple() -> (u16, &'static str) {
  (3020, "Programmable redirection")
}

pub fn redirected_to_another_url_tuple() -> (u16, &'static str) {
  (3021, "Redirected to another URL")
}

// Full response with status code and description encapsulated in a JSON response
pub fn parsing_error_unfinished_header() -> (u16, serde_json::Value) {
  let (code, desc) = parsing_error_unfinished_header_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn parsing_error_header() -> (u16, serde_json::Value) {
  let (code, desc) = parsing_error_header_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn parsing_error_missing_http_code() -> (u16, serde_json::Value) {
  let (code, desc) = parsing_error_missing_http_code_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn parsing_error_body() -> (u16, serde_json::Value) {
  let (code, desc) = parsing_error_body_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn excluded_by_robots_txt_file() -> (u16, serde_json::Value) {
  let (code, desc) = excluded_by_robots_txt_file_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn robots_temporarily_unavailable() -> (u16, serde_json::Value) {
  let (code, desc) = robots_temporarily_unavailable_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn excluded_by_definition_of_exploration_space() -> (u16, serde_json::Value) {
  let (code, desc) = excluded_by_definition_of_exploration_space_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn not_allowed_by_local_exploration_space() -> (u16, serde_json::Value) {
  let (code, desc) = not_allowed_by_local_exploration_space_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn incorrect_protocol_or_non_standard_system_port() -> (u16, serde_json::Value) {
  let (code, desc) = incorrect_protocol_or_non_standard_system_port_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn excluded_by_file_type_exclusions() -> (u16, serde_json::Value) {
  let (code, desc) = excluded_by_file_type_exclusions_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn invalid_card() -> (u16, serde_json::Value) {
  let (code, desc) = invalid_card_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn cannot_disable_physical_card() -> (u16, serde_json::Value) {
  let (code, desc) = cannot_disable_physical_card_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn invalid_url() -> (u16, serde_json::Value) {
  let (code, desc) = invalid_url_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn no_index_meta_tag() -> (u16, serde_json::Value) {
  let (code, desc) = no_index_meta_tag_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn programmable_redirection() -> (u16, serde_json::Value) {
  let (code, desc) = programmable_redirection_tuple();
  (code, json!({ "status": code, "description": desc }))
}

pub fn redirected_to_another_url() -> (u16, serde_json::Value) {
  let (code, desc) = redirected_to_another_url_tuple();
  (code, json!({ "status": code, "description": desc }))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_parsing_error_unfinished_header() {
    let response = ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader;
    let (code, description) = response.into();
    assert_eq!(code, 700);
    assert_eq!(description, "Parsing error: unfinished header");
  }

  #[test]
  fn test_to_u16_parsing_error_header() {
    let response = ResponsesCrawlerCodes::ParsingErrorHeader;
    let code = response.to_u16();
    assert_eq!(code, 710);
  }

  #[test]
  fn test_parsing_error_missing_http_code() {
    assert_eq!(parsing_error_missing_http_code_tuple(), (720, "Parsing error: missing HTTP code"));
  }

  #[test]
  fn test_from_u16_invalid_url() {
    let response = ResponsesCrawlerCodes::from_u16(786);
    assert_eq!(response, Some(ResponsesCrawlerCodes::InvalidURL));
  }

  #[test]
  fn test_no_index_meta_tag() {
    let (code, response) = no_index_meta_tag();
    assert_eq!(code, 2004);
    assert_eq!(response, json!({ "status": 2004, "description": "No index meta tag" }));
  }

  #[test]
  fn test_programmable_redirection() {
    let (code, response) = programmable_redirection_tuple();
    assert_eq!(code, 3020);
    assert_eq!(response, "Programmable redirection");
  }
}
