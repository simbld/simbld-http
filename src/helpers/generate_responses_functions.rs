/// File: generate_responses_functions.rs
/// This file defines a macro that generates response-related functions and implementations for enums.
/// Example usage:
/// ```rust
/// use simbld_http::generate_responses_functions;
/// use simbld_http::helpers::unified_tuple_helper;
/// use simbld_http::ResponsesClientCodes::*;
/// use simbld_http::helpers::to_u16_trait::ToU16;
/// use simbld_http::helpers::unified_tuple_helper::UnifiedTuple;
/// use serde::Serialize;
///
/// generate_responses_functions! {
///     "Client errors",
///     ResponsesClientCodes,
///     BadRequest => (400, "Bad Request", "Example of a bad request.", 444, "Bad Request"),
///     MethodNotFound => (404, "Not Found", "Resource not found.", 404, "NotFound"),
///     UnrecoverableError => (456, "Unrecoverable Error", "Unrecoverable error.", 456, "UnrecoverableError"),
///     NoResponse => (444, "No Response", "No response from the server.", 444, "NoResponse"),
/// }
/// assert_eq!(BadRequest.to_u16(), 400);
/// assert_eq!(BadRequest.to_http_code().standard_name, "Bad Request");
/// assert_eq!(BadRequest.description(), "Example of a bad request.");
/// assert_eq!(BadRequest.internal_code(), 444);
/// assert_eq!(BadRequest.as_tuple(), UnifiedTuple {
///    standard_code: 400,
///   standard_name: "Bad Request",
///  unified_description: "Example of a bad request.",
/// internal_code: Some(444),
/// internal_name: Option::from("Bad Request"),
/// });
/// assert_eq!(BadRequest.as_json(), serde_json::json!({
///     "type": "Client errors",
///     "details": {
///         "standard_code": 400,
///         "standard_name": "Bad Request",
///         "unified_description": "Example of a bad request.",
///         "internal_code": 444,
///         "internal_name": "Bad Request",
///     },
/// }));
/// ```
#[macro_export]
macro_rules! generate_responses_functions {
    (
        $doc_family:expr,
        $enum_name:ident,
        $first_variant:ident => ($std_code_first:expr, $std_name_first:expr, $desc_first:expr, $int_code_first:expr, $int_std_name_first:expr)
        $(, $variant:ident => ($std_code:expr, $std_name:expr, $desc:expr, $int_code:expr, $int_name:expr) )* $(,)?
    ) => {
        /// Enum representing HTTP response status codes and their descriptions.
        #[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
        #[doc = $doc_family]
        #[doc = concat!(
            "\n\nEnum representing HTTP response status codes and descriptions for `",
            stringify!($enum_name),
            "`. This file defines the following methods:\n",
            "- `to_http_code`: Converts the enum variant to its corresponding `HttpCode`.\n",
            "- `to_u16`: Returns the standard code as `u16`.\n",
            "- `from_u16`: Constructs an enum variant from a given `u16` code (first matching standard, then internal).\n",
            "- `from_internal_code`: Constructs an enum variant from a given internal `u16` code.\n",
            "- `as_tuple`: Returns a unified tuple representation.\n",
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
            $($variant,)*
        }

        /// Custom Serialize implementation to include both "type" and "details" fields.
       impl serde::Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                // Convert the enum variant into its unified HTTP code tuple.
                // Example: let unified = self.to_http_code().as_unified_tuple();
                let unified = self.to_http_code().as_unified_tuple();
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(stringify!($enum_name), 2)?;
                // Serialize the "type" field using the provided doc_family.
                state.serialize_field("type", $doc_family)?;
                // Serialize the "details" field using the custom JSON structure from as_json().
                state.serialize_field("details", &unified.as_json())?;
                state.end()
            }
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
                    Self::$first_variant => crate::helpers::http_code_helper::HttpCode::new(
                        $std_code_first,
                        $std_name_first,
                        $desc_first,
                        $int_code_first,
                        $int_std_name_first
                    ),
                    $(
                        Self::$variant => crate::helpers::http_code_helper::HttpCode::new(
                            $std_code,
                            $std_name,
                            $desc,
                            $int_code,
                            $int_name
                        ),
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

            /// Constructs an enum variant from a given u16 code.
            /// It first checks standard codes, then internal codes.
            pub fn from_u16(code: u16) -> Option<Self> {
                if code == $std_code_first { return Some(Self::$first_variant); }
                $(
                    if code == $std_code { return Some(Self::$variant); }
                )*
                if code == $int_code_first { return Some(Self::$first_variant); }
                $(
                    if code == $int_code { return Some(Self::$variant); }
                )*
                None
            }

            /// Attempts to create a standardized enumeration variant from the HTTP code `U16 '' which is internal to it.
            /// returns "none" if no variant corresponds.
            pub fn from_internal_code(code: u16) -> Option<Self> {
              match code {
                  $int_code_first => Some(Self::$first_variant),
                  $(
                      $int_code => Some(Self::$first_variant),
                  )*
                  _ => None,
              }
            }

            /// Returns the internal code (u16) of the response.
            pub fn internal_code(&self) -> u16 {
                match self {
                    Self::$first_variant => $int_code_first,
                    $(
                        Self::$variant => $int_code,
                    )*
                }
            }

            /// Returns a unified tuple representation.
            pub fn as_tuple(&self) -> crate::helpers::unified_tuple_helper::UnifiedTuple {
                self.to_http_code().as_unified_tuple()
            }

            /// Returns a JSON representation of the response code.
            pub fn as_json(&self) -> serde_json::Value {
                serde_json::to_value(self).unwrap()
            }
        }

        /// Implementation for converting the enum into a tuple `(u16, &'static str)`.
        impl crate::helpers::tuple_traits::IntoTwoFieldsTuple for $enum_name {
            fn into_two_fields_tuple(self) -> crate::helpers::two_fields_tuple_helper::TwoFieldsTuple {
                let http_code = self.to_http_code();
                http_code.into_two_fields_tuple()
            }
        }

        /// Implementation for converting the enum into a tuple '(u16, &'static str, &'static str)'.
        impl crate::helpers::tuple_traits::IntoThreeFieldsTuple for $enum_name {
            fn into_three_fields_tuple(self) -> crate::helpers::three_fields_tuple_helper::ThreeFieldsTuple {
                let http_code = self.to_http_code();
                http_code.into_three_fields_tuple()
            }
        }

        /// Implementation of the `ToU16` trait for the enum.
        impl ToU16 for $enum_name {
            fn to_u16(&self) -> u16 {
                match self {
                    Self::$first_variant => $std_code_first,
                    $(
                        Self::$variant => $std_code,
                    )*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::tuple_traits::{IntoThreeFieldsTuple, IntoTwoFieldsTuple};
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesClientCodes;
    use crate::ResponsesSuccessCodes;
    use serde_json::json;

    #[test]
    fn test_description() {
        let ex = ResponsesClientCodes::BadRequest;
        assert_eq!(ex.description(), "The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing)." );
    }

    #[test]
    fn test_to_http_code() {
        let ex = ResponsesClientCodes::PageExpired;
        let expected = crate::helpers::http_code_helper::HttpCode::new(
            401,
            "Unauthorized",
            "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.",
            419,
            "PageExpired");

        assert_eq!(ex.to_http_code(), expected);
    }

    #[test]
    fn test_to_u16() {
        let ex = ResponsesClientCodes::BadRequest;
        assert_eq!(ex.to_u16(), 400);
    }

    #[test]
    fn test_from_internal_code() {
        assert_eq!(
            ResponsesClientCodes::from_internal_code(444),
            Some(ResponsesClientCodes::BadRequest)
        );
        assert_eq!(
            ResponsesClientCodes::from_internal_code(445),
            Some(ResponsesClientCodes::BadRequest)
        );
        assert_eq!(ResponsesClientCodes::from_internal_code(492), None);
    }

    #[test]
    fn test_as_tuple() {
        let ex = ResponsesClientCodes::BadRequest;
        let expected = UnifiedTuple {
            standard_code: 400,
            standard_name: "Bad Request",
            unified_description:
                "The server cannot or will not process the request due to something \
                that is perceived to be a client error (e.g., malformed request \
                syntax, invalid request message framing, or deceptive request \
                routing).",
            internal_code: None,
            internal_name: None,
        };

        assert_eq!(ex.as_tuple(), expected);
    }

    #[test]
    fn test_as_json() {
        // Use the BadRequest variant from the client error responses.
        let ex = ResponsesClientCodes::BadRequest;
        let ex_json = ex.as_json();

        // Define the expected JSON with exact field names.
        let expected_json = json!({
            "type": "Client errors",
            "details": {
                "standard http code": {
                    "code": 400,
                    "name": "Bad Request"
                },
                "description": "The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).",
                "internal http code": {
                    "code": null,
                    "name": null
                }
            }
        });

        // Assert that both JSON structures are identical.
        assert_eq!(ex_json, expected_json);
    }

    #[test]
    fn test_from_u16() {
        assert_eq!(ResponsesClientCodes::from_u16(400), Some(ResponsesClientCodes::BadRequest));
        assert_eq!(
            ResponsesClientCodes::from_u16(456),
            Some(ResponsesClientCodes::UnrecoverableError)
        );
        assert_eq!(ResponsesClientCodes::from_u16(500), None);
    }

    #[test]
    fn test_internal_code() {
        let ex = ResponsesClientCodes::NoResponse;
        assert_eq!(ex.internal_code(), 444);
    }

    #[test]
    fn test_into_two_fields_tuple() {
        // Par exemple, pour un code client "BadRequest"
        let response = ResponsesClientCodes::BadRequest;
        let tuple = response.into_two_fields_tuple();
        let json_result = serde_json::to_value(&tuple).unwrap();

        let expected = json!({
            "code": 400,
            "name": "Bad Request"
        });
        assert_eq!(json_result, expected);
    }

    #[test]
    fn test_into_three_fields_tuple() {
        // Par exemple, pour une réponse "Ok" du SuccessCodes
        let response = ResponsesSuccessCodes::Ok;
        let tuple = response.into_three_fields_tuple();
        let json_result = serde_json::to_value(&tuple).unwrap();

        let expected = json!({
            "code": 200,
            "name": "OK",
            "description": "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
        });
        assert_eq!(json_result, expected);
    }
}
