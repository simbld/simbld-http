/// representing HTTP codes along with metadata.
///
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
            $variant:ident => ($std_code:expr, $std_name:expr, $desc:expr, $int_code:expr, $int_name:expr)
        ),* $(,)?
    ) => {
        $( #[$meta] )*
        pub enum $enum_name {
            $($variant),*
        }

        // Wrapper enum
        #[derive(Debug, Clone, PartialEq)]
        pub enum ResponseWrapper {
            Standard($enum_name),
            Custom {
                std_code: u16,
                std_name: &'static str,
                description: &'static str,
                int_code: u16,
                int_name: &'static str
            }
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
                        Self::$variant => UnifiedTuple::FiveFields(
                            $std_code,
                            $std_name,
                            $desc,
                            $int_code,
                            $int_name
                        ),
                    )*
                }
            }

            /// Returns a JSON object with 9 fields every time (metadata always included).
            pub fn as_json(&self) -> Value {
                match self {
                    $(
                        Self::$variant => {
                            json!({
                                "standard http code": { "code": $std_code, "name": $std_name },
                                "internal http code": { "code": $int_code, "name": $int_name },
                                "description": $desc
                            })
                        },
                    )*
                }
            }
        }

        /// Implementing as_json for ResponsesTypes
        impl ResponsesTypes {
            pub fn as_json(&self) -> Value {
                match self {
                    ResponsesTypes::Informational(code) => code.as_json(),
                    ResponsesTypes::Success(code) => code.as_json(),
                    ResponsesTypes::Redirection(code) => code.as_json(),
                    ResponsesTypes::ClientError(code) => code.as_json(),
                    ResponsesTypes::CrawlerError(code) => code.as_json(),
                    ResponsesTypes::LocalApiError(code) => code.as_json(),
                    ResponsesTypes::ServerError(code) => code.as_json(),
                    ResponsesTypes::ServiceError(code) => code.as_json(),
                }
            }
        }

        impl ResponseWrapper {
            pub fn to_u16(&self) -> u16 {
                match self {
                    Self::Standard(code) => code.to_u16(),
                    Self::Custom { std_code, .. } => *std_code
                }
            }

            pub fn as_tuple(&self) -> UnifiedTuple {
                match self {
                    Self::Standard(code) => code.as_tuple(),
                    Self::Custom { std_code, std_name, description, int_code, int_name} => {
                        UnifiedTuple::FiveFields(
                            *std_code, std_name, description, *int_code, int_name
                        )
                    }
                }
            }

            pub fn as_json(&self) -> Value {
                match self {
                    Self::Standard(code) => code.as_json(),
                    Self::Custom { std_code, std_name, description, int_code, int_name} => {
                        json!({
                            "standard http code": { "code": std_code, "name": std_name },
                            "internal http code": { "code": int_code, "name": int_name },
                            "description": description
                        })
                    }
                }
            }
        }

        /// Converts the enum into its standard code as `u16`.
        impl ::std::convert::From<$enum_name> for u16 {
            fn from(value: $enum_name) -> Self {
                value.to_u16()
            }
        }

        /// Converts the enum into a tuple `(u16, &'static str)`.
        impl ::std::convert::From<$enum_name> for (u16, &'static str) {
            fn from(value: $enum_name) -> Self {
                (value.to_u16(), value.get_str("Description").unwrap_or(""))
            }
        }

        ///
        impl ::std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }

    }
}

/// Represents a unified structure with 9 fields.
#[derive(Debug, PartialEq)]
pub enum UnifiedTuple {
  FiveFields(u16, &'static str, &'static str, u16, &'static str),
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::{
    ResponseWrapper, ResponsesClientCodes, ResponsesCrawlerCodes, ResponsesCrawlerCodes,
    ResponsesRedirectionCodes, ResponsesSuccessCodes, ResponsesTypes, UnifiedTuple,
  };

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

  #[cfg(test)]
  mod tests {
    use super::*;
    use crate::responses::{ResponsesCrawlerCodes, ResponsesTypes, UnifiedTuple};

    #[test]
    fn test_as_tuple() {
      let code = ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::InvalidURL);
      let tuple = code.as_tuple();
      assert_eq!(
        tuple,
        UnifiedTuple::FiveFields(
          400,
          "Bad Request",
          "Invalid URL encountered by crawler.",
          786,
          "Invalid URL"
        )
      );
    }
  }

  #[test]
  fn test_as_json() {
    let code = ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::RobotsTemporarilyUnavailable);
    let json_result = code.as_json();
    let expected = json!({
        "standard http code": {
            "code": 503,
            "name": "Service Unavailable"
        },
        "internal http code": {
            "code": 741,
            "name": "Robots Temporarily Unavailable"
        },
        "description": "Robots temporarily unavailable."
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
