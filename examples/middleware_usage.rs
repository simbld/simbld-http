use actix_web::{web, App, HttpServer};
use simbld_http::helpers::{
  http_interceptor_helper::HttpInterceptor, unified_middleware_helper::UnifiedMiddleware,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(UnifiedMiddleware)
      .wrap(HttpInterceptor)
      .route("/", web::get().to(example_response))
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}

async fn example_response() -> impl actix_web::Responder {
  simbld_http::responses::ResponsesServerCodes::internal_server_error()
}
