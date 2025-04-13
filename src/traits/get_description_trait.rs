use crate::responses::ResponsesTypes;
use crate::{
    ResponsesClientCodes, ResponsesCrawlerCodes, ResponsesInformationalCodes,
    ResponsesLocalApiCodes, ResponsesRedirectionCodes, ResponsesServerCodes, ResponsesServiceCodes,
    ResponsesSuccessCodes,
};

pub trait GetDescription {
    fn get_description_field(&self, field: &str) -> Option<&'static str>;
}

/// This macro implements the `GetDescription` trait for a given type.
macro_rules! impl_get_description {
    ($type_name:ty) => {
        impl GetDescription for $type_name {
            fn get_description_field(&self, field: &str) -> Option<&'static str> {
                match field {
                    "Description" => Some(self.get_description()),
                    _ => None,
                }
            }
        }
    };
}

impl_get_description!(ResponsesTypes);
impl_get_description!(ResponsesInformationalCodes);
impl_get_description!(ResponsesSuccessCodes);
impl_get_description!(ResponsesRedirectionCodes);
impl_get_description!(ResponsesClientCodes);
impl_get_description!(ResponsesServerCodes);
impl_get_description!(ResponsesServiceCodes);
impl_get_description!(ResponsesCrawlerCodes);
impl_get_description!(ResponsesLocalApiCodes);
