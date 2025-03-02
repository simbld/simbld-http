use serde::Serialize;

/// SimpleTuple represents a simplified view of HttpCode with three fields.
#[derive(Debug, Serialize)]
pub struct TreeFieldsTuple {
    pub code: u16,
    pub name: &'static str,
    pub description: &'static str,
}

impl TreeFieldsTuple {
    /// Create a TreeFieldsTuple from a HttpCode.
    pub fn from_http_code(http_code: &crate::helpers::http_code_helper::HttpCode) -> Self {
        TreeFieldsTuple {
            code: http_code.standard_code,
            name: http_code.standard_name,
            description: http_code.unified_description,
        }
    }
}
