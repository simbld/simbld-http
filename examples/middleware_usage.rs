use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use simbld_http::ResponseMiddleware;

async fn index() -> impl Responder {
  HttpResponse::Ok().body("App is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().wrap(ResponseMiddleware).route("/", web::get().to(index)))
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
