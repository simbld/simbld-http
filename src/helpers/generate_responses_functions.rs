//! Macro to generate response-related functions and implementations for enums.
//!
//! The macro creates an enum using all provided variant definitions and generates
//! methods to convert an enum variant into its corresponding `HttpCode` (which returns a unified tuple
//! with optional internal fields) and a JSON representation.
//!
//! In the automatically generated documentation the example is built using the **first variant**
//! provided.
//!
//! # Example
//!
//! ```rust
//! use simbld_http::responses::ResponsesSuccessCodes;
//!
//! let example = ResponsesSuccessCodes::MethodNotFound;
//! assert_eq!(example.to_u16(), 200);
//! assert_eq!(example.as_tuple(), (200, "OK", "The server does not recognize the request method or lacks the capability to fulfill it, and the response body contains the status of the request, indicating that the server does not recognize the request method or lacks the capability to fulfill it, and the response body may contain the status of the request, the server is unable to process the request due to an unsupported method", 254, "Method Not Found"));
//! ```
//!
//! # Arguments
//!
//! - `$doc_family`: A description string for the family of response codes.
//! - `$enum_name`: The name of the enum to generate.
//! - Then, one or more variant definitions in the form:
//!   - `$variant => ($std_code, $std_std_name, $desc, $int_code, $int_name)`.
//!
//! The **first variant** provided will be used in the documentation example.
#[macro_export]
macro_rules! generate_responses_functions {
    (
        $doc_family:expr,
        $enum_name:ident,
        $first_variant:ident => ($std_code_first:expr, $std_name_first:expr, $desc_first:expr, $int_code_first:expr, $int_std_name_first:expr)
        $(, $variant:ident => ($std_code:expr, $std_name:expr, $desc:expr, $int_code:expr, $int_name:expr) )* $(,)?
    ) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        #[doc = $doc_family]
        #[doc = concat!(
            "\n\nEnum representing HTTP response status codes and descriptions for `",
            stringify!($enum_name),
            "`. This file defines the following methods:\n",
            "- `to_http_code`: Converts the enum variant to its corresponding `HttpCode`.\n",
            "- `to_u16`: Returns the standard code as `u16`.\n",
            "- `from_u16`: Constructs an enum variant from a given `u16` code.\n",
            "- `as_tuple`: Returns a unified tuple representation (internal fields are optional).\n",
            "- `as_json`: Returns a JSON representation.\n\n",
            "# Example\n```rust\nuse simbld_http::responses::",
            stringify!($enum_name),
            ";\n\nlet example = ",
            stringify!($enum_name),
            "::",
            stringify!($first_variant),
            ";\nassert_eq!(example.to_u16(), ",
            stringify!($std_code_first),
            ");\nassert_eq!(example.as_tuple(), (",
            stringify!($std_code_first), ", ",
            stringify!($std_name_first), ", ",
            stringify!($desc_first), ", ",
            stringify!($int_code_first), ", ",
            stringify!($int_std_name_first),
            "));\n```"
        )]
        pub enum $enum_name {
            $first_variant,
            $(
                $variant,
            )*
        }

        impl $enum_name {
            /// Returns the description associated with the response code.
            pub fn description(&self) -> &'static str {
                match self {
                    Self::$first_variant => $desc_first,
                    $(
                        Self::$variant => $desc,
                    )*
                }
            }

            /// Converts the enum variant into its corresponding `HttpCode`.
            pub fn to_http_code(&self) -> crate::helpers::http_code_helper::HttpCode {
                match self {
                    Self::$first_variant => crate::helpers::http_code_helper::HttpCode::new($std_code_first, $std_name_first, $desc_first, $int_code_first, $int_std_name_first),
                    $(
                        Self::$variant => crate::helpers::http_code_helper::HttpCode::new($std_code, $std_name, $desc, $int_code, $int_name),
                    )*
                }
            }

            /// Returns the standard code (u16) of the response.
            pub fn to_u16(&self) -> u16 {
                match self {
                    Self::$first_variant => $std_code_first,
                    $(
                        Self::$variant => $std_code,
                    )*
                }
            }

            /// Attempts to construct an enum variant from a given `u16` code.
            pub fn from_u16(code: u16) -> Option<Self> {
                match code {
                    $std_code_first | $int_code_first => Some(Self::$first_variant),
                    $(
                        $std_code | $int_code => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            /// Returns a unified tuple representation.
            /// (The unified tuple type is defined separately and uses optional internal fields:
            /// if the standard and internal codes are equal, the internal fields are `None`.)
            pub fn as_tuple(&self) -> crate::helpers::unified_tuple_helper::UnifiedTuple {
                self.to_http_code().as_unified_tuple()
            }

            /// Returns a JSON representation of the response code.
            pub fn as_json(&self) -> serde_json::Value {
                self.to_http_code().as_json()
            }
        }

        impl crate::helpers::to_u16_helper::ToU16 for $enum_name {
            fn to_u16(self) -> u16 {
                Self::to_u16(&self)
            }
        }

        impl crate::helpers::from_u16_helper::FromU16 for $enum_name {
            fn from_u16(code: u16) -> Option<Self> {
                Self::from_u16(code)
            }
        }

        impl From<$enum_name> for (u16, &'static str) {
            fn from(value: $enum_name) -> Self {
                let http_code = value.to_http_code();
                (http_code.standard_code, http_code.standard_name)
            }
        }
    };
}
