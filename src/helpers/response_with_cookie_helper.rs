use serde_json::json;

/// This function is used to return a JSON response with a status of 200 and a description of "OK" along with a cookie.
pub fn ok_with_cookie(cookie: (&str, &str)) -> String {
  let (key, value) = cookie;
  json!({
      "status": "OK",
      "code": 200,
      "description": "Request processed successfully",
      "cookie": {
          "key": key,
          "value": value
      }
  })
  .to_string()
}

pub fn bad_request_with_cookie(cookie: (&str, &str)) -> String {
  let (key, value) = cookie;
  json!({
      "status": "Bad Request",
      "code": 400,
      "description": "The server cannot process the request due to malformed syntax",
      "cookie": {
          "key": key,
          "value": value
      }
  })
  .to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ok_with_cookie() {
    let response = ok_with_cookie(("session_id", "abc123"));
    assert!(response.contains("\"status\":\"OK\""));
    assert!(response.contains("\"code\":200"));
    assert!(response.contains("\"cookie\":{\"key\":\"session_id\",\"value\":\"abc123\"}"));
  }

  #[test]
  fn test_bad_request_with_cookie() {
    let response = bad_request_with_cookie(("error_id", "xyz789"));
    assert!(response.contains("\"status\":\"Bad Request\""));
    assert!(response.contains("\"code\":400"));
    assert!(response.contains("\"cookie\":{\"key\":\"error_id\",\"value\":\"xyz789\"}"));
  }
}
