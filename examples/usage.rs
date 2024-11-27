// Importing modules and helpers from simbld-http
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use simbld_http::helpers::response_with_cookie_helper::bad_request_with_cookie;
use simbld_http::helpers::response_with_cookie_helper::ok_with_cookie;
use simbld_http::helpers::response_with_headers_helper::bad_request_with_headers;
use simbld_http::helpers::response_with_headers_helper::ok_with_headers;
use simbld_http::responses::*;
use std::collections::HashMap;
use strum::EnumProperty;

/// Example of success
pub fn ok() -> actix_web::HttpResponse {
  HttpResponse::Ok().finish()
}

/// Example of client error
pub fn bad_request() -> actix_web::HttpResponse {
  HttpResponse::BadRequest().finish()
}

/// Example of server error
pub fn internal_server_error() -> actix_web::HttpResponse {
  HttpResponse::InternalServerError().finish()
}

/// Example of using helpers
fn examples_with_helpers() {
  println!("=== Examples with Helpers ===");

  // Example 1: Using standard HTTP codes
  let response = ResponsesInformationalCodes::Continue;
  println!("{} - {}", response as u16, response.get_str("Description").unwrap_or("No description"));

  // Example 2: Using ResponsesTypes for success codes
  let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
  println!("Code: {}, Description: {}", response.to_u16(), response.get_str("Description"));

  // Example 3: Transforming a response to JSON
  let response = ResponsesInformationalCodes::EarlyHints;
  let custom_response =
    response_helpers::transform_to_json(ResponsesTypes::Informational(response));
  println!("{}", custom_response);

  // Example 4: Searching for description by code
  if let Some(description) = response_helpers::get_description_by_code(100) {
    println!("Found description: {}", description);
  } else {
    println!("Code not found");
  }

  // Example 5: Searching for a response type by code
  if let Some(response) = response_helpers::get_response_by_code(200) {
    let (code, description) = response_helpers::get_response_description(response);
    println!("Code: {}, Description: {}", code, description);
  } else {
    println!("Code not found");
  }

  // Example 6: Using helpers with cookies
  let cookie = ("session_id", "abc123"); // Key-value for the cookie
  let ok_response = ok_with_cookie(cookie);
  println!("{}", ok_response);

  let error_cookie = ("error_id", "xyz789");
  let error_response = bad_request_with_cookie(error_cookie);
  println!("{}", error_response);

  // Example 7: Using helpers with headers
  let mut headers = HashMap::new();
  headers.insert("x-trace-id", "123456");
  headers.insert("x-correlation-id", "abc-def");

  let ok_response = ok_with_headers(headers.clone());
  println!("{}", ok_response);

  let error_response = bad_request_with_headers(headers);
  println!("{}", error_response);

  println!("=== End of Examples ===");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // Launch an Actix-Web server with middlewares and routes
  HttpServer::new(|| {
    App::new()
      .wrap(UnifiedMiddleware) // General middleware
      .wrap(UnifiedInterceptor) // Specific interceptor
      .route("/success", web::get().to(example_success))
      .route("/client_error", web::get().to(example_client_error))
      .route("/server_error", web::get().to(example_server_error))
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await?;

  // Call static examples
  examples_with_helpers();

  Ok(())
}
