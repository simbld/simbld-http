use crate::helpers::to_u16_helper::ToU16;
use crate::responses::ResponsesTypes;
use strum::EnumProperty;

/// Fetches the response description for a given `ResponsesTypes`.
pub fn get_response_description(response: ResponsesTypes) -> (u16, &'static str) {
  let code = response.to_u16();
  let description = response.description();
  (code, description)
}

/// Matches an HTTP code and returns the corresponding `ResponsesTypes` enum.
pub fn get_response_by_code(code: u16) -> Option<ResponsesTypes> {
  ResponsesTypes::from_u16(code)
}

/// Fetches the description for a specific HTTP code.
pub fn get_description_by_code(code: u16) -> Option<&'static str> {
  get_response_by_code(code).map(|r| r.description())
}
