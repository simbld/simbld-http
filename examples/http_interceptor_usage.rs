use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::http_interceptor_helper::HttpInterceptor;
use simbld_http::responses::client::bad_request;
use simbld_http::responses::success::already_reported_tuple;
use simbld_http::responses::CustomResponse;

async fn example_already_reported() -> impl Responder {
  let (code, name, desc) = already_reported_tuple();
  CustomResponse::new(code, name, "", desc)
}

async fn example_bad_request() -> impl Responder {
  let json_value = bad_request();
  let code = json_value["status"].as_u64().unwrap_or(400) as u16;
  let name = json_value["name"].as_str().unwrap_or("Bad Request").to_string();
  let desc = json_value["description"].as_str().unwrap_or("").to_string();

  CustomResponse::new(code, name, "", desc)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .wrap(HttpInterceptor)
      .route("/already_reported", web::get().to(example_already_reported))
      .route("/bad_request", web::post().to(example_bad_request))
  })
  .bind("127.0.0.1:8087")?
  .run()
  .await
}
