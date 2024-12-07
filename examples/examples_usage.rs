// Importing modules and helpers from simbld-http
use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::response_helpers;
use simbld_http::helpers::response_with_cookie_helper::{bad_request_with_cookie, ok_with_cookie};
use simbld_http::helpers::response_with_headers_helper::{
  bad_request_with_headers, ok_with_headers,
};
use simbld_http::helpers::to_u16_helper::ToU16;
use simbld_http::helpers::{
  http_interceptor_helper::HttpInterceptor, unified_middleware_helper::UnifiedMiddleware,
};
use simbld_http::responses::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use strum::EnumProperty;

/// Example route for success
async fn example_success() -> impl Responder {
  response_helpers::create_response(
    ResponsesSuccessCodes::Ok.to_u16(),
    ResponsesSuccessCodes::Ok.get_str("Description").unwrap_or("No description"),
    "",
  )
}

/// Example route for client error
async fn example_client_error() -> impl Responder {
  response_helpers::create_response(
    ResponsesClientCodes::BadRequest.to_u16(),
    ResponsesClientCodes::BadRequest.get_str("Description").unwrap_or("No description"),
    "",
  )
}

/// Example route for server error
async fn example_server_error() -> impl Responder {
  response_helpers::create_response(
    ResponsesServerCodes::InternalServerError.to_u16(),
    ResponsesServerCodes::InternalServerError.get_str("Description").unwrap_or("No description"),
    "",
  )
}

/// Example of using helpers
fn examples_with_helpers() {
  println!("=== Examples with Helpers ===");

  // Example 1: Using standard HTTP codes
  let response = ResponsesInformationalCodes::Continue;
  println!("{} - {}", response as u16, response.get_str("Description").unwrap_or("No description"));

  // Example 2: Using ResponsesTypes for success codes
  let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
  println!(
    "Code: {}, Description: {}",
    response.to_u16(),
    response.get_str("Description").unwrap_or("No description")
  );

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
  examples_with_helpers();
  HttpServer::new(|| {
    App::new()
      .wrap(UnifiedMiddleware {
        allowed_origins: vec!["*".to_string()],
        rate_limiters: Arc::new(Mutex::new(HashMap::new())),
        max_requests: 100,
        window_duration: std::time::Duration::from_secs(60),
      })
      .wrap(HttpInterceptor) // Specific interceptor
      .route("/success", web::get().to(example_success))
      .route("/client_error", web::get().to(example_client_error))
      .route("/server_error", web::get().to(example_server_error))
  }) // General middleware
  .bind("127.0.0.1:8083")?
  .run()
  .await
}
