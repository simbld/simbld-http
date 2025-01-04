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
      int_code,
      int_name,
      desc,
    }
  }
}
