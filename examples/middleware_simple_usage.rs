use actix_web::{web, App, HttpResponse, HttpServer};
use simbld_http::helpers::{
    http_interceptor_helper::HttpInterceptor, unified_middleware_helper::UnifiedMiddleware,
};

/// # Simple Example of UnifiedMiddleware Integration
///
/// This example demonstrates the basic integration of UnifiedMiddleware and HttpInterceptor
/// with an Actix Web application using the simplified configuration approach.
///
/// ## Features Demonstrated
/// - Usage of UnifiedMiddleware::simple() for streamlined configuration
/// - Basic rate limiting configuration
/// - CORS origins restriction
/// - Integration with HttpInterceptor
///
/// ## Middleware Configuration
/// The example uses UnifiedMiddleware with simplified configuration:
/// - CORS origin restriction to "allowed_origins"
/// - Rate limiting of 100 requests per 60 seconds
/// - No custom request validation or path exclusion rules
///
/// ## Endpoints
/// - `GET /`: Returns a standard 200 OK response
/// - `GET /custom`: Returns a 400 Bad Request response
///
/// ## Usage
/// Run the application with `cargo run` and access:
/// - `http://127.0.0.1:8086/` - Should succeed with 200 OK
/// - `http://127.0.0.1:8086/custom` - Should return 400 Bad Request
///
/// This example shows how to quickly implement basic API protection
/// with minimal configuration.
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(UnifiedMiddleware::simple(
                vec!["allowed_origins".to_string()],
                100,
                std::time::Duration::from_secs(60),
            ))
            .wrap(HttpInterceptor)
            .route("/", web::get().to(home))
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
