use crate::helpers::get_str_helper::GetDescription;
use crate::responses::ResponsesTypes;
use chrono::Utc;
use std::collections::HashMap;
use std::time::Duration;

/// Fetches a full response with CORS headers and metadata.
pub fn get_enriched_response_with_metadata(
  response: ResponsesTypes,
  cors_origin: Option<&str>,
  duration: Duration,
) -> String {
  let (code, description) = get_response_description(response);
  let timestamp = Utc::now();

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

/// Fetches a response description based on the provided `ResponsesTypes` enum.
pub fn get_response_description(response: ResponsesTypes) -> (u16, &'static str) {
  let code = response.to_u16();
  let description = response.get_str("Description").unwrap_or("No description available.");
  (code, description)
}

/// Recovers the description from the tuple
pub fn get_description_by_code(status_code: u16) -> &'static str {
  match ResponsesTypes::from_u16(status_code) {
    Some(response) => response.as_tuple().get_description(),
    None => "Description not available",
  }
}

/// Converts a `ResponsesTypes` into a JSON string with metadata.
pub fn transform_to_json_with_metadata(response: ResponsesTypes) -> String {
  let (code, description) = get_response_description(response);
  let timestamp = Utc::now();

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
  });

  json!({
      "code": code,
      "description": description,
      "metadata": metadata
  })
  .to_string()
}

/// Filters predefined response codes based on a specified range.
pub fn filter_codes_by_range(start: u16, end: u16) -> Vec<(u16, &'static str)> {
  let all_codes = ResponsesTypes::iter();
  all_codes
    .filter(|code| {
      let u16_code = code.to_u16();
      u16_code >= start && u16_code <= end
    })
    .map(|code| (code.to_u16(), code.get_str("Description").unwrap_or("No description")))
    .collect()
}

/// Filters response codes by range with metadata.
pub fn filter_codes_by_range_with_metadata(
  start: u16,
  end: u16,
  request_metadata: Option<HashMap<&str, &str>>,
) -> Vec<(u16, &'static str, HashMap<String, String>)> {
  let all_codes = ResponsesTypes::iter();
  let mut results = vec![];

  for code in all_codes {
    let u16_code = code.to_u16();
    if (start..=end).contains(&u16_code) {
      let description = code.get_str("Description").unwrap_or("No description");
      let mut metadata = HashMap::new();
      metadata.insert("description".to_string(), description.to_string());
      metadata.insert("is_error".to_string(), (u16_code >= 400).to_string());
      metadata.insert(
        "status_family".to_string(),
        match u16_code {
          100..=199 => "Informational",
          200..=299 => "Success",
          300..=399 => "Redirection",
          400..=499 => "Client Error",
          500..=599 => "Server Error",
          _ => "Unknown",
        }
        .to_string(),
      );

      if let Some(req_meta) = &request_metadata {
        for (key, value) in req_meta {
          metadata.insert((*key).to_string(), (*value).to_string());
        }
      }

      results.push((u16_code, description, metadata));
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ResponsesSuccessCodes::Ok;

  use std::collections::HashMap;
  use std::time::Duration;

  #[test]
  fn test_get_response_description() {
    let response = Ok;
    let (code, description) = get_response_description(response);
    assert_eq!(code, 200);
    assert_eq!(description, "Request processed successfully.");
  }

  #[test]
  fn test_get_enriched_response_with_metadata() {
    let response = Ok;
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

  #[test]
  fn test_filter_codes_by_range() {
    let codes = filter_codes_by_range(200, 299);
    assert!(!codes.is_empty());
    assert_eq!(codes[0].0, 200);
  }

  #[test]
  fn test_filter_codes_by_range_with_metadata() {
    let metadata = Some(HashMap::from([("source", "test")]));
    let codes = filter_codes_by_range_with_metadata(200, 299, metadata);
    assert!(!codes.is_empty());
    assert!(codes[0].2.contains_key("source"));
  }
}
