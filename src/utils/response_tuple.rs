use crate::utils::json_formatter::JsonFormatter;
use serde_json::json;
/// A struct representing the details of an HTTP response code.
/// Provides a tuple-like representation for compatibility with older APIs.
///
/// # Fields
/// - `std_code`: The standard HTTP response code (e.g., 404, 200).
/// - `std_name`: The name associated with the standard code (e.g., "Not Found").
/// - `int_code`: An optional internal code for additional error categorization.
/// - `int_name`: An optional name for the internal code.
/// - `desc`: A description of the response.
///
/// # Example
/// ```
/// use simbld_http::utils::response_tuple::ResponseTuple;
///
/// let response = ResponseTuple {
///     std_code: 404,
///     std_name: "Not Found",
///     int_code: Some(1001),
///     int_name: Some("Custom Error"),
///     desc: "The requested resource is missing.",
/// };
/// ```
#[derive(Debug)]
pub struct ResponseTuple {
  pub std_code: u16,
  pub std_name: &'static str,
  pub int_code: Option<u16>,
  pub int_name: Option<&'static str>,
  pub desc: &'static str,
}

impl JsonFormatter for ResponseTuple {
  fn to_json_response(&self) -> serde_json::Value {
    json!({
        "standard http code": {
            "code": self.std_code,
            "name": self.std_name
        },
        "internal http code": {
            "code": self.int_code.unwrap_or(0),
            "name": self.int_name.unwrap_or("")
        },
        "description": self.desc
    })
  }
}
