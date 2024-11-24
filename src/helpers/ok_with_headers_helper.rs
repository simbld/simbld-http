/// This helper function is used to return a JSON response with a status of 200 and a description of "OK" along with the data and headers provided.
pub fn ok_with_headers<T>(data: T, headers: Vec<(&str, &str)>) -> serde_json::Value
where
  T: serde::Serialize,
{
  let mut response = serde_json::json!({
      "status": 200,
      "description": "OK",
      "data": data,
  });

  for (key, value) in headers {
    response[key] = serde_json::json!(value);
  }

  response
}
