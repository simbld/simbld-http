use crate::responses::{
  ResponsesClientCodes, ResponsesCrawlerCodes, ResponsesInformationalCodes, ResponsesLocalApiCodes,
  ResponsesRedirectionCodes, ResponsesServerCodes, ResponsesServiceCodes, ResponsesSuccessCodes,
  ResponsesTypes,
};
use inflector::Inflector;
use paste::paste;
use strum::IntoEnumIterator;

/// Dynamically generate functions for HTTP statuses
macro_rules! generate_http_response_functions {
    ($($enum:ty),*) => {
        $(
            paste! {
                impl $enum {
                    $(
                        pub fn [<$enum:snake_case>]() -> (u16, &'static str) {
                            let variant = stringify!($enum);
                            let code: u16 = variant.into(); // Convert enum to u16
                            let description = variant.get_str("Description").unwrap_or("No description");
                            (code, description)
                        }
                    )*
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

/// Dynamically generate functions for HTTP statuses with metadata.
macro_rules! generate_http_response_with_metadata {
    ($($enum:ty),*) => {
        $(
            paste! {
                impl $enum {
                    $(
                        pub fn [<$enum:snake_case>_with_metadata]() -> String {
                            let variant = stringify!($enum);
                            let code: u16 = variant.into(); // Convert enum to u16
                            let duration = std::time::Duration::from_millis(100);
                            response_helpers::get_enriched_response_with_metadata(code, None, duration)
                        }
                    )*
                }
            }
        )*
    };
}

generate_http_response_with_metadata!(
  ResponsesInformationalCodes,
  ResponsesSuccessCodes,
  ResponsesRedirectionCodes,
  ResponsesClientCodes,
  ResponsesServerCodes,
  ResponsesServiceCodes,
  ResponsesCrawlerCodes,
  ResponsesLocalApiCodes
);
