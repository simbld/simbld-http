use crate::helpers::to_u16_helper::ToU16;
use crate::responses::ResponsesTypes;
use crate::responses::*;
use strum::EnumProperty;
use strum::IntoEnumIterator;

/// Takes an input of type `ResponsesTypes`, extracts the associated response code and description, and returns a tuple containing the code as a `u16` and the description as a static string reference (`&'static str`).
pub fn get_response_description(response: ResponsesTypes) -> (u16, &'static str) {
  let code = response.to_u16();
  let description = response.description();
  (code, description)
}

/// Corresponds to predefined HTTP response codes. If a match is found, it returns an `Option` containing the corresponding `ResponsesTypes` enumeration variant.
pub fn get_response_by_code(code: u16) -> Option<ResponsesTypes> {
  // Informational Codes
  if let Ok(response) = ResponsesInformationalCodes::try_from(code) {
    return Some(ResponsesTypes::Informational(response));
  }
  // Success Codes
  if let Ok(response) = ResponsesSuccessCodes::try_from(code) {
    return Some(ResponsesTypes::Success(response));
  }
  // Redirection Codes
  if let Ok(response) = ResponsesRedirectionCodes::try_from(code) {
    return Some(ResponsesTypes::Redirection(response));
  }
  // Client Error Codes
  if let Ok(response) = ResponsesClientCodes::try_from(code) {
    return Some(ResponsesTypes::ClientError(response));
  }
  // Server Error Codes
  if let Ok(response) = ResponsesServerCodes::try_from(code) {
    return Some(ResponsesTypes::ServerError(response));
  }
  // Service Error Codes
  if let Ok(response) = ResponsesServiceCodes::try_from(code) {
    return Some(ResponsesTypes::ServiceError(response));
  }
  // Crawler Error Codes
  if let Ok(response) = ResponsesCrawlerCodes::try_from(code) {
    return Some(ResponsesTypes::CrawlerError(response));
  }
  // Local API Error Codes
  if let Ok(response) = ResponsesLocalApiCodes::try_from(code) {
    return Some(ResponsesTypes::LocalApiError(response));
  }
  None
}

/// Matches the input code with predefined HTTP response codes and returns the corresponding description as a static string if a match is found.
pub fn get_description_by_code(code: u16) -> Option<&'static str> {
  if let Some(response_type) = ResponsesTypes::from_u16(code) {
    Some(response_type.description())
  } else {
    None
  }
}

/// Takes an input of type `ResponsesTypes`, extracts the response code and description, and formats them into a JSON string.
pub fn transform_to_json(response: ResponsesTypes) -> String {
  let (code, description) = get_response_description(response);
  serde_json::json!({
      "code": code,
      "description": description
  })
  .to_string()
}

/// Takes an input of type `ResponsesTypes`, extracts the response code and description, and formats them into an XML string.
pub fn transform_to_xml(response: ResponsesTypes) -> String {
  let (code, description) = get_response_description(response);
  format!(r#"<response><code>{}</code><description>{}</description></response>"#, code, description)
}

/// Filters predefined response codes based on a specified range. The function takes two input parameters, `start` and `end`, representing the lower and upper bounds of the range, respectively. It returns a vector containing tuples of response codes and descriptions that fall within the specified range.
pub fn filter_codes_by_range(start: u16, end: u16) -> Vec<(u16, &'static str)> {
  let mut filtered_codes = Vec::new();

  // Function to add filtered codes
  fn add_filtered_codes<T, U>(
    start: u16,
    end: u16,
    iter: T,
    filtered_codes: &mut Vec<(u16, &'static str)>,
  ) where
    T: Iterator<Item = U>,
    U: Into<u16> + EnumProperty + Copy,
  {
    for code_enum in iter {
      let code: u16 = code_enum.into();
      if code >= start && code <= end {
        if let Some(description) = code_enum.get_str("Description") {
          filtered_codes.push((code, description));
        }
      }
    }
  }

  add_filtered_codes(start, end, ResponsesInformationalCodes::iter(), &mut filtered_codes);
  add_filtered_codes(start, end, ResponsesSuccessCodes::iter(), &mut filtered_codes);
  add_filtered_codes(start, end, ResponsesRedirectionCodes::iter(), &mut filtered_codes);
  add_filtered_codes(start, end, ResponsesClientCodes::iter(), &mut filtered_codes);
  add_filtered_codes(start, end, ResponsesServerCodes::iter(), &mut filtered_codes);
  add_filtered_codes(start, end, ResponsesServiceCodes::iter(), &mut filtered_codes);
  add_filtered_codes(start, end, ResponsesCrawlerCodes::iter(), &mut filtered_codes);
  add_filtered_codes(start, end, ResponsesLocalApiCodes::iter(), &mut filtered_codes);

  filtered_codes
}

/// Returns a vector of tuples containing response codes and descriptions based on the input family. The function takes a string parameter `family` representing the family of response codes to list.
pub fn list_codes_and_descriptions(family: &str) -> Vec<(u16, &'static str)> {
  match family {
    "Informational" => ResponsesInformationalCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    "Success" => ResponsesSuccessCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    "Redirection" => ResponsesRedirectionCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    "ClientError" => ResponsesClientCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    "ServerError" => ResponsesServerCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    "ServiceError" => ResponsesServiceCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    "CrawlerError" => ResponsesCrawlerCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    "LocalApiError" => ResponsesLocalApiCodes::iter()
      .map(|code_enum| {
        (code_enum as u16, code_enum.get_str("Description").unwrap_or("No description"))
      })
      .collect(),
    _ => vec![],
  }
}

/// Generates a complete response string in JSON format based on input parameters. The function takes three input parameters: `code`, `description`, and `data`.
pub fn create_response(code: u16, description: &str, data: &str) -> String {
  serde_json::json!({
      "code": code,
      "description": description,
      "data": serde_json::from_str::<serde_json::Value>(data).unwrap_or(serde_json::Value::Null)
  })
  .to_string()
}

/// Generates a complete response string in XML format based on input parameters. The function takes three input parameters: `code`, `description`, and `data`.
pub fn create_response_xml(code: u16, description: &str, data: &str) -> String {
  format!(
    r#"<response><code>{}</code><description>{}</description><data>{}</data></response>"#,
    code, description, data
  )
}

/// Converts a `u16` code to an `Option` containing the corresponding `ResponsesTypes` enumeration variant. The function reuses the existing `get_response_by_code` function to perform the conversion.
pub fn convert_to_enum(code: u16) -> Option<ResponsesTypes> {
  get_response_by_code(code)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::{ResponsesSuccessCodes, ResponsesTypes};
  use serde_json::json;

  #[test]
  fn test_get_response_by_code() {
    let response = get_response_by_code(200);
    assert!(response.is_some());
    assert_eq!(response.unwrap().to_u16(), 200);
  }

  #[test]
  fn test_get_description_by_code() {
    let description = get_description_by_code(200);
    assert!(description.is_some());
    assert_eq!(
            description.unwrap(),
            "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
        );
  }

  #[test]
  fn test_transform_to_json() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let json_str = transform_to_json(response);
    let expected_json = json!({
            "code": 200,
            "description": "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
        })
        .to_string();
    assert_eq!(json_str, expected_json);
  }

  #[test]
  fn test_transform_to_xml() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let xml_str = transform_to_xml(response);
    let expected_description = "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response";
    let expected_xml = format!(
      r#"<response><code>200</code><description>{}</description></response>"#,
      expected_description
    );
    assert_eq!(xml_str, expected_xml);
  }

  #[test]
  fn test_get_response_by_code_unknown() {
    let response = get_response_by_code(9999);
    assert!(response.is_none());
  }

  #[test]
  fn test_filter_codes_by_range() {
    let codes = filter_codes_by_range(100, 103);
    let expected_codes = vec![
            (
                100,
                "The server has received the initial part of the request, the headers, and asks the client to continue, proceed to send the body of the request, a POST request",
            ),
            (
                101,
                "The server is complying with a request to switch protocols, used in WebSocket connections",
            ),
            (
                102,
                "Indicates the server is processing the request but has not yet finished, used to prevent timeout errors in asynchronous operations, webdav RFC 2518",
            ),
            (
                103,
                "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared",
            ),
        ];
    assert_eq!(codes, expected_codes);
  }

  #[test]
  fn test_create_response() {
    let code = 200;
    let description = "OK";
    let data = r#"{"message": "Success"}"#;
    let response = create_response(code, description, data);
    let expected_response = json!({
        "code": 200,
        "description": "OK",
        "data": {
            "message": "Success"
        }
    })
    .to_string();
    assert_eq!(response, expected_response);
  }

  #[test]
  fn test_create_response_xml() {
    let code = 200;
    let description = "OK";
    let data = "<message>Success</message>";
    let response = create_response_xml(code, description, data);
    let expected_response = format!(
      r#"<response><code>{}</code><description>{}</description><data>{}</data></response>"#,
      code, description, data
    );
    assert_eq!(response, expected_response);
  }

  #[test]
  fn test_convert_to_enum() {
    let code = 200;
    let response_type = convert_to_enum(code);
    assert!(response_type.is_some());
    assert_eq!(response_type.unwrap().to_u16(), 200);
  }
}
