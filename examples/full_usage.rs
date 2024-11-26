use actix_web::{web, App, HttpServer};
use simbld_http::helpers::{
  http_interceptor_helper::HttpInterceptor, http_interceptor_helper2::HttpInterceptor2,
  response_middleware_helper::ResponseMiddleware, response_middleware_helper2::ResponseMiddleware2,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(ResponseMiddleware) // General-purpose middleware
      .wrap(HttpInterceptor) // Middleware for HTTP code descriptions
      .wrap(ResponseMiddleware2) // Another general-purpose middleware
      .wrap(HttpInterceptor2) // Another middleware for HTTP code descriptions
      .route("/", web::get().to(index))
      .route("/custom", web::get().to(custom_example))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

async fn index() -> impl actix_web::Responder {
  simbld_http::responses::success::ok() // Example response
}

async fn custom_example() -> impl actix_web::Responder {
  simbld_http::responses::client::bad_request() // Another example response
}
