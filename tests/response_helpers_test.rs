use simbld_http::helpers::{response_with_cookie_helper::*, response_with_headers_helper::*};
use std::collections::HashMap;

#[test]
fn test_ok_with_cookie() {
  let cookie = ("session_id", "abc123");
  let response = ok_with_cookie(cookie);
  assert!(response.contains("\"status\":\"OK\""));
  assert!(response.contains("\"cookie\":{\"key\":\"session_id\",\"value\":\"abc123\"}"));
}

#[test]
fn test_bad_request_with_cookie() {
  let cookie = ("error_id", "xyz789");
  let response = bad_request_with_cookie(cookie);
  assert!(response.contains("\"status\":\"Bad Request\""));
  assert!(response.contains("\"cookie\":{\"key\":\"error_id\",\"value\":\"xyz789\"}"));
}

#[test]
fn test_ok_with_headers() {
  let mut headers = HashMap::new();
  headers.insert("x-trace-id", "123456");
  headers.insert("x-correlation-id", "abc-def");

  let response = ok_with_headers(headers.clone());
  let parsed: serde_json::Value = serde_json::from_str(&response).unwrap();

  assert_eq!(parsed["status"], "OK");
  assert_eq!(parsed["headers"]["x-trace-id"], "123456");
  assert_eq!(parsed["headers"]["x-correlation-id"], "abc-def");
}

#[test]
fn test_bad_request_with_headers() {
  let mut headers = HashMap::new();
  headers.insert("x-trace-id", "654321");
  headers.insert("x-correlation-id", "fed-cba");

  let response = bad_request_with_headers(headers.clone());
  let parsed: serde_json::Value = serde_json::from_str(&response).unwrap();

  assert_eq!(parsed["status"], "Bad Request");
  assert_eq!(parsed["headers"]["x-trace-id"], "654321");
  assert_eq!(parsed["headers"]["x-correlation-id"], "fed-cba");
}

#[test]
fn test_helper_headers_output() {
  let mut headers = HashMap::new();
  headers.insert("x-trace-id", "123456");
  headers.insert("x-correlation-id", "abc-def");

  let response = ok_with_headers(headers.clone());
  println!("OK with Headers: {}", response);

  let response = bad_request_with_headers(headers.clone());
  println!("Bad Request with Headers: {}", response);
}
