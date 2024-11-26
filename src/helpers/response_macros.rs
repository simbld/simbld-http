use crate::responses::*;
use actix_web::{http::StatusCode, HttpResponse};
use inflector::Inflector;
use strum::IntoEnumIterator;

/// Dynamically generate functions for HTTP statuses
macro_rules! generate_http_response_functions {
    ($($enum:ty),*) => {
        $(
            impl $enum {
                pub fn generate_responses() {
                    for variant in <$enum>::iter() {
                        let function_name = variant.to_string().to_snake_case();
                        let code: u16 = variant.into();

                        let response_function = quote::quote! {
                            pub fn #function_name() -> HttpResponse {
                                HttpResponse::build(StatusCode::from_u16(#code).unwrap()).finish()
                            }
                        };
                        println!("{}", response_function);
                    }
                }
            }
        )*
    };
}

generate_http_response_functions!(
  ResponsesInformationalCodes,
  ResponsesSuccessCodes,
  ResponsesRedirectionCodes,
  ResponsesClientCodes,
  ResponsesServerCodes,
  ResponsesServiceCodes,
  ResponsesCrawlerCodes,
  ResponsesLocalApiCodes
);
