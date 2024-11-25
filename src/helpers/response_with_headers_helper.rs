use serde_json::json;
use std::collections::HashMap;

/// This helper function is used to return a JSON response with a status of 200 and a description of "OK" along with the data and headers provided.
pub fn ok_with_headers(headers: HashMap<&str, &str>) -> String {
  json!({
      "status": "OK",
      "code": 200,
      "description": "Request processed successfully",
      "headers": headers
  })
  .to_string()
}

pub fn bad_request_with_headers(headers: HashMap<&str, &str>) -> String {
  json!({
      "status": "Bad Request",
      "code": 400,
      "description": "The server cannot process the request due to malformed syntax",
      "headers": headers
  })
  .to_string()
}
