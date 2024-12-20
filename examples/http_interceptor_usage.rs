use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::http_interceptor_helper::HttpInterceptor;

async fn test_response() -> impl Responder {
  actix_web::HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(HttpInterceptor)
      .route("/", web::get().to(test_response))
      .route("/test", web::get().to(test_response))
  })
  .bind("127.0.0.1:8087")?
  .run()
  .await
}
