/// Trait définissant les fonctions pour générer des réponses HTTP.
pub trait ResponseFunctions {
  /// Génère des réponses HTTP pour chaque variant de l'enum.
  fn generate_responses();

  /// Génère des réponses HTTP avec métadonnées pour chaque variant de l'enum.
  fn generate_responses_with_metadata();
}

/// Dynamically generate functions for HTTP status codes
#[macro_export]
macro_rules! generate_http_response_functions {
    ($($enum:ty),*) => {
        $(
            impl crate::helpers::response_functions::ResponseFunctions for $enum
            where
                $enum: strum::IntoEnumIterator + std::fmt::Debug,
            {
                fn generate_responses() {
                    for variant in <$enum>::iter() {
                        let function_name = variant.to_string().to_snake_case();
                        let code: u16 = variant.into();

                        println!(
                            "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}",
                            function_name,
                            code,
                            variant
                        );
                    }
                }

                fn generate_responses_with_metadata() {
                    for variant in <$enum>::iter() {
                        let function_name = variant.to_string().to_snake_case();
                        let code: u16 = variant.into();

                        println!(
                            "fn {}() -> String {{ response_helpers::get_enriched_response_with_metadata({}, None, std::time::Duration::from_millis(100)) }}",
                            function_name,
                            code
                        );
                    }
                }
            }
        )*
    };
}
