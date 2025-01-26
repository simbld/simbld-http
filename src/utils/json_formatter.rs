/// Trait to convert a structure into a detailed JSON response.
/// This is used to centralize the logic for JSON generation.
pub trait JsonFormatter {
  /// Converts the implementing structure into a JSON value.
  fn to_json_response(&self) -> serde_json::Value;
}
