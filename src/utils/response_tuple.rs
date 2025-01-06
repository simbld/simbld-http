/// The function `as_struct` constructs a `ResponseTuple` struct based on certain key-value pairs and default values.
///
/// Arguments:
///
/// * `key`: The `key` parameter in the `get_or_default` function is a reference to a string that represents the key used to retrieve a value from a data structure. This function attempts to get the value associated with the given key and returns it if it exists. If the value does not exist, it
/// * `default`: The `default` parameter in the `get_or_default` function is used to provide a default value in case the key is not found in the data structure being accessed. If the key is not found, the function will return the default value provided instead.
///
/// Returns:
///
/// The `as_struct` function returns a `ResponseTuple` struct based on the data extracted from the input. The function constructs a `ResponseTuple` object with the following fields:
fn get_or_default(&self, key: &str, default: &'static str) -> &'static str {
  self.get_str(key).unwrap_or(default)
}

pub struct ResponseTuple {
  pub std_code: u16,
  pub std_name: &'static str,
  pub int_code: Option<u16>,
  pub int_name: Option<&'static str>,
  pub desc: &'static str,
}

pub fn as_struct(&self) -> ResponseTuple {
  let std_code = *self as u16;
  let int_code = self.get_str("internal_code").and_then(|s| s.parse::<u16>().ok());
  let std_name = self.get_or_default("http_standard_name", "No Http Standard Name");
  let int_name = self.get_or_default("internal_name", "No Internal name");
  let desc = self.get_or_default("description", "No Description");

  if std_code == int_code {
    ResponseTuple {
      std_code,
      std_name,
      int_code: None,
      int_name: None,
      desc,
    }
  } else {
    ResponseTuple {
      std_code,
      std_name,
      int_code: Some(int_code),
      int_name: Some(int_name),
      desc,
    }
  }
}
