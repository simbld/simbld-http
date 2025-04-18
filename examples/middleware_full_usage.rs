use actix_web::dev::ServiceRequest;
use actix_web::{web, App, HttpResponse, HttpServer};
use simbld_http::helpers::{
    http_interceptor_helper::HttpInterceptor, unified_middleware_helper::UnifiedMiddleware,
};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// # Demonstration of UnifiedMiddleware and HttpInterceptor
///
/// This example shows how to integrate the UnifiedMiddleware and HttpInterceptor
/// with an Actix Web application to implement request filtering and response modification.
///
/// ## Features Demonstrated
/// - Configuration of UnifiedMiddleware with custom request validation
/// - Conditional middleware application based on request path
/// - Rate limiting configuration (100 requests per minute)
/// - Integration with HttpInterceptor for additional request/response processing
///
/// ## Middleware Configuration
/// The example configures UnifiedMiddleware with:
/// - CORS origins restriction ("allowed_origins")
/// - Rate limiting (100 requests per 60 seconds)
/// - Request method filtering (only GET requests)
/// - Path-based exclusion (only requests starting with "/api")
///
/// ## Endpoints
/// - `GET /`: Returns a simple 200 OK response
/// - `GET /custom`: Returns a 400 Bad Request response for testing
///
/// ## Usage
/// Run the application with `cargo run` and access:
/// - `http://127.0.0.1:8086/` - Should return 200 OK
/// - `http://127.0.0.1:8086/custom` - Should return 400 Bad Request
/// - Any path starting with "/api" will bypass the middleware
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(UnifiedMiddleware::new(
                "allowed_origins".to_string(),
                Arc::new(Mutex::new(HashMap::<String, (u64, Instant)>::new())),
                100,
                Duration::from_secs(60),
                Rc::new(|req: &ServiceRequest| {
                    // Example of personalized condition: Check if the request is a GET request
                    req.method() == actix_web::http::Method::GET
                }),
                Some(Box::new(|req: &ServiceRequest| {
                    // Example of personalized condition: Check if the request path starts with "/api"
                    req.path().starts_with("/api")
                })),
            )) // Applies a custom middleware
            .wrap(HttpInterceptor) // Applies another custom middleware
            .route("/", web::get().to(home)) // Modify here to call the `home` function
            .route("/custom", web::get().to(custom_example))
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}

/// Handler for the root endpoint.
/// Returns a simple 200 OK response.
///
async fn home() -> impl actix_web::Responder {
    HttpResponse::Ok().finish()
}

/// Example handler that returns a 400 Bad Request.
/// Useful for testing error response handling.
///
async fn custom_example() -> impl actix_web::Responder {
    HttpResponse::BadRequest().finish()
}
