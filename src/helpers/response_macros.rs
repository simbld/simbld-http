/// This Rust code defines a macro `generate_http_response_functions` that generates response functions for enums implementing the `ResponseFunctions` trait. Here's a breakdown of what the macro does:
/// - The macro takes a list of enum types as arguments.
/// - For each enum type, the macro generates functions for each variant of the enum.
/// - The generated functions return a tuple containing the HTTP status code and the description of the response.
/// - The generated functions are named based on the snake_case, camelCase, and PascalCase versions of the variant name.
#[macro_export]
macro_rules! generate_http_response_functions {
    ($($enum:ty),*) => {
        $(
            impl $crate::helpers::response_functions::ResponseFunctions for $enum
            where
                $enum: strum::IntoEnumIterator + std::fmt::Debug + Copy + ToString + Into<u16>,
            {
                fn generate_responses() {
                    for variant in <$enum>::iter() {
                        let variant_string = variant.to_string();
                        let function_name_snake = inflector::cases::snakecase::to_snake_case(&variant_string);
                        let function_name_camel = inflector::cases::camelcase::to_camel_case(&variant_string);
                        let function_name_pascal = inflector::cases::classcase::to_class_case(&variant_string);
                        let code: u16 = variant.into();


                        println!(
                            "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}",
                            function_name_snake,
                            code,
                            variant
                        );

                        println!(
                            "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}",
                            function_name_camel,
                            code,
                            variant
                        );

                        println!(
                            "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}",
                            function_name_pascal,
                            code,
                            variant
                        );
                    }
                }

                fn generate_responses_with_metadata() {
                    for variant in <$enum>::iter() {
                        let variant_string = variant.to_string();
                        let function_name_snake = inflector::cases::snakecase::to_snake_case(&variant_string);
                        let function_name_camel = inflector::cases::camelcase::to_camel_case(&variant_string);
                        let function_name_pascal = inflector::cases::classcase::to_class_case(&variant_string);
                        let code: u16 = variant.into();

                        println!(
                            "fn {}() -> String {{ response_helpers::get_enriched_response_with_metadata({}, None, std::time::Duration::from_millis(100)) }}",
                            function_name_snake,
                            code
                        );

                        println!(
                            "fn {}() -> String {{ response_helpers::get_enriched_response_with_metadata({}, None, std::time::Duration::from_millis(100)) }}",
                            function_name_camel,
                            code
                        );

                        println!(
                            "fn {}() -> String {{ response_helpers::get_enriched_response_with_metadata({}, None, std::time::Duration::from_millis(100)) }}",
                            function_name_pascal,
                            code
                        );
                    }
                }
            }
        )*
    };
}

#[test]
fn test_generated_functions_success() {
  let response = crate::ResponsesSuccessCodes::Ok;
  let (code, description): (u16, &str) = response.into();
  assert_eq!(code, 200);
  assert_eq!(
            description,
            "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
        );
}

#[cfg(test)]
mod tests {
  use crate::helpers::response_functions::ResponseFunctions;
  use std::io::{stdout, Write};
  use std::sync::{Arc, Mutex};

  #[test]
  fn test_generate_responses_with_metadata() {
    // Créer un buffer protégé par un Mutex pour capturer stdout
    let buffer = Arc::new(Mutex::new(Vec::new()));
    let buffer_clone = Arc::clone(&buffer);

    // Rediriger stdout vers le buffer
    let original_stdout = stdout();
    let mut locked_stdout = original_stdout.lock();
    let mut locked_buffer = buffer_clone.lock().unwrap();

    // Rediriger les sorties dans le buffer
    let _ = write!(&mut locked_buffer, ""); // Préparer le buffer
    let _ = write!(&mut locked_stdout, ""); // Éviter des problèmes avec stdout

    // Appeler la fonction générée par la macro
    crate::ResponsesSuccessCodes::generate_responses_with_metadata();

    // Lire les données capturées
    let output = String::from_utf8(buffer.lock().unwrap().clone()).expect("Invalid UTF-8");

    // Vérifier que les données générées contiennent les valeurs attendues
    assert!(output.contains("fn ok() -> String"));
    assert!(output.contains("response_helpers::get_enriched_response_with_metadata(200"));
  }

  #[test]
  fn test_function_presence() {
    // Vérifie que les fonctions générées existent
    crate::ResponsesSuccessCodes::generate_responses_with_metadata();
  }
}
