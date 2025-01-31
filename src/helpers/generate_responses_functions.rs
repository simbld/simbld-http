// Macro to generate response-related functions and implementations for enums.
//
// The macro generates a new enum with the specified name and enum variants.
// It also generates methods to convert between the enum variants and their corresponding HTTP status codes.
//
// # Example
//
// ```rust
// generate_responses_functions! {
//   ExampleEnum,
//   Variant1 => (200, "OK", "Description", 1001, "InternalName")
// }
// ```
//
// # Arguments
//
// - `$enum_name`: The name of the enum.
// - `$variant => ($std_code, $std_name, $desc, $int_code, $int_name)`: Variants with associated data.
// - `$doc_family`: The description for the family of response codes.

#[macro_export]
macro_rules! generate_responses_functions {
    (
        $doc_family:expr,
        $enum_name:ident,
        $(
            $variant:ident => ($std_code:expr, $std_name:expr, $description:expr, $int_code:expr, $int_name:expr)
        ),+ $(,)?
    ) => {
        #[derive(Debug, PartialEq)]
        #[doc = $doc_family]
        /// Enum representing HTTP response status codes and descriptions for `$enum_name`.
        ///
        /// This file defines the `$enum_name` methods:
        /// - `to_http_code`: Converts the enum variant to its corresponding `HttpCode`.
        /// - `to_u16`: Returns the "standard" code (std_code) as `u16`.
        /// - `from_u16`: Attempts to construct an enum variant from a given `u16` code.
        /// - `as_tuple`: Converts the enum variant into a tuple representation.
        /// - `as_json`: Converts the enum variant into a JSON representation.
        ///
        /// # Example
        /// ```
        /// use simbld_http::responses::$enum_name;
        ///
        /// let example = $enum_name::$variant;
        /// assert_eq!(example.to_u16(), $std_code);
        /// assert_eq!(example.as_tuple(), ($std_code, $std_name, $description, $int_code, $int_name));
        /// ```
        pub enum $enum_name {
            $($variant),+
        }

        impl $enum_name {
            /// Returns the description of the response code.//
              pub fn description(&self) -> &'static str {
        match self {
            $(
                Self::$variant => $description,
            )+
        }
    }

            /// Converts the enum variant to its corresponding `HttpCode`.
            pub fn to_http_code(&self) -> crate::responses::http_code::HttpCode {
                match self {
                    $(
                        Self::$variant => crate::responses::http_code::HttpCode::new(
                            $std_code,
                            $std_name,
                            $description,
                            $int_code,
                            $int_name,
                        ),
                    )+
                }
            }

            /// Returns the "standard" code (`std_code`) as `u16`.
            pub fn to_u16(&self) -> u16 {
                match self {
                    $(
                        Self::$variant => $std_code,
                    )*
                }
            }

            /// Attempts to construct an enum variant from a given `u16` code.
            pub fn from_u16(code: u16) -> Option<Self> {
                match code {
                    $(
                        $std_code => Some(Self::$variant),
                        $int_code => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            /// Converts the enum variant into a tuple representation.
          pub fn as_http_code(&self) -> crate::responses::http_code::HttpCode {
                self.to_http_code()
            }

            /// Converts the enum variant into a JSON representation.
          pub fn as_json(&self) -> serde_json::Value {
                let http_code = self.to_http_code();

                http_code.as_json()
            }
        }

            /// Converts the enum variant in a code http representation.
        impl From<$enum_name> for u16 {
            fn from(value: $enum_name) -> Self {
                value.to_u16()
            }
        }

            /// Converts the enum variant into a tuple representation.
        impl From<$enum_name> for (u16, &'static str) {
            fn from(value: $enum_name) -> Self {
                (value.to_u16(), value.as_tuple().2)
            }
        }
    };
    }

#[cfg(test)]
mod tests {
  use serde_json::json;
  
  #[test]
  fn test_to_u16_and_from_u16() {
    assert_eq!(ResponsesClientCodes::BadRequest.to_u16(), 400);
    assert_eq!(ResponsesClientCodes::from_u16(400), Some(ResponsesClientCodes::BadRequest));
    assert_eq!(ResponsesClientCodes::from_u16(550), Some(ResponsesClientCodes::CustomClientError));
    assert_eq!(ResponsesClientCodes::from_u16(999), None); // Code non valide
  }

  #[test]
  fn test_as_json() {
    let response_code = ResponsesClientCodes::BadRequest;
    let result = response_code.as_json();
    assert_eq!(
      result,
      json!({
          "code": 400,
          "name": "Bad Request",
          "description": "The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax)."
      })
    );

    let response_code_custom = ResponsesClientCodes::CustomClientError;
    let result_custom = response_code_custom.as_json();
    assert_eq!(
      result_custom,
      json!({
          "std_code": 450,
          "std_name": "Custom Client Error",
          "description": "This is a custom client error that does not map directly to standard HTTP code.",
          "int_code": 550,
          "int_name": "Internal Custom Error"
      })
    );
  }

  #[test]
  fn test_as_tuple() {
    let response_code = ResponsesClientCodes::BadRequest;
    let result = response_code.as_tuple();
    assert_eq!(
      result,
      UnifiedTuple::ThreeFields(
        400,
        "Bad Request",
        "The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax)."
      )
    );

    let response_code_custom = ResponsesClientCodes::CustomClientError;
    let result_custom = response_code_custom.as_tuple();
    assert_eq!(
      result_custom,
      UnifiedTuple::FiveFields(
        450,
        "Custom Client Error",
        "This is a custom client error that does not map directly to standard HTTP code.",
        550,
        "Internal Custom Error"
      )
    );
  }

  #[test]
  fn test_crawler_codes_into_tuple() {
    let variant_tuple: (u16, &'static str) = ResponsesCrawlerCodes::ProgrammableRedirection.into();
    assert_eq!(variant_tuple, (302, "Found"));
  }
}
