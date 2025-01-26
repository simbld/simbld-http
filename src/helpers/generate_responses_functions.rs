/// Macro to generate response-related functions and implementations for enums.
///
/// The macro generates a new enum with the specified name and enum variants.
/// It also generates methods to convert between the enum variants and their corresponding HTTP status codes.
///
/// The macro generates the following methods:
/// - `to_http_code`: Converts the enum variant to its corresponding `HttpCode`.
/// - `to_u16`: Returns the "standard" code (std_code) as `u16`.
/// - `from_u16`: Attempts to construct an enum variant from a given `u16` code.
/// - `as_tuple`: Converts the enum variant into a tuple representation.
/// - `as_json`: Converts the enum variant into a JSON representation.
///
/// The generated enum and methods can be used to simplify handling and conversion of HTTP response codes in Rust.
///
/// # Example
///
/// ```rust
/// generate_responses_functions! {
///   ExampleEnum,
///   Variant1 => (200, "OK", "Description", 1001, "InternalName")
/// }
/// ```
///
/// # Arguments
///
/// - `$enum_name`: The name of the enum.
/// - `$variant => ($std_code, $std_name, $desc, $int_code, $int_name)`: Variants with associated data.

#[macro_export]
macro_rules! generate_responses_functions {
    ($enum_name:ident, $($variant:ident => ($std_code:expr, $std_name:expr, $description:expr, $int_code:expr, $int_name:expr)),+ $(,)?) => {
        #[derive(Debug, PartialEq)]
        pub enum $enum_name {
            $($variant),*
        }

        impl $enum_name {
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

            pub fn to_u16(&self) -> u16 {
                match self {
                    $(
                        Self::$variant => $std_code,
                    )*
                }
            }

            pub fn from_u16(code: u16) -> Option<Self> {
                match code {
                    $(
                        $std_code => Some(Self::$variant),
                        $int_code => Some(Self::$variant),
                    )+
                    _ => None,
                }
            }

            pub fn as_tuple(&self) -> (u16, &'static str, &'static str, u16, &'static str) {
                self.to_http_code().as_tuple()
            }

            pub fn as_json(&self) -> serde_json::Value {
                self.to_http_code().as_json()
            }
        }

        impl From<$enum_name> for u16 {
            fn from(value: $enum_name) -> Self {
                value.to_u16()
            }
        }

        impl From<$enum_name> for (u16, &'static str) {
            fn from(value: $enum_name) -> Self {
                (value.to_u16(), value.as_tuple().2)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::UnifiedTuple;
    use crate::responses::ResponsesCrawlerCodes;
    use serde_json::json;
    
    #[test]
    fn test_crawler_codes_to_u16() {
        let response = ResponsesCrawlerCodes::ParsingErrorHeader;
        assert_eq!(response.to_u16(), 400);
    }
    
    #[test]
    fn test_crawler_codes_from_u16() {
        let status = ResponsesCrawlerCodes::from_u16(400);
        assert_eq!(status, Some(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader));
    }
    
    #[test]
    fn test_crawler_codes_as_tuple() {
        let code = ResponsesCrawlerCodes::InvalidURL;
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
    
    #[test]
    fn test_crawler_codes_as_json() {
        let code = ResponsesCrawlerCodes::RobotsTemporarilyUnavailable;
        let json_result = code.as_json();
        let expected_json = json!({
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
        assert_eq!(json_result, expected_json);
    }
    
    #[test]
    fn test_crawler_codes_into_tuple() {
        let code = ResponsesCrawlerCodes::ProgrammableRedirection;
        let (std_code, std_name): (u16, &'static str) = code.into();
        assert_eq!(std_code, 302);
        assert_eq!(std_name, "Found");
    }
}
