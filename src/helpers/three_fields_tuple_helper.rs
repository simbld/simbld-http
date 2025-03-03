use crate::helpers::http_code_helper::HttpCode;
use crate::helpers::tuple_traits::IntoThreeFieldsTuple;
use serde::Serialize;

/// SimpleTuple represents a simplified view of HttpCode with three fields.
#[derive(Debug, Serialize)]
pub struct ThreeFieldsTuple {
    pub code: u16,
    pub name: &'static str,
    pub description: &'static str,
}

impl ThreeFieldsTuple {
    /// Create a new ThreeFieldsTuple from a HttpCode.
    pub fn from_http_code(http_code: &HttpCode) -> Self {
        Self {
            code: http_code.standard_code,
            name: http_code.standard_name,
            description: http_code.unified_description,
        }
    }
}

impl IntoThreeFieldsTuple for HttpCode {
    fn into_three_fields_tuple(self) -> ThreeFieldsTuple {
        ThreeFieldsTuple::from_http_code(&self)
    }
}
