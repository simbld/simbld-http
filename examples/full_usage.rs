use actix_web::{web, App, HttpServer};
use simbld_http::helpers::{
  http_interceptor_helper::HttpInterceptor, unified_middleware_helper::UnifiedMiddleware,
};
use simbld_http::responses::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(UnifiedMiddleware) // Applies a custom middleware
      .wrap(HttpInterceptor) // Applies another custom middleware
      .route("/", web::get().to(home)) // Modify here to call the `home` function
      .route("/custom", web::get().to(custom_example))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

// Renamed `index` to `home`
async fn home() -> impl actix_web::Responder {
  ok()
}

async fn custom_example() -> impl actix_web::Responder {
  bad_request()
}

// Renamed to avoid conflict
async fn detailed_index() -> impl actix_web::Responder {
  let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
  let duration = std::time::Duration::from_millis(120);
  let cors_origin = Some("http://example.com");

  simbld_http::helpers::response_helpers::get_enriched_response_with_metadata(
    response,
    cors_origin,
    duration,
  )
}
