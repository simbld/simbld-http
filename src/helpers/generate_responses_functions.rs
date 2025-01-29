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
          pub fn as_tuple(&self) -> UnifiedTuple {
    let http_code = self.to_http_code();

    if http_code.internal_code == http_code.standard_code {
        UnifiedTuple::ThreeFields(
            http_code.standard_code,
            http_code.standard_name,
            self.description(), // Appel à la méthode centralisée
        )
    } else {
        UnifiedTuple::FiveFields(
            http_code.standard_code,
            http_code.standard_name,
            self.description(), // Appel à la méthode centralisée
            http_code.internal_code,
            http_code.internal_name,
        )
    }
}

            /// Converts the enum variant into a JSON representation.
          pub fn as_json(&self) -> serde_json::Value {
    let http_code = self.to_http_code();

    if http_code.internal_code == http_code.standard_code {
        serde_json::json!({
            "code": http_code.standard_code,
            "name": http_code.standard_name,
            "description": self.description(),
        })
    } else {
        serde_json::json!({
            "std_code": http_code.standard_code,
            "std_name": http_code.standard_name,
            "description": self.description(),
            "int_code": http_code.internal_code,
            "int_name": http_code.internal_name,
        })
    }
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
        }
    }

#[cfg(test)]
mod tests {
  use crate::responses::ResponsesCrawlerCodes;
  use crate::responses::UnifiedTuple;
  
  #[test]
  fn test_crawler_codes_to_u16() {
    assert_eq!(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader.to_u16(), 400);
  }

  #[test]
  fn test_crawler_codes_from_u16() {
    assert_eq!(
      ResponsesCrawlerCodes::from_u16(400),
      Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader)
    );
    assert_eq!(ResponsesCrawlerCodes::from_u16(999), None);
  }

  #[test]
  fn test_crawler_codes_as_tuple() {
    // Case where the internal and standard codes are identical
    assert_eq!(
      ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader.as_tuple(),
      UnifiedTuple::ThreeFields(400, "Parsing Error", "Parsing error: Unfinished header.")
    );

    // Case where the internal and standard codes are different
    assert_eq!(
      ResponsesCrawlerCodes::ExcludedByRobotsTxtFile.as_tuple(),
      UnifiedTuple::FiveFields(
        403,
        "Forbidden",
        "Access denied by Robots.txt file.",
        700,
        "Excluded by Robots.txt"
      )
    );
  }

  #[test]
  fn test_crawler_codes_as_json() {
    // Case where the internal and standard codes are identical
    let json_value = ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader.as_json();
    let expected_json = serde_json::json!({
        "code": 400,
        "name": "Parsing Error",
        "description": "Parsing error: Unfinished header."
    });
    assert_eq!(json_value, expected_json);

    // Case where the internal and standard codes are different
    let json_value = ResponsesCrawlerCodes::ExcludedByRobotsTxtFile.as_json();
    let expected_json = serde_json::json!({
        "std_code": 403,
        "std_name": "Forbidden",
        "description": "Access denied by Robots.txt file.",
        "int_code": 700,
        "int_name": "Excluded by Robots.txt"
    });
    assert_eq!(json_value, expected_json);
  }

  #[test]
  fn test_crawler_codes_into_tuple() {
    let variant_tuple: (u16, &'static str) = ResponsesCrawlerCodes::ProgrammableRedirection.into();
    assert_eq!(variant_tuple, (302, "Found"));
  }
}
