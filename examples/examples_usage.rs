use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::{json, Value};
use simbld_http::helpers::{
    http_interceptor_helper::HttpInterceptor, response_helpers,
    response_with_cookie_helper::ok_with_cookie, response_with_headers_helper::ok_with_headers,
    to_u16_trait::ToU16, unified_middleware_helper::UnifiedMiddleware,
};
use simbld_http::responses::actix_responder::CustomResponse;
use simbld_http::responses::{ResponsesSuccessCodes, ResponsesTypes};
use simbld_http::ResponsesSuccessCodes::Ok;
use simbld_http::{ResponsesClientCodes, ResponsesCrawlerCodes, ResponsesServerCodes};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use strum::EnumProperty;

fn examples_with_helpers() {
    println!("=== Examples with Helpers ===");

    let response = ResponsesCrawlerCodes::ParsingErrorHeader;

    // Convert to HTTP code
    let http_code = response.to_http_code();
    println!("{:?}", http_code);

    // Get the standard code in u16
    let std_code = response.to_u16();
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

    // HACK: Extract data from the JSON
    let code = ok_value["status"].as_u64().unwrap_or(200) as u16;
    let name = ok_value["name"].as_str().unwrap_or("unknown");
    let desc = ok_value["description"].as_str().unwrap_or("No description");
    let ok_response = json!({ "status": code, "name": name, "description": desc });
    println!("Extracted from ok(): {}", ok_response);

    // Example 3: Using the bad_request() helper
    let bad_request = ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest);
    let http_code = bad_request.as_tuple();
    
    HttpResponse::BadRequest().json(json!({
        "status": http_code.standard_code,
        "name": http_code.standard_name,
        "description": http_code.unified_description
    });

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
    let unified_middleware = UnifiedMiddleware {
        allowed_origins: vec!["*".to_string()],
        rate_limiters: Arc::new(Mutex::new(HashMap::new())),
        max_requests: 100,
        window_duration: Duration::from_secs(60),
        intercept_dependencies: None,
    };
    println!("Created UnifiedMiddleware: {:?}", unified_middleware);

    // Example 7: Using helpers with http interceptor
    let http_interceptor = HttpInterceptor;
    println!("Created HttpInterceptor: {:?}", http_interceptor);

    // Example 8: Using helpers with custom responses
    let body_str = json!({
        "status": bad_request_code,
        "description": bad_request_desc
    })
    .to_string();
    let bad_request_desc = "Bad request example description";
    let custom_response = CustomResponse::new(bad_request_code, body_str);
    println!("CustomResponse from bad_request: {:?}", custom_response);

    // Example 9: Using helpers with success codes
    let success_code = ResponsesSuccessCodes::Ok;
    let success_code_u16 = success_code.to_u16();
    let success_code_str = success_code.get_str("Description").unwrap_or("No description");
    let success_code_json = json!({ "status": success_code_u16, "description": success_code_str });
    println!("{}", success_code_json);

    println!("=== End of Examples ===");
}

// Route to transform a response into JSON
///
/// @function transform_bad_request_to_json
/// @description Route: Retourne un JSON de type "BadRequest" via create_response.
///
async fn transform_bad_request_to_json() -> impl Responder {
    // XXX: We retrieve code, name, desc from the tuple
    let (code, _name, desc) = bad_request_tuple();
    // QUESTION: Do we want the `_name` included? We keep it minimal here.
    let data = r#"{"extraData":"someValue"}"#;
    let response_str = response_helpers::create_response(code, desc, data);

    let response_json: Value = match serde_json::from_str(&response_str) {
        Ok(v) => v,
        Err(_e) => json!({"error": "Failed to parse response"}),
    };
    HttpResponse::Ok().json(response_json)
}

// Route for ok with metadata
///
/// @function example_ok_with_metadata
/// @description Route: Affiche un enrichissement de métadonnées sur un code HTTP 200.
///
async fn example_ok_with_metadata() -> impl Responder {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let enriched_response = response_helpers::get_enriched_response_with_metadata(
        response,
        Some("https://example.com"),
        Duration::from_millis(200),
    );

    let enriched_value: Value = match serde_json::from_str(&enriched_response) {
        Ok(v) => v,
        Err(_e) => json!({"error": "Failed to parse metadata response"}),
    };
    HttpResponse::Ok().json(enriched_value)
}

// Route for success
async fn example_handler() -> HttpResponse {
    HttpResponse::Ok().content_type("application/json").json(json!({"message": "Hello, World!"}))
}

// Route for client error
async fn example_client_error() -> impl Responder {
    let (code, name, description) = ResponsesClientCodes::BadRequest.into();
    let body_str = json!({ "description": description }).to_string();
    CustomResponse::new(code, body_str)
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

    CustomResponse::new(code, error_str)
}

// Route to example JSON response
async fn example_json() -> impl Responder {
    let code = ResponsesSuccessCodes::Ok.to_u16();
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
    let http_code = bad_request.as_tuple(); // `http_code` est de type `HttpCode`.
    // QUESTION: Do we want to include the name in the response?
    let code = http_code.standard_code;
    let name = http_code.standard_name;
    let desc = http_code.unified_description;
    
    HttpResponse::BadRequest().json(json!({
        "status": code,
        "name": name,
        "description": desc,
    }))
}

/// The above function sets up an Actix web server with various routes and middleware for handling different types of requests.
///
/// The `UnifiedMiddleware` struct is used to define a middleware that can be applied to all routes in the Actix web server.
///
///  let wildcard = self.allowed_origins.contains(&"*".to_string());
///  if !(wildcard || self.allowed_origins.contains(&origin.to_string())) {
///    warn!("Origin not allowed: {}", origin);
///  }
///
/// The `main` function is returning a `std::io::Result<()>`, which indicates that it returns a result that may contain an `io::Error` if an I/O operation fails.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    examples_with_helpers();
    HttpServer::new(|| {
        App::new()
            .wrap(UnifiedMiddleware {
                allowed_origins: vec!["*".to_string()], // NOTE: ["*"] we authorize everything, otherwise we replace with our domain or localhost
                rate_limiters: Arc::new(Mutex::new(HashMap::new())),
                max_requests: 100,
                window_duration: std::time::Duration::from_secs(60),
                intercept_dependencies: None,
            })
            .wrap(HttpInterceptor) // Specific interceptor
            .route("/transform_bad_request_to_json", web::get().to(transform_bad_request_to_json))
            .route("/example_success", web::get().to(example_success))
            .route("/success", web::get().to(example_ok_with_metadata))
            .route("/client_error", web::get().to(example_client_error))
            .route("/server_error", web::get().to(example_server_error))
            .route("/example_json", web::get().to(example_json))
            .route("/ok_with_cookie", web::get().to(example_ok_with_cookie))
            .route("/ok_with_headers", web::get().to(example_ok_with_headers))
    }) // General middleware
    .bind("127.0.0.1:8083")?
    .run()
    .await
}
