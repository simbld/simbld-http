/// Represents a detailed HTTP code with its associated properties.
#[derive(Debug, Clone, PartialEq)]
pub struct HttpCode {
  pub standard_code: u16,
  pub standard_name: &'static str,
  pub description: &'static str,
  pub internal_code: u16,
  pub internal_name: &'static str,
}

impl HttpCode {
  /// Creates a new `HttpCode`.
  pub fn new(
    standard_code: u16,
    standard_name: &'static str,
    description: &'static str,
    internal_code: u16,
    internal_name: &'static str,
  ) -> Self {
    Self {
      standard_code,
      standard_name,
      description,
      internal_code,
      internal_name,
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
  pub fn as_tuple(&self) -> (u16, &'static str, &'static str, u16, &'static str) {
    (
      self.standard_code,
      self.standard_name,
      self.description,
      self.internal_code,
      self.internal_name,
    )
  }
}
