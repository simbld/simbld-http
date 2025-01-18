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
macro_rules! generate_responses_functions {
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
  use crate::helpers::to_u16_helper::ToU16;
  use crate::responses::ResponsesTypes;

  #[test]
  fn test_to_u16() {
    let success_code = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let redirection_code = ResponsesTypes::Redirection(ResponsesRedirectionCodes::MovedPermanently);
    assert_eq!(success_code.to_u16(), 200);
    assert_eq!(redirection_code.to_u16(), 301);
  }

  #[test]
  fn test_from_u16() {
    let status = ResponsesTypes::from_u16(400);
    assert_eq!(status, Some(ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest)));
  }

  #[test]
  fn test_as_tuple() {
    let code = ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::InvalidURL);
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::NineFields(
        400,
        "Bad Request",
        "Invalid URL encountered by crawler.",
        786,
        "Invalid URL",
        110,
        "req-13",
        "user-13",
        "status-13"
      )
    );
  }

  #[test]
  fn test_as_json() {
    let code = ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::RobotsTemporarilyUnavailable);
    let json_result = code.as_json();
    let expected = serde_json::json!({
        "standard http code": {
            "code": 503,
            "name": "Service Unavailable"
        },
        "internal http code": {
            "code": 741,
            "name": "Robots Temporarily Unavailable"
        },
        "description": "Robots temporarily unavailable.",
        "metadata": {
            "meta1": 103,
            "meta2": "req-6",
            "meta3": "user-6",
            "meta4": "status-6"
        }
    });
    assert_eq!(json_result, expected);
  }

  #[test]
  fn test_into_tuple() {
    let code = ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ProgrammableRedirection);
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 302);
    assert_eq!(std_name, "Found");
  }
}
