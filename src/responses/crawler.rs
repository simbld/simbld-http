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

/// implementation of a custom trait `ToU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “to_u16” method which converts a value from the enumeration into a “u16” type. Self accesses the value of the enum In the implementation, it calls the `into()` method to perform the conversion, which relies on the `Into<u16>` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl ToU16 for ResponsesCrawlerCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

/// implementation of a custom trait `FromU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “from_u16” method which converts a value from the enumeration into an `Option<Self>` type. The method uses the `try_from` method to perform the conversion, which relies on the `TryFromPrimitive` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl FromU16 for ResponsesCrawlerCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

/// implementation of a custom trait `Into` for the `ResponsesLocalApiCodes` enumeration. We provide an “into” method which converts a value from the enumeration into a tuple containing a `u16` and a `&'static str`. The method calls the `to_u16` method to get the status code and the `get_str` method to get the description. The `unwrap_or` method is used to provide a default value in case the description is not found. The method returns the tuple containing the status code and the description
impl Into<(u16, &'static str)> for ResponsesCrawlerCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description)
  }
}

/// Functions return raw data as a tuple for further processing or formats containing HTTP status code, status message and description of various client error responses.
pub fn parsing_error_unfinished_header_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Parsing Error: Unfinished Header", description)
}
pub fn parsing_error_header_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ParsingErrorHeader;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Parsing Error: Header", description)
}

pub fn parsing_error_missing_http_code_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ParsingErrorMissingHTTPCode;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Parsing Error: Missing HTTP Code", description)
}

pub fn parsing_error_body_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ParsingErrorBody;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Parsing Error: Body", description)
}

pub fn excluded_by_robots_txt_file_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ExcludedByRobotsTxtFile;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Excluded by Robots.txt file", description)
}

pub fn robots_temporarily_unavailable_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Robots Temporarily Unavailable", description)
}

pub fn excluded_by_definition_of_exploration_space_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ExcludedByDefinitionOfExplorationSpace;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Excluded by Definition of Exploration Space", description)
}

pub fn not_allowed_by_local_exploration_space_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::NotAllowedByLocalExplorationSpace;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Not Allowed by Local Exploration Space", description)
}

pub fn incorrect_protocol_or_non_standard_system_port_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::IncorrectProtocolOrNonStandardSystemPort;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Incorrect Protocol or Non-Standard System Port", description)
}

pub fn excluded_by_file_type_exclusions_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ExcludedByFileTypeExclusions;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Excluded by File Type Exclusions", description)
}

pub fn invalid_card_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::InvalidCard;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card", description)
}

pub fn cannot_disable_physical_card_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::CannotDisablePhysicalCard;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Cannot Disable Physical Card", description)
}

pub fn invalid_url_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::InvalidURL;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid URL", description)
}

pub fn no_index_meta_tag_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::NoIndexMetaTag;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "No Index Meta Tag", description)
}

pub fn programmable_redirection_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::ProgrammableRedirection;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Programmable Redirection", description)
}

pub fn redirected_to_another_url_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesCrawlerCodes::RedirectedToAnotherURL;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Redirected to Another URL", description)
}

/// Functions return formatted data as JSON containing HTTP status code, status message and description of various informational responses.
pub fn parsing_error_unfinished_header() -> serde_json::Value {
  let (code, name, desc) = parsing_error_unfinished_header_tuple();
  json!({ "status": code, "name": name, "description": desc })
}
pub fn parsing_error_header() -> serde_json::Value {
  let (code, name, desc) = parsing_error_header_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn parsing_error_missing_http_code() -> serde_json::Value {
  let (code, name, desc) = parsing_error_missing_http_code_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn parsing_error_body() -> serde_json::Value {
  let (code, name, desc) = parsing_error_body_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn excluded_by_robots_txt_file() -> serde_json::Value {
  let (code, name, desc) = excluded_by_robots_txt_file_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn robots_temporarily_unavailable() -> serde_json::Value {
  let (code, name, desc) = robots_temporarily_unavailable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn excluded_by_definition_of_exploration_space() -> serde_json::Value {
  let (code, name, desc) = excluded_by_definition_of_exploration_space_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn not_allowed_by_local_exploration_space() -> serde_json::Value {
  let (code, name, desc) = not_allowed_by_local_exploration_space_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn incorrect_protocol_or_non_standard_system_port() -> serde_json::Value {
  let (code, name, desc) = incorrect_protocol_or_non_standard_system_port_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn excluded_by_file_type_exclusions() -> serde_json::Value {
  let (code, name, desc) = excluded_by_file_type_exclusions_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_card() -> serde_json::Value {
  let (code, name, desc) = invalid_card_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn cannot_disable_physical_card() -> serde_json::Value {
  let (code, name, desc) = cannot_disable_physical_card_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_url() -> serde_json::Value {
  let (code, name, desc) = invalid_url_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn no_index_meta_tag() -> serde_json::Value {
  let (code, name, desc) = no_index_meta_tag_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn programmable_redirection() -> serde_json::Value {
  let (code, name, desc) = programmable_redirection_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn redirected_to_another_url() -> serde_json::Value {
  let (code, name, desc) = redirected_to_another_url_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

// Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_parsing_error_unfinished_header() {
    let response = ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader;
    let (code, description) = response.into();
    assert_eq!(code, 700);
    assert_eq!(description, "Parsing Error: Unfinished Header");
  }

  #[test]
  fn test_to_u16_parsing_error_header() {
    let response = ResponsesCrawlerCodes::ParsingErrorHeader;
    let code = response.to_u16();
    assert_eq!(code, 710);
  }

  #[test]
  fn test_parsing_error_missing_http_code() {
    assert_eq!(
      parsing_error_missing_http_code_tuple(),
      (720, "Parsing Error: Missing HTTP Code", "The HTTP code is missing and cannot be parsed")
    );
  }

  #[test]
  fn test_from_u16_invalid_url() {
    let response = ResponsesCrawlerCodes::from_u16(786);
    assert_eq!(response, Some(ResponsesCrawlerCodes::InvalidURL));
  }

  #[test]
  fn test_programmable_redirection() {
    assert_eq!(
      programmable_redirection_tuple(),
      (3020, "Programmable Redirection", "The URL is programmably redirected")
    );
  }

  #[test]
  fn test_no_index_meta_tag() {
    let (code, name, description) = no_index_meta_tag_tuple();
    assert_eq!(code, 2004);
    assert_eq!(name, "No Index Meta Tag");
    assert_eq!(description, "No index meta tag");
  }
}
