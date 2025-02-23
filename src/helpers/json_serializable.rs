/// transforms the object that implements this trait into a JSON value.
/// The returned type is `Serde_json :: Value`.
pub trait JsonSerializable {
    fn as_json(&self) -> serde_json::Value;
}
