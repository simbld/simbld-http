/// The code provides functions for handling HTTP response codes, including retrieving descriptions, converting to JSON/XML, filtering by range, and adding metadata.
///
/// Arguments:
///
/// * `response`: The `response` parameter in the provided code snippets refers to an enumeration type `ResponsesTypes` that represents various HTTP response code families and their descriptions. Snippet functions use this enumeration to extract information such as response codes, descriptions, and metadata related to HTTP responses.
// use crate::helpers::to_u16_helper::ToU16;
use crate::responses::ResponsesTypes;
use crate::responses::*;
use crate::responses::{
  ResponsesClientCodes, ResponsesCrawlerCodes, ResponsesInformationalCodes, ResponsesLocalApiCodes,
  ResponsesRedirectionCodes, ResponsesServerCodes, ResponsesServiceCodes, ResponsesSuccessCodes,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;
use strum::EnumProperty;
use strum::IntoEnumIterator;

impl ResponsesTypes {
  pub fn get_str(&self, key: &str) -> Option<&'static str> {
    match self {
      ResponsesTypes::Client(code) => code.as_tuple().get_str(key),
      ResponsesTypes::Crawler(code) => code.as_tuple().get_str(key),
      ResponsesTypes::Informational(code) => code.as_tuple().get_str(key),
      ResponsesTypes::Local(code) => code.as_tuple().get_str(key),
      ResponsesTypes::Redirection(code) => code.as_tuple().get_str(key),
      ResponsesTypes::Server(code) => code.as_tuple().get_str(key),
      ResponsesTypes::Service(code) => code.as_tuple().get_str(key),
      ResponsesTypes::Success(code) => code.as_tuple().get_str(key),
    }
  }

  pub fn description(&self) -> &'static str {
    match self {
      ResponsesTypes::Client(code) => code.as_tuple().description(),
      ResponsesTypes::Crawler(code) => code.as_tuple().description(),
      ResponsesTypes::Informational(code) => code.as_tuple().description(),
      ResponsesTypes::Local(code) => code.as_tuple().description(),
      ResponsesTypes::Redirection(code) => code.as_tuple().description(),
      ResponsesTypes::Server(code) => code.as_tuple().description(),
      ResponsesTypes::Service(code) => code.as_tuple().description(),
      ResponsesTypes::Success(code) => code.as_tuple().description(),
    }
  }

  pub fn to_u16(&self) -> u16 {
    match self {
      ResponsesTypes::Client(code) => code.to_u16(),
      ResponsesTypes::Crawler(code) => code.to_u16(),
      ResponsesTypes::Informational(code) => code.to_u16(),
      ResponsesTypes::Local(code) => code.to_u16(),
      ResponsesTypes::Redirection(code) => code.to_u16(),
      ResponsesTypes::Server(code) => code.to_u16(),
      ResponsesTypes::Service(code) => code.to_u16(),
      ResponsesTypes::Success(code) => code.to_u16(),
    }
  }

  pub fn as_json(&self) -> Value {
    match self {
      ResponsesTypes::Client(code) => code.as_json(),
      ResponsesTypes::Crawler(code) => code.as_json(),
      ResponsesTypes::Informational(code) => code.as_json(),
      ResponsesTypes::Local(code) => code.as_json(),
      ResponsesTypes::Redirection(code) => code.as_json(),
      ResponsesTypes::Server(code) => code.as_json(),
      ResponsesTypes::Service(code) => code.as_json(),
      ResponsesTypes::Success(code) => code.as_json(),
    }
  }
}

// Implémentations pour les itérations
#[derive(EnumIter)]
pub enum ResponsesIter {
  Client(ResponsesClientCodes),
  Crawler(ResponsesCrawlerCodes),
  Informational(ResponsesInformationalCodes),
  Local(ResponsesLocalApiCodes),
  Redirection(ResponsesRedirectionCodes),
  Server(ResponsesServerCodes),
  Service(ResponsesServiceCodes),
  Success(ResponsesSuccessCodes),
}

// Fonctions helper pour l'itération
pub fn iter_responses() -> impl Iterator<Item = ResponsesTypes> {
  ResponsesIter::iter().map(|code| code.into())
}

/// Takes an input of type `ResponsesTypes`, extracts the associated response code and description,
/// and returns a tuple containing the code as a `u16` and the description as a static string reference (`&'static str`).
pub fn get_response_description(response: ResponsesTypes) -> (u16, &'static str) {
  let code = response.to_u16();
  let description = response.get_str("Description").unwrap_or("No description");
  (code, description)
}

/// This advanced implementation includes additional features:
/// - Logs the response code and the timestamp for debugging and tracking purposes.
/// - Simulates a CORS validation by checking the `ALLOWED_ORIGIN` environment variable.
/// - Logs warnings or debug information based on the state of the environment variable.
pub fn get_advance_response_description(response: ResponsesTypes) -> (u16, &'static str) {
  let timestamp = SystemTime::now();

  // Simulate a middleware call here to record logs
  log::info!("Fetching description for code: {}, timestamp: {:?}", response.to_u16(), timestamp);

  // Simulate a CORS check or any other advanced security logic
  if let Some(origin) = std::env::var("ALLOWED_ORIGIN").ok() {
    log::debug!("Applying CORS check for origin: {}", origin);
  } else {
    log::warn!("No ALLOWED_ORIGIN set; defaulting to open.");
  }

  // Provide a fallback description if not present
  let code = response.to_u16();
  let description = response.get_str("Description").unwrap_or("No description");

  (code, description)
}

/// Matches an HTTP code and returns the corresponding `ResponsesTypes` enum.
pub fn get_response_by_code(code: u16) -> Option<ResponsesTypes> {
  ResponsesTypes::from_u16(code)
}

/// Fetches the description for a specific HTTP code.
pub fn get_description_by_code(code: u16) -> Option<&'static str> {
  get_response_by_code(code).and_then(|r| r.description().into())
}

/// Matches the input code with predefined HTTP response codes and returns the corresponding description as a static string if a match is found.
pub fn get_advance_description_by_code(code: u16) -> Option<&'static str> {
  log::info!("Fetching description for code: {}", code);

  let fetched_description = get_response_by_code(code).map(|response_type| {
    let description = response_type.get_str("Description").unwrap_or("No description");
    log::debug!("Code {} corresponds to description: {}", code, description);
    description
  });

  if fetched_description.is_none() {
    log::warn!("No response type found for code: {}", code);
  }

  fetched_description
}

/// Converts a `ResponsesTypes` into a short JSON string. Includes a fallback if the response is not found.
pub fn transform_to_json_short(response: ResponsesTypes) -> String {
  let (code, description) = get_response_description(response);
  json!({
      "code": code,
      "description": description
  })
  .to_string()
}

/// Converts a `ResponsesTypes` into a detailed JSON string with fallback for missing descriptions.
pub fn transform_to_json(response: ResponsesTypes) -> String {
  let code = response.to_u16();
  let description = response.description();
  serde_json::json!({
      "code": code,
      "description": description
  })
  .to_string()
}

/// Converts a `ResponsesTypes` into JSON only for standard HTTP codes (100–599). Returns `None` for invalid codes.
pub fn transform_to_json_filtered(response: ResponsesTypes) -> Option<String> {
  let (code, description) = get_response_description(response);
  if code < 100 || code > 599 {
    None
  } else {
    Some(
      json!({
          "code": code,
          "description": description,
          "is_standard_code": true
      })
      .to_string(),
    )
  }
}

/// Converts a `ResponsesTypes` into JSON, enriched with metadata like timestamps and status families.
pub fn transform_to_json_with_metadata(response: ResponsesTypes) -> String {
  let (code, description) = get_response_description(response);
  let timestamp = chrono::Utc::now();
  let metadata = json!({
      "requested_at": timestamp.to_rfc3339(),
      "status_family": match code {
          100..=199 => "Informational",
          200..=299 => "Success",
          300..=399 => "Redirection",
          400..=499 => "Client Error",
          500..=599 => "Server Error",
          600..=699 => "Service Error",
          700..=799 => "Crawler Error",
          900..=999 => "Local API Error",
          _ => "Unknown",
      },
      "is_error": code >= 400,
  });

  json!({
      "code": code,
      "description": description,
      "metadata": metadata
  })
  .to_string()
}

/// Converts a `ResponsesTypes` into a simple XML string. Includes a fallback for missing responses.
pub fn transform_to_xml_short(response: ResponsesTypes) -> String {
  let (code, description) = get_response_description(response);
  format!(r#"<response><code>{}</code><description>{}</description></response>"#, code, description)
}

/// Converts a `ResponsesTypes` into a detailed XML string. Handles missing descriptions gracefully.
pub fn transform_to_xml(response: ResponsesTypes) -> String {
  let code = response.to_u16();
  let description = response.description();
  format!("<response><code>{}</code><description>{}</description></response>", code, description)
}

/// Converts a `ResponsesTypes` into an XML string for valid HTTP codes (100–599). Returns `None` for invalid codes.
pub fn transform_to_xml_filtered(response: ResponsesTypes) -> Option<String> {
  let (code, description) = get_response_description(response);
  if code < 100 || code > 599 {
    None
  } else {
    Some(format!(
      r#"<response><code>{}</code><description>{}</description><is_standard_code>true</is_standard_code></response>"#,
      code, description
    ))
  }
}

/// Converts a `ResponsesTypes` into an XML string enriched with metadata such as timestamps and status families.
pub fn transform_to_xml_with_metadata(response: ResponsesTypes) -> String {
  let (code, description) = get_response_description(response);
  let timestamp = chrono::Utc::now();
  let status_family = match code {
    100..=199 => "Informational",
    200..=299 => "Success",
    300..=399 => "Redirection",
    400..=499 => "Client Error",
    500..=599 => "Server Error",
    600..=699 => "Service Error",
    700..=799 => "Crawler Error",
    900..=999 => "Local API Error",
    _ => "Unknown",
  };
  let is_error = code >= 400;

  format!(
    r#"<response>
  <code>{}</code>
  <description>{}</description>
  <metadata>
    <requested_at>{}</requested_at>
    <status_family>{}</status_family>
    <is_error>{}</is_error>
  </metadata>
</response>"#,
    code,
    description,
    timestamp.to_rfc3339(),
    status_family,
    is_error
  )
}

/// Filters predefined response codes based on a specified range. The function takes two input parameters, `start` and `end`, representing the lower and upper bounds of the range, respectively. It returns a vector containing tuples of response codes and descriptions that fall within the specified range.
pub fn filter_codes_by_range(start: u16, end: u16) -> Vec<(u16, &'static str)> {
  let mut filtered_codes = Vec::new();

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
      let description = code_enum.get_str("Description").unwrap_or("No description");
      filtered_codes.push((code, description));
    }
  }
}

/// Filters predefined response codes based on a specified range and includes additional metadata. The function takes three input parameters: `start`, `end`, and `request_metadata`. The `start` and `end` parameters represent the lower and upper bounds of the range, respectively. The `request_metadata` parameter is an optional `HashMap` containing additional metadata to include in the response. The function returns a vector containing tuples of response codes, descriptions, and metadata that fall within the specified range.
pub fn filter_codes_by_range_with_metadata(
  start: u16,
  end: u16,
  request_metadata: Option<HashMap<&str, &str>>,
) -> Vec<(u16, &'static str, HashMap<String, String>)> {
  let iterators: Vec<Box<dyn Iterator<Item = (u16, &'static str)>>> = vec![
    Box::new(
      ResponsesInformationalCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
    Box::new(
      ResponsesSuccessCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
    Box::new(
      ResponsesRedirectionCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
    Box::new(
      ResponsesClientCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
    Box::new(
      ResponsesServerCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
    Box::new(
      ResponsesServiceCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
    Box::new(
      ResponsesCrawlerCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
    Box::new(
      ResponsesLocalApiCodes::iter()
        .map(|code| (code.into(), code.get_str("Description").unwrap_or("No description"))),
    ),
  ];

  let mut results = vec![];

  for codes in iterators {
    for code_enum in codes {
      let code: u16 = code_enum.0;
      if (start..=end).contains(&code) {
        let description = code_enum.1;
        let mut metadata = HashMap::new();
        metadata.insert("description".to_string(), description.to_string());
        metadata.insert("is_error".to_string(), (code >= 400).to_string());
        metadata.insert(
          "status_family".to_string(),
          match code {
            100..=199 => "Informational",
            200..=299 => "Success",
            300..=399 => "Redirection",
            400..=499 => "Client Error",
            500..=599 => "Server Error",
            600..=699 => "Service Error",
            700..=799 => "Crawler Error",
            900..=999 => "Local API Error",
            _ => "Unknown",
          }
          .to_string(),
        );

        if let Some(req_meta) = &request_metadata {
          for (key, value) in req_meta {
            metadata.insert((*key).to_string(), (*value).to_string());
          }
        }

        results.push((code, description, metadata));
      }
    }
  }

  results
}

/// Lists response codes and descriptions for a given family in a concise way.
pub fn list_codes_and_descriptions_short(family: &str) -> Vec<(u16, &'static str)> {
  let iterator: Box<dyn Iterator<Item = (u16, &'static str)>> = match family {
    "Informational" => Box::new(
      ResponsesInformationalCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    "Success" => Box::new(
      ResponsesSuccessCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    "Redirection" => Box::new(
      ResponsesRedirectionCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    "ClientError" => Box::new(
      ResponsesClientCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    "ServerError" => Box::new(
      ResponsesServerCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    "Service" => Box::new(
      ResponsesServiceCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    "Crawler" => Box::new(
      ResponsesCrawlerCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    "LocalApi" => Box::new(
      ResponsesLocalApiCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("Unknown description"))),
    ),
    _ => return vec![],
  };

  iterator.collect()
}

/// Lists response codes and descriptions for a given family with extended metadata.
pub fn list_codes_and_descriptions_with_metadata(
  family: &str,
  request_metadata: Option<HashMap<&str, &str>>,
) -> Vec<(u16, &'static str, HashMap<String, String>)> {
  let iterator: Box<dyn Iterator<Item = (u16, &'static str)>> = match family {
    "Informational" => Box::new(
      ResponsesInformationalCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    "Success" => Box::new(
      ResponsesSuccessCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    "Redirection" => Box::new(
      ResponsesRedirectionCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    "ClientError" => Box::new(
      ResponsesClientCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    "ServerError" => Box::new(
      ResponsesServerCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    "ServiceError" => Box::new(
      ResponsesServiceCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    "CrawlerError" => Box::new(
      ResponsesCrawlerCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    "LocalApiError" => Box::new(
      ResponsesLocalApiCodes::iter()
        .map(|c| (c.to_u16(), c.get_str("Description").unwrap_or("No description"))),
    ),
    _ => return vec![],
  };

  iterator
    .map(|(code, description)| {
      let mut metadata = HashMap::new();
      metadata.insert("description".to_string(), description.to_string());
      metadata.insert("is_error".to_string(), (code >= 400).to_string());
      metadata.insert(
        "status_family".to_string(),
        match code {
          100..=199 => "Informational",
          200..=299 => "Success",
          300..=399 => "Redirection",
          400..=499 => "Client Error",
          500..=599 => "Server Error",
          600..=699 => "Service Error",
          700..=799 => "Crawler Error",
          900..=999 => "Local API Error",
          _ => "Unknown",
        }
        .to_string(),
      );

      if let Some(req_meta) = &request_metadata {
        for (key, value) in req_meta {
          metadata.insert((*key).to_string(), (*value).to_string());
        }
      }

      (code, description, metadata)
    })
    .collect()
}

/// Generates a complete response string in JSON format based on input parameters. The function takes three input parameters: `code`, `description`, and `data`.
pub fn create_response(code: u16, description: &str, data: &str) -> String {
  json!({
      "code": code,
      "description": description,
      "data": serde_json::from_str::<Value>(data).unwrap_or(Value::Null)
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

/// Generates a complete response string in JSON format based on input parameters. The function takes two input parameters: `response_type` and `data`.
pub fn create_response_with_types(
  response_type: Option<ResponsesTypes>,
  data: Option<&str>,
) -> String {
  let mut map = serde_json::Map::new();
  if let Some(rt) = response_type {
    map.insert("code".to_string(), json!(rt.to_u16()));
    let description = rt.description();
    map.insert("description".to_string(), json!(description));
  }
  if let Some(d) = data {
    let data_value: serde_json::Value = serde_json::from_str(d).unwrap();
    map.extend(data_value.as_object().unwrap().clone());
  }
  serde_json::Value::Object(map).to_string()
}

/// Fetches a full response with CORS headers and metadata.
pub fn get_enriched_response_with_metadata(
  response: ResponsesTypes,
  cors_origin: Option<&str>,
  duration: Duration,
) -> String {
  let (code, description) = get_response_description(response);
  let timestamp = chrono::Utc::now();

  // Add CORS headers
  let cors_headers = match cors_origin {
    Some(origin) => json!({ "Access-Control-Allow-Origin": origin }),
    None => json!({ "Access-Control-Allow-Origin": "*" }),
  };

  // Metadata
  let metadata = json!({
      "requested_at": timestamp.to_rfc3339(),
      "status_family": match code {
          100..=199 => "Informational",
          200..=299 => "Success",
          300..=399 => "Redirection",
          400..=499 => "Client Error",
          500..=599 => "Server Error",
          _ => "Unknown",
      },
      "is_error": code >= 400,
      "processing_time_ms": duration.as_millis()
  });

  json!({
      "code": code,
      "description": description,
      "headers": cors_headers,
      "metadata": metadata
  })
  .to_string()
}

/// Validates if the given origin is allowed based on a list of allowed origins.
pub fn is_origin_allowed(origin: &str, allowed_origins: &[&str]) -> bool {
  allowed_origins.contains(&origin)
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
  fn test_transform_to_json_with_metadata() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let json_str = transform_to_json_with_metadata(response);
    assert!(json_str.contains("\"requested_at\""));
    assert!(json_str.contains("\"status_family\":\"Success\""));
  }

  #[test]
  fn test_transform_to_json_filtered() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let json_str = transform_to_json_filtered(response).unwrap();
    assert!(json_str.contains("\"is_standard_code\":true"));
  }

  #[test]
  fn test_transform_to_xml() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let xml_str = transform_to_xml(response);
    let expected_xml = r#"<response><code>200</code><description>Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response</description></response>"#;
    assert_eq!(xml_str, expected_xml);
  }

  #[test]
  fn test_transform_to_xml_with_metadata() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let xml_str = transform_to_xml_with_metadata(response);
    assert!(xml_str.contains("<requested_at>"));
    assert!(xml_str.contains("<status_family>Success</status_family>"));
  }

  #[test]
  fn test_transform_to_xml_filtered() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let xml_str = transform_to_xml_filtered(response).unwrap();
    assert!(xml_str.contains("<is_standard_code>true</is_standard_code>"));
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
                "The server has received the initial part of the request, the headers, and asks the client to continue request, proceed to send the body of the request, a POST request",
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
  fn test_filter_codes_by_range_with_metadata() {
    let request_metadata = Some(std::collections::HashMap::from([("source", "test_case")]));
    let codes = filter_codes_by_range_with_metadata(200, 299, request_metadata);
    assert!(!codes.is_empty());
    assert!(codes[0].2.contains_key("source"));
  }

  #[test]
  fn test_filter_codes_by_range_by_length() {
    let codes = filter_codes_by_range(200, 201);
    assert_eq!(codes.len(), 2);
  }

  #[test]
  fn test_list_codes_and_descriptions_with_metadata() {
    let request_metadata = Some(std::collections::HashMap::from([("source", "unit_test")]));
    let descriptions = list_codes_and_descriptions_with_metadata("Success", request_metadata);
    assert!(!descriptions.is_empty());
    assert!(descriptions[0].2.contains_key("source"));
  }

  #[test]
  fn test_list_codes_and_descriptions_short() {
    let informational = list_codes_and_descriptions_short("Informational");
    assert!(!informational.is_empty());
    assert_eq!(informational[0].0, 100);

    let success = list_codes_and_descriptions_short("Success");
    assert!(!success.is_empty());
    assert_eq!(success[0].0, 200);

    let unknown = list_codes_and_descriptions_short("UnknownFamily");
    assert!(unknown.is_empty());
  }

  #[test]
  fn test_create_response_with_types() {
    let response = Some(ResponsesTypes::Success(ResponsesSuccessCodes::Ok));
    let json_response = create_response_with_types(response, Some(r#"{"key": "value"}"#));
    println!("JSON Response: {}", json_response);
    assert!(json_response.contains("\"description\":\"Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response\""));
    assert!(json_response.contains("\"key\":\"value\""));
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
  fn test_create_response_without_json() {
    let code = 200;
    let description = "OK";
    let data = r#"{"message": "Success"}"#;
    let response = create_response(code, description, data);
    assert!(response.contains("\"code\":200"));
    assert!(response.contains("\"description\":\"OK\""));
    assert!(response.contains("\"message\":\"Success\""));
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

  #[test]
  fn test_get_enriched_response_with_metadata() {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let enriched_response = get_enriched_response_with_metadata(
      response,
      Some("http://example.com"),
      Duration::from_millis(150),
    );
    assert!(enriched_response.contains("\"code\":200"));
    assert!(enriched_response.contains("\"Access-Control-Allow-Origin\":\"http://example.com\""));
  }

  #[test]
  fn test_is_origin_allowed() {
    let allowed_origins = vec!["http://example.com", "http://localhost"];
    assert!(is_origin_allowed("http://example.com", &allowed_origins));
    assert!(!is_origin_allowed("http://unauthorized.com", &allowed_origins));
  }
}
