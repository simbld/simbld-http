use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::{json, Value};
use simbld_http::helpers::{
    http_interceptor_helper::HttpInterceptor, response_helpers,
    response_with_cookie_helper::ok_with_cookie, response_with_headers_helper::ok_with_headers,
    unified_middleware_helper::UnifiedMiddleware,
};
use simbld_http::responses::actix_responder::CustomResponse;
use simbld_http::responses::{ResponsesSuccessCodes, ResponsesTypes};
use simbld_http::ResponsesSuccessCodes::Ok;
use simbld_http::{ResponsesClientCodes, ResponsesCrawlerCodes, ResponsesServerCodes};
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use std::result::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn examples_with_helpers() {
    println!("=== Examples with Helpers ===");

    let response = ResponsesCrawlerCodes::ParsingErrorHeader;

    // Convert to HTTP code
    let http_code = response.to_http_code();
    println!("{:?}", http_code);

    // Get the standard code in u16
    let std_code = response.get_code();
    println!("Standard code: {}", std_code);

    // Get the internal code from from_u16
    let internal_code = ResponsesCrawlerCodes::from_u16(700);
    println!("Internal code: {:?}", internal_code);

    // Convert to tuple
    let tuple = response.as_tuple();
    println!("{:?}", tuple);

    // Convert to JSON
    let json = response.as_json();
    println!("{}", json);

    // Get the standard code and description
    let (bad_request_code, bad_request_desc) = response.into();
    println!("Bad request code: {}, description: {}", bad_request_code, bad_request_desc);

    // Example 1: Using helpers to transform responses into JSON
    let bad_request = ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest);
    let bad_request_json = response_helpers::transform_to_json(bad_request);

    println!("{}", bad_request_json);

    // Example 2: Using the ok() helper (returns a JSON Value)
    let ok_value: ResponsesSuccessCodes = Ok;
    println!("ok() returns: {:?}", ok_value);

    // Extract data from the enum
    let code = ok_value.get_code();
    let name = ok_value.get_name();
    let desc = ok_value.get_description();
    let ok_response = json!({ "status": code, "name": name, "description": desc });
    println!("Extracted from ok(): {}", ok_response);

    // Example 3: Using the bad_request() helper
    let bad_request = ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest);
    let http_code = bad_request.as_tuple();

    HttpResponse::BadRequest().json(json!({
        "status": http_code.standard_code,
        "name": http_code.standard_name,
        "description": http_code.unified_description
    }));

    // Example 4: Using helpers with cookies
    let ok_cookie = ("session_id", "abc123"); // Key-value for the cookie
    let ok_response_cookies = ok_with_cookie(ok_cookie);
    println!("{}", ok_response_cookies);

    // Example 5: Using helpers with headers
    let mut headers = HashMap::new();
    headers.insert("x-trace-id", "123456");
    headers.insert("x-correlation-id", "abc-def");

    let ok_response_headers = ok_with_headers(headers.clone());

    println!("{}", ok_response_headers);

    // Example 6: Using helpers with unified middleware
    let unified_middleware = UnifiedMiddleware::new(
        "*".to_string(),                      // Allowed origins
        Arc::new(Mutex::new(HashMap::new())), // Rate limiters
        100,                                  // Max requests
        Duration::from_secs(60),              // Window duration
        Rc::new(|_req| true),                 // Intercept dependencies
        None,
    );
    println!("Created UnifiedMiddleware: {:?}", unified_middleware);

    // Example 7: Using helpers with http interceptor
    let http_interceptor = HttpInterceptor;
    println!("Created HttpInterceptor: {:?}", http_interceptor);

    // Example 8: Using helpers with custom responses
    let bad_request_code = 400;
    let bad_request_name = "BadRequest";
    let bad_request_desc = "Bad request example description";
    let body_str = json!({
        "status": bad_request_code,
        "description": bad_request_desc
    })
    .to_string();

    let custom_response =
        CustomResponse::new(bad_request_code, bad_request_name, body_str, bad_request_desc);
    println!("CustomResponse from bad_request: {:?}", custom_response);

    // Example 9: Using helpers with success codes
    let success_code = Ok;
    let success_code_u16 = success_code.get_code();
    let success_code_str = success_code.get_description();
    let success_code_json = json!({ "status": success_code_u16, "description": success_code_str });
    println!("{}", success_code_json);

    println!("=== End of Examples ===");
}

// Route to transform a response into JSON
///
/// @function transform_bad_request_to_json
/// @description Route: Returns a "BadRequest" type JSON via create_response.
///
async fn transform_bad_request_to_json() -> impl Responder {
    // XXX: We retrieve code, name, desc from the tuple
    let bad_request = ResponsesClientCodes::BadRequest;
    let code = bad_request.get_code();
    let desc = bad_request.get_description();
    let data = r#"{"extraData":"someValue"}"#;
    let response_str = response_helpers::create_response(code, desc, data);
    let response_json: Value = match serde_json::from_str::<Value>(&response_str) {
        Result::Ok(v) => v,
        Err(_e) => json!({"error": "Failed to parse response"}),
    };

    HttpResponse::Ok().json(response_json)
}

// Route for ok with metadata
///
/// @function example_ok_with_metadata
/// @description Route: Displays metadata enrichment on HTTP code 200.
///
async fn example_ok_with_metadata() -> impl Responder {
    let response = ResponsesTypes::Success(Ok);
    let enriched_response = response_helpers::get_enriched_response_with_metadata(
        response,
        Some("https://example.com"),
        Duration::from_millis(200),
    );

    let enriched_value: Value = match serde_json::from_str(&enriched_response) {
        Result::Ok(v) => v,
        Err(_e) => json!({"error": "Failed to parse metadata response"}),
    };
    HttpResponse::Ok().json(enriched_value)
}

// Route for success with metadata on actix web
async fn example_success() -> HttpResponse {
    HttpResponse::Ok().content_type("application/json").json(json!({"message": "Hello, World!"}))
}

// Route for client error with 2 parameters fot CustomResponse
async fn example_client_error() -> impl Responder {
    let (code, description) = ResponsesClientCodes::BadRequest.into();
    let body_str = json!({ "description": description }).to_string();
    CustomResponse::new(code, body_str, "application/json", "")
}

// Route for Ok with cookies
async fn example_ok_with_cookie() -> impl Responder {
    let ok_cookie = ("session_id", "abc123");
    let cookie_json_str = ok_with_cookie(ok_cookie);
    let parsed_json: Value = serde_json::from_str(&cookie_json_str)
        .unwrap_or_else(|_| json!({"warning": "Could not parse the cookie response"}));
    HttpResponse::Ok().json(parsed_json)
}

// Route for helper with headers
async fn example_ok_with_headers() -> impl Responder {
    let mut headers = HashMap::new();
    headers.insert("Content-Type", "application/json");
    let ok_headers_str = ok_with_headers(headers);
    let parsed_json: Value = serde_json::from_str(&ok_headers_str)
        .unwrap_or_else(|_| json!({"warning": "Could not parse the headers response"}));
    HttpResponse::Ok().json(parsed_json)
}

// Route for server error with custom headers
async fn example_server_error() -> impl Responder {
    let server_error_value = ResponsesServerCodes::InternalServerError.as_json();
    let code = server_error_value["status"].as_u64().unwrap_or(500) as u16;
    let description = server_error_value["description"].as_str().unwrap_or("Unknown server error");
    let error_str = json!({
        "status": code,
        "description": description,
        "error_id": "123456"
    })
    .to_string();

    CustomResponse::new(code, error_str, "application/json", "")
}

// Route to example JSON response
async fn example_json() -> impl Responder {
    let code = Ok.get_code();
    let name = "Ok";
    let description = "A successful response (manually created)";
    let response = json!({
        "status": code,
        "name": name,
        "description": description
    });
    HttpResponse::Ok().json(response)
}

/// Route for the 400 Bad Request response.
async fn bad_request_route() -> HttpResponse {
    let bad_request = ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest);
    let http_code = bad_request.as_tuple();
    let code = http_code.standard_code;
    let name = http_code.standard_name;
    let desc = http_code.unified_description;

    HttpResponse::BadRequest().json(json!({
        "status": code,
        "name": name,
        "description": desc,
    }))
}

/// # Actix Web Server with UnifiedMiddleware
///
/// Sets up an Actix web server with various routes and middleware for handling
/// different types of HTTP requests with standardized responses.
///
/// ## Middleware Configuration
/// The server uses UnifiedMiddleware to provide:
/// - CORS protection with allowed origin verification
/// - Rate limiting for API protection
/// - Standardized error responses
///
/// ## CORS Implementation
/// Origin verification is performed with the following logic:
/// ```
/// let wildcard = allowed_origins.contains(&"*".to_string());
/// if !(wildcard || allowed_origins.contains(&origin.to_string())) {
///   warn!("Origin not allowed: {}", origin);
///   // Request is rejected or logged based on configuration
/// }
/// ```
///
/// ## Server Configuration
/// The server is configured to:
/// - Listen on a specified address and port
/// - Apply middleware to all routes
/// - Provide standardized responses for all endpoints
///
/// ## Error Handling
/// Returns an `std::io::Result<()>` to properly handle any I/O errors during server startup or execution.
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    examples_with_helpers();
    HttpServer::new(|| {
        App::new()
            .wrap(UnifiedMiddleware {
                allowed_origins: {
                    let mut set = HashSet::new();
                    set.insert("*".to_string());
                    set
                },
                rate_limiters: Arc::new(Mutex::new(HashMap::new())),
                max_requests: 100,
                window_duration: Duration::from_secs(60),
                intercept_dependencies: Rc::new(|_req| true),
                condition: Rc::new(Box::new(|_req| true)),
            })
            .wrap(HttpInterceptor) // Specific interceptor
            .route("/transform_bad_request_to_json", web::get().to(transform_bad_request_to_json))
            .route("/example_success", web::get().to(example_success))
            .route("/success", web::get().to(example_ok_with_metadata))
            .route("/client_error", web::get().to(example_client_error))
            .route("/ok_with_cookie", web::get().to(example_ok_with_cookie))
            .route("/ok_with_headers", web::get().to(example_ok_with_headers))
            .route("/server_error", web::get().to(example_server_error))
            .route("/example_json", web::get().to(example_json))
            .route("/bad_request", web::get().to(bad_request_route))
    }) // General middleware
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
