use simbld_http::helpers::response_helpers;
use simbld_http::responses::*;
use strum::EnumProperty;
use strum::IntoEnumIterator;

fn main() {
  // Using standard HTTP codes
  let response = ResponsesInformationalCodes::Continue;
  println!("{} - {}", response as u16, response.get_str("Description").unwrap_or("No description"));

  // Generating a response with a function
  let response = ResponsesInformationalCodes::EarlyHints;
  let custom_response =
    response_helpers::transform_to_json(ResponsesTypes::Informational(response));
  println!("{}", custom_response);

  // Searching for a code by mapping
  if let Some(description) = response_helpers::get_description_by_code(100) {
    println!("Found description: {}", description);
  } else {
    println!("Code not found");
  }

  // Searching for a type by code
  if let Some(response) = response_helpers::get_response_by_code(200) {
    let (code, description) = response_helpers::get_response_description(response);
    println!("Code: {}, Description: {}", code, description);
  } else {
    println!("Code not found");
  }

  // Transforming response to JSON
  let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
  println!("JSON: {}", response_helpers::transform_to_json(response));

  // Additional examples
  let code_num: u16 = 100;
  match ResponsesInformationalCodes::try_from(code_num) {
    Ok(code_enum) => println!("Le code {} correspond à {:?}", code_num, code_enum),
    Err(_) => println!("Code {} inconnu", code_num),
  }

  // Conversion from enum to u16
  let code_enum = ResponsesInformationalCodes::Processing;
  let code_num: u16 = code_enum.into();
  println!("{:?} a le code {}", code_enum, code_num);

  // Accessing the description
  if let Some(description) = code_enum.get_str("Description") {
    println!("Description: {}", description);
  }

  // Iterating over variants
  for code in ResponsesInformationalCodes::iter() {
    let num_code: u16 = code.into();
    let description = code.get_str("Description").unwrap_or("Aucune description");
    println!("Code {}: {}", num_code, description);
  }
}
