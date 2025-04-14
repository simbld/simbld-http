/// This file defines the UnifiedTuple struct used to represent HTTP response codes uniformly.
/// Example:
/// ```rust
/// use simbld_http::helpers::unified_tuple_helper::UnifiedTuple;
/// let tuple = UnifiedTuple {
///     standard_code: 200,
///     standard_name: "OK",
///     unified_description: "Successful request",
///     internal_code: None,
///     internal_name: None,
/// };
/// assert_eq!(tuple.as_json()["description"], "Successful request");
/// ```
use crate::responses::ResponsesTypes;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct UnifiedTuple {
    /// Standard HTTP code.
    pub standard_code: u16,
    /// Standard HTTP name.
    pub standard_name: &'static str,
    /// Description of the response.
    pub unified_description: &'static str,
    /// Internal HTTP code (None if equal to standard code).
    pub internal_code: Option<u16>,
    /// Internal HTTP name (None if equal to standard name).
    pub internal_name: Option<&'static str>,
}

impl UnifiedTuple {
    pub fn as_json(&self) -> serde_json::Value {
        serde_json::json!({
            "standard http code": {
                "code": self.standard_code,
                "name": self.standard_name,
            },
            "description": self.unified_description,
            "internal http code": {
                "code": self.internal_code,
                "name": self.internal_name,
            },
        })
    }
}

impl From<ResponsesTypes> for UnifiedTuple {
    fn from(response: ResponsesTypes) -> Self {
        let standard_code = response.get_code();
        let standard_name = response.get_description();
        let unified_description = response.get_description();
        UnifiedTuple {
            standard_code,
            standard_name,
            unified_description,
            internal_code: None,
            internal_name: None,
        }
    }
}
