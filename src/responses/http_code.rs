use serde_json::json;

/// Represents a detailed HTTP code with its associated properties.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpCode {
  pub standard_code: u16,
  pub standard_name: &'static str,
  pub description: &'static str,
  pub internal_code: u16,
  pub internal_name: &'static str,
}

/// Represents a unified structure with five fields for non-standard response metadata.
impl HttpCode {
  /// Creates a new `HttpCode`.
  pub fn as_combined_fields(&self) -> serde_json::Value {
    if self.standard_code == self.internal_code {
      json!({
          "code": self.standard_code,
          "name": self.standard_name,
          "description": self.description
      })
    } else {
      json!({
          "standard_http_code": {
              "code": self.standard_code,
              "name": self.standard_name
          },
          "internal_http_code": {
              "code": self.internal_code,
              "name": self.internal_name
          },
          "description": self.description
      })
    }
  }

  /// Converts the `HttpCode` into a JSON representation.
  pub fn as_json(&self) -> serde_json::Value {
    serde_json::json!({
        "standard_http_code": {
            "code": self.standard_code,
            "name": self.standard_name
        },
        "internal_http_code": {
            "code": self.internal_code,
            "name": self.internal_name
        },
        "description": self.description
    })
  }

  /// Returns the tuple representation of the `HttpCode`.
  pub fn as_tuple(&self) -> (&u16, &&'static str, &&'static str, &u16, &&'static str) {
    (
      &self.standard_code,
      &self.standard_name,
      &self.description,
      &self.internal_code,
      &self.internal_name,
    )
  }
}

/// Represents a list of detailed HTTP codes.
pub struct HttpCodeList {
  pub http_codes: Vec<HttpCode>,
}

impl HttpCodeList {
  /// Creates a new `HttpCodeList`.
  pub fn new(http_codes: Vec<HttpCode>) -> Self {
    Self { http_codes }
  }

  /// Converts the `HttpCodeList` into a JSON representation.
  pub fn as_json(&self) -> serde_json::Value {
    let mut http_codes_json = Vec::new();
    for http_code in &self.http_codes {
      http_codes_json.push(http_code.as_json());
    }
    serde_json::json!({ "http_codes": http_codes_json })
  }

  /// Returns the list of tuples representation of the `HttpCodeList`.
  pub fn as_tuples(&self) -> Vec<(u16, &'static str, &'static str, u16, &'static str)> {
    let mut http_codes_tuples = Vec::new();
    for http_code in &self.http_codes {
      http_codes_tuples.push(http_code.as_tuple());
    }
    http_codes_tuples
  }
}
