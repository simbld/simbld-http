use inflector::Inflector;
use strum::IntoEnumIterator;

pub struct ResponseWrapper<T>(pub T);

impl<T> ResponseWrapper<T>
where
  T: IntoEnumIterator + std::fmt::Debug + Copy + ToString + Into<u16>,
{
  pub fn generate_responses() {
    for variant in T::iter() {
      let function_name_snake = variant.to_string().to_snake_case();
      let function_name_camel = variant.to_string().to_camel_case();
      let code: u16 = variant.into();

      println!(
        "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}",
        function_name_snake, code, variant
      );

      println!(
        "fn {}() -> (u16, &'static str) {{ ({}, {:?}) }}",
        function_name_camel, code, variant
      );
    }
  }

  pub fn generate_responses_with_metadata() {
    for variant in T::iter() {
      let function_name_snake = variant.to_string().to_snake_case();
      let function_name_camel = variant.to_string().to_camel_case();
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
    }
  }
}
