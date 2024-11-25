use actix_web::{middleware, App, HttpServer};
use simbld_http::helpers::response_middleware_helper::ResponseMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(ResponseMiddleware) // Add middleware
      .wrap(middleware::Logger::default()) // Optional logging middleware
      .service(actix_web::web::resource("/").to(|| async { "Hello, Simbld-HTTP!" }))
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
