use crate::helpers::http_code_helper::HttpCode;

/// Trait to convert various types into an HttpCode.
pub trait IntoHttpCode {
    fn into_http_code(self) -> HttpCode;
}
