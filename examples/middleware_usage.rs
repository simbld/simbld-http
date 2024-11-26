use actix_web::{web, App, HttpServer};
use simbld_http::helpers::{
  http_interceptor_helper::HttpInterceptor, http_interceptor_helper2::HttpInterceptor2,
  response_middleware_helper::ResponseMiddleware, response_middleware_helper2::ResponseMiddleware2,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(ResponseMiddleware) // Adds headers like X-Powered-By
      .wrap(ResponseMiddleware2)
      .wrap(HttpInterceptor) // Adds HTTP status descriptions
      .wrap(HttpInterceptor2)
      .route("/", web::get().to(example_response))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

async fn example_response() -> impl actix_web::Responder {
  simbld_http::responses::server::internal_server_error()
}
