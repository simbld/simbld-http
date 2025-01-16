/// representing HTTP codes along with metadata.
///
/// Metadata is always displayed in the JSON representation.
/// Any filtering or conditional display of metadata should be
/// handled on the front-end side.
///
/// ```rust
/// * Example:
///
/// let code = ResponseSuccessCodes::Ok;
/// println!("{}", code.as_json().to_string());
/// ```

#[macro_export]
macro_rules! generate_responses_functions_unified_always_metadata {
    (
        $( #[$meta:meta] )*
        $enum_name:ident,
        $(
            $variant:ident => ($std_code:expr, $std_name:expr, $desc:expr, $int_code:expr, $int_name:expr, $meta1:expr, $meta2:expr, $meta3:expr, $meta4:expr)
        ),* $(,)?
    ) => {
        $( #[$meta] )*
        pub enum $enum_name {
            $($variant),*
        }

        impl $enum_name {
            /// Returns the "standard" code (std_code) as u16.
            pub fn to_u16(&self) -> u16 {
                match self {
                    $(
                        Self::$variant => $std_code,
                    )*
                }
            }

            /// Builds the enum variant from a `u16` matching on the standard code.
            /// Returns the first matching variant (if any).
            pub fn from_u16(code: u16) -> Option<Self> {
                match code {
                    $(
                        $std_code => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            /// Returns a tuple with always 9 fields, including metadata.
            /// The logic is simplified to force metadata display.
            pub fn as_tuple(&self) -> UnifiedTuple {
                match self {
                    $(
                        Self::$variant => UnifiedTuple::NineFields(
                            $std_code,
                            $std_name,
                            $desc,
                            $int_code,
                            $int_name,
                            $meta1,
                            $meta2,
                            $meta3,
                            $meta4
                        ),
                    )*
                }
            }

            /// Returns a JSON object with 9 fields every time (metadata always included).
            pub fn as_json(&self) -> ::serde_json::Value {
                match self {
                    $(
                        Self::$variant => {
                            ::serde_json::json!({
                                "standard http code": { "code": $std_code, "name": $std_name },
                                "internal http code": { "code": $int_code, "name": $int_name },
                                "description": $desc,
                                "metadata": {
                                    "meta1": $meta1,
                                    "meta2": $meta2,
                                    "meta3": $meta3,
                                    "meta4": $meta4
                                }
                            })
                        },
                    )*
                }
            }
        }

        /// Converts the enum into its standard code as `u16`.
        impl ::std::convert::From<$enum_name> for u16 {
            fn from(value: $enum_name) -> Self {
                value.to_u16()
            }
        }
    }
}

/// Represents a unified structure with 9 fields.
#[derive(Debug, PartialEq)]
pub enum UnifiedTuple {
  NineFields(
    u16,
    &'static str,
    &'static str,
    u16,
    &'static str,
    u32,
    &'static str,
    &'static str,
    &'static str,
  ),
}

#[cfg(test)]
mod tests {
  use super::*;

  generate_responses_functions_unified_always_metadata! {
      #[derive(Debug, PartialEq)]
      ResponseCode,
      Success => (200, "OK", "Request succeeded", 200, "OK", 123, "req-1", "user-1", "status-1"),
      NotFound => (404, "Not Found", "Resource not found", 404, "Not Found", 456, "req-2", "user-2", "status-2"),
      InternalError => (500, "Internal Server Error", "An error occurred", 500, "Internal Server Error", 789, "req-3", "user-3", "status-3"),
      CustomError => (600, "Custom Error", "A custom error occurred", 601, "Custom Error", 101, "req-4", "user-4", "status-4")
  }

  #[test]
  fn test_to_u16() {
    assert_eq!(ResponseCode::Success.to_u16(), 200);
    assert_eq!(ResponseCode::NotFound.to_u16(), 404);
  }

  #[test]
  fn test_from_u16() {
    assert_eq!(ResponseCode::from_u16(200), Some(ResponseCode::Success));
    assert_eq!(ResponseCode::from_u16(999), None);
  }

  #[test]
  fn test_as_tuple() {
    let code = ResponseCode::InternalError;
    assert_eq!(
      code.as_tuple(),
      UnifiedTuple::NineFields(
        500,
        "Internal Server Error",
        "An error occurred",
        500,
        "Internal Server Error",
        789,
        "req-3",
        "user-3",
        "status-3"
      )
    );
  }

  #[test]
  fn test_as_json() {
    let code = ResponseCode::Success;
    let json_val = code.as_json();
    assert_eq!(json_val["standard http code"]["code"], 200);
    assert_eq!(json_val["metadata"]["meta1"], 123);
  }

  #[test]
  fn test_into() {
    let code = ResponseCode::CustomError;
    let std_code: u16 = code.into();
    assert_eq!(std_code, 600);
  }
}
