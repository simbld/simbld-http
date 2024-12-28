use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::unified_middleware_helper::UnifiedMiddleware;
use simbld_http::responses::{CustomResponse, ResponsesServerCodes};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use strum::EnumProperty;

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

async fn example_response() -> impl Responder {
  CustomResponse {
    code: ResponsesServerCodes::InternalServerError.into(),
    name: ResponsesServerCodes::InternalServerError.get_str("Name").unwrap_or("No name"),
    description: ResponsesServerCodes::InternalServerError
      .get_str("Description")
      .unwrap_or("No description"),
  }
}
