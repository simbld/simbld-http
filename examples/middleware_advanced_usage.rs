use actix_web::dev::ServiceRequest;
use actix_web::HttpRequest;
use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::http_interceptor_helper::HttpInterceptor;
use simbld_http::helpers::unified_middleware_helper::UnifiedMiddleware;
use simbld_http::responses::CustomResponse;
use simbld_http::ResponsesSuccessCodes;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Example of an answer that processes data
async fn example_response(path: web::Path<String>) -> impl Responder {
    let user_id = path.into_inner();
    CustomResponse::new(
        ResponsesSuccessCodes::Ok.get_code(),
        "Success",
        serde_json::json!({
            "userId": user_id,
            "status": "actif",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })
        .to_string(),
        "User data retrieved successfully",
    )
}

// Example of simulated author verification
async fn auth_check(req: HttpRequest) -> impl Responder {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if auth_header == "Bearer valid-token" {
            return CustomResponse::new(
                ResponsesSuccessCodes::Ok.get_code(),
                "Authenticated",
                serde_json::json!({"authenticated": true}).to_string(),
                "User is authenticated",
            );
        }

        return CustomResponse::new(
            401,
            "Unauthorized",
            serde_json::json!({"error": "Invalid token"}).to_string(),
            "User is not authenticated",
        );
    }

    CustomResponse::new(
        401,
        "Unauthorized",
        serde_json::json!({"error": "Missing token"}).to_string(),
        "User is not authenticated",
    )
}
async fn health_check() -> impl Responder {
    CustomResponse::new(
        ResponsesSuccessCodes::Ok.get_code(),
        "Healthy",
        serde_json::json!({"status": "healthy", "version": "1.0.0"}).to_string(),
        "Service is healthy",
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configuration for API routes
    let create_api_middleware = || {
        UnifiedMiddleware::new(
            "https://app.example.com,https://admin.example.com".to_string(),
            Arc::new(Mutex::new(HashMap::<String, (u64, Instant)>::new())),
            200,
            Duration::from_secs(120),
            Rc::new(|req: &ServiceRequest| {
                return if let Some(content_type) = req.headers().get("Content-Type") {
                    let content_type = content_type.to_str().unwrap_or("");
                    content_type.contains("application/json")
                        || content_type.contains("application/x-www-form-urlencoded")
                } else {
                    req.method() == actix_web::http::Method::GET
                };
            }),
            Some(Box::new(|req: &ServiceRequest| req.path().contains("/auth"))),
        )
    };

    // Configuration for public routes
    let create_public_middleware =
        || UnifiedMiddleware::simple(vec!["*".to_string()], 50, Duration::from_secs(30));

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api")
                    .wrap(create_api_middleware())
                    .route("/users/{user_id}", web::get().to(example_response))
                    .route("/auth", web::post().to(auth_check)),
            )
            .service(
                web::scope("/public")
                    .wrap(create_public_middleware())
                    .route("/health", web::get().to(health_check)),
            )
            .wrap(HttpInterceptor)
    })
    .bind("127.0.0.1:8090")?
    .workers(4) // Specify a number of workers
    .run()
    .await
}
