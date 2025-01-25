/// Macro to generate response-related functions and implementations for enums.
///
/// Arguments:
/// - `$enum_name`: The name of the enum.
/// - `$variant => ($std_code, $std_name, $desc, $int_code, $int_name)`: Variants with associated data.
///
/// Example usage:
/// ```rust
/// generate_responses_functions! {
///   ExampleEnum,
///   Variant1 => (200, "OK", "Description", 1001, "InternalName")
/// }
/// ```
use crate::responses::ResponsesTypes;

/// Macro to generate response-related enums and their associated methods.
#[macro_export]
macro_rules! generate_responses_functions {
($enum_name:ident, $($variant:ident => ($std_code:expr, $std_name:expr, $description:expr, $int_code:expr, $int_name:expr)),+ $(,)?) => {
/// Enum representing HTTP response codes with standard and optional internal fields.
#[derive(Debug, PartialEq)]
/// Enum representing various HTTP response codes and metadata.
pub enum $enum_name {
	$($variant),*
}

impl $enum_name {
	/// Converts the enum variant to its corresponding `HttpCode`.
	pub fn to_http_code(&self) -> HttpCode {
		match self {
			$(
				Self::$variant => HttpCode::new_internal(
					$std_code,
					$std_name,
					$description,
					$int_code,
					$int_name,
				),
			)+
		}
	}

		/// Returns the "standard" code (std_code) as `u16`.
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
		pub fn as_tuple(&self) -> (u16, &'static str, &'static str, Option<u16>, Option<&'static str>) {
			self.to_http_code().as_tuple()
		}

			/// Converts the enum variant into a JSON representation.
		pub fn as_json(&self) -> serde_json::Value {
			self.to_http_code().as_json()
		}
	}

	/// Allows conversion from the enum to `u16`.
	impl From<$enum_name> for u16 {
		fn from(value: $enum_name) -> Self {
			value.to_u16()
		}
	}

	/// Allows conversion from the enum to `(u16, &'static str)`.
	impl From<$enum_name> for (u16, &'static str) {
		fn from(value: $enum_name) -> Self {
			(value.to_u16(), value.as_tuple().get_description())
		}
	}
};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::ResponsesSuccessCodes;

    #[test]
    fn test_to_u16() {
        let response = ResponsesSuccessCodes::Ok;
        assert_eq!(response.to_u16(), 200);
    }

    #[test]
    fn test_from_u16() {
        let response = ResponsesSuccessCodes::from_u16(200);
        assert_eq!(response, Some(ResponsesSuccessCodes::Ok));
    }

    #[test]
    fn test_as_tuple() {
        let response = ResponsesSuccessCodes::Ok;
        let tuple = response.as_tuple();
        assert_eq!(
            tuple,
            UnifiedTuple::FiveFields(200, "OK", "Request processed successfully.", 1000, "Success")
        );
    }

    #[test]
    fn test_as_json() {
        let response = ResponsesSuccessCodes::Ok;
        let json_result = response.as_json();
        let expected_json = json!({
            "standard http code": {
                "code": 200,
                "name": "OK"
            },
            "internal http code": {
                "code": 1000,
                "name": "Success"
            },
            "description": "Request processed successfully."
        });
        assert_eq!(json_result, expected_json);
    }

    #[test]
    fn test_into_conversion() {
        let response = ResponsesSuccessCodes::Ok;
        let (code, desc): (u16, &'static str) = response.into();
        assert_eq!(code, 200);
        assert_eq!(desc, "Request processed successfully.");
    }
}
