
use std::collections::HashMap;

/// This helper function is used to return a JSON response with a status of 200 and a description of "OK" along with the data and headers provided.
///
/// # Arguments
///
/// * `headers` - A `HashMap` containing the headers to be included in the response.
///
/// # Returns
///
/// A `String` containing the JSON response.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use simbld_http::helpers::response_with_headers_helper::ok_with_headers;
///
/// let mut headers = HashMap::new();
/// headers.insert("Content-Type", "application/json");
/// let response = ok_with_headers(headers);
/// println!("{}", response);
/// pub fn ok_with_headers(headers: HashMap<&str, &str>) -> String {
///  json!({
///      "status": "OK",
///      "code": 200,
///      "description": "Request processed successfully",
///      "headers": headers
///  })
///  .to_string()
/// }
/// ```

/// This helper function is used to return a JSON response with a status of 400 and a description of "Bad Request" along with the data and headers provided.
///
/// # Arguments
///
/// * `headers` - A `HashMap` containing the headers to be included in the response.
///
/// # Returns
///
/// A `String` containing the JSON response.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use simbld_http::helpers::response_with_headers_helper::bad_request_with_headers;
///
/// let mut headers = HashMap::new();
/// headers.insert("Content-Type", "application/json");
/// let response = bad_request_with_headers(headers);
/// println!("{}", response);
/// ```
pub fn bad_request_with_headers(headers: HashMap<&str, &str>) -> String {
  json!({
      "status": "Bad Request",
      "code": 400,
      "description": "The server cannot process the request due to malformed syntax",
      "headers": headers
  })
  .to_string()
}
