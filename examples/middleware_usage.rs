use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::{to_u16_trait::ToU16, unified_middleware_helper::UnifiedMiddleware};
use simbld_http::responses::{CustomResponse, ResponsesSuccessCodes::Ok};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

async fn example_response() -> impl Responder {
    CustomResponse::new(Ok.to_u16(), "Success", "Test data", "Request was successful")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(UnifiedMiddleware {
                allowed_origins: vec!["*".to_string()],
                rate_limiters: Arc::new(Mutex::new(HashMap::new())),
                max_requests: 100,
                window_duration: std::time::Duration::from_secs(60),
            })
            .route("/", web::get().to(example_response))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
