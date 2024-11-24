/// This function is used to return a JSON response with a status of 200 and a description of "OK" along with a cookie.
pub fn ok_with_cookie<T>(data: T, cookie_name: &str, cookie_value: &str) -> serde_json::Value
where
  T: serde::Serialize,
{
  serde_json::json!({
      "status": 200,
      "description": "OK",
      "data": data,
      "cookie": {
          "name": cookie_name,
          "value": cookie_value
      }
  })
}
