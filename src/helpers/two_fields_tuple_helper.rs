use crate::helpers::http_code_helper::HttpCode;
use crate::traits::tuple_traits::IntoTwoFieldsTuple;
use serde::Serialize;

/// SimpleTuple represents a simplified view of HttpCode with two fields.
#[derive(Debug, Serialize)]
pub struct TwoFieldsTuple {
    pub code: u16,
    pub name: &'static str,
}

impl TwoFieldsTuple {
    /// Create a new TwoFieldsTuple from a HttpCode.
    pub fn from_http_code(http_code: &HttpCode) -> Self {
        Self { code: http_code.standard_code, name: http_code.standard_name }
    }
}

impl IntoTwoFieldsTuple for HttpCode {
    fn into_two_fields_tuple(self) -> TwoFieldsTuple {
        TwoFieldsTuple::from_http_code(&self)
    }
}
