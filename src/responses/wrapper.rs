/// This module provides functions to generate HTTP response functions based on enum variants.
///
/// # Functions
///
/// - `generate_responses`: Generates functions that return a tuple containing the HTTP status code and a static string representation of the variant.
/// - `generate_responses_with_metadata`: Generates functions that return a string response enriched with metadata.
///
/// # Example
///
/// ```rustpeut on mo
///
/// let responses = generate_responses();
/// println!("{}", responses);
///
/// let responses_with_metadata = generate_responses_with_metadata();
/// println!("{}", responses_with_metadata);
/// ```
///
/// # Details
///
/// Both functions iterate over all variants of an enum `T` and generate two functions for each variant:
/// one with a snake_case name and one with a CamelCase name. The `generate_responses` function creates
/// functions that return a tuple `(u16, &'static str)`, while the `generate_responses_with_metadata` function
/// creates functions that return a string enriched with metadata.
use inflector::Inflector;
use strum::IntoEnumIterator;

pub struct ResponseWrapper<T>(pub T);

impl<T> ResponseWrapper<T>
where
  T: IntoEnumIterator + std::fmt::Debug + Copy + ToString + Into<u16>,
{
  pub fn generate_responses() -> String {
    let mut output = String::new();
    let variants = T::iter().collect::<Vec<_>>(); // Collect all variations
    for variant in variants {
      let function_name_snake = variant.to_string().to_snake_case();
      let function_name_camel = variant.to_string().to_camel_case();
      let code: u16 = variant.into();

      output.push_str(&format!(
        "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}\n",
        function_name_snake, code, variant
      ));
      output.push_str(&format!(
        "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}\n",
        function_name_camel, code, variant
      ));
    }
    output
  }

  pub fn generate_responses_with_metadata() -> String {
    let mut output = String::new();
    let variants = T::iter().collect::<Vec<_>>(); // Collect all variations
    for variant in variants {
      let function_name_snake = variant.to_string().to_snake_case();
      let function_name_camel = variant.to_string().to_camel_case();
      let code: u16 = variant.into();

      output.push_str(&format!(
                "fn {}() -> String {{ response_helpers::get_enriched_response_with_metadata({}, None, std::time::Duration::from_millis(100)) }}\n",
                function_name_snake, code
            ));
      output.push_str(&format!(
                "fn {}() -> String {{ response_helpers::get_enriched_response_with_metadata({}, None, std::time::Duration::from_millis(100)) }}\n",
                function_name_camel, code
            ));
    }
    output
  }
}

#[cfg(test)]
mod tests {
  use crate::mocks::mock_responses::MockResponses;
  use crate::responses::wrapper::ResponseWrapper;

  #[test]
  fn test_generate_responses() {
    let output = ResponseWrapper::<MockResponses>::generate_responses();

    assert!(output.contains("fn ok() -> (u16, &'static str) { (200, Ok) }"));
    assert!(output.contains("fn bad_request() -> (u16, &'static str) { (400, BadRequest) }"));
    assert!(output.contains("fn unauthorized() -> (u16, &'static str) { (401, Unauthorized) }"));
    assert!(output.contains("fn not_found() -> (u16, &'static str) { (404, NotFound) }"));
    assert!(output.contains(
      "fn internal_server_error() -> (u16, &'static str) { (500, InternalServerError) }"
    ));
  }

  #[test]
  fn test_generate_responses_with_metadata() {
    let output = ResponseWrapper::<MockResponses>::generate_responses_with_metadata();

    assert!(output.contains("fn ok() -> String { response_helpers::get_enriched_response_with_metadata(200, None, std::time::Duration::from_millis(100)) }"));
    assert!(output.contains("fn bad_request() -> String { response_helpers::get_enriched_response_with_metadata(400, None, std::time::Duration::from_millis(100)) }"));
    assert!(output.contains("fn unauthorized() -> String { response_helpers::get_enriched_response_with_metadata(401, None, std::time::Duration::from_millis(100)) }"));
    assert!(output.contains("fn not_found() -> String { response_helpers::get_enriched_response_with_metadata(404, None, std::time::Duration::from_millis(100)) }"));
    assert!(output.contains("fn internal_server_error() -> String { response_helpers::get_enriched_response_with_metadata(500, None, std::time::Duration::from_millis(100)) }"));
  }
}
