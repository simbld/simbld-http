use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use simbld_http::responses::client::{bad_request, unauthorized};
use simbld_http::responses::success::{created, ok};

/// Route for the 200 OK response.
async fn ok_route() -> impl Responder {
  let (code, description) = ok(); // Appel de la fonction générée dynamiquement
  HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap()).body(description)
}

/// Route for the 400 Bad Request response.
async fn bad_request_route() -> impl Responder {
  let (code, description) = bad_request();
  HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap()).body(description)
}

/// Route for the 201 Created response.
async fn created_route() -> impl Responder {
  let (code, description) = created();
  HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap()).body(description)
}

/// Route for the 401 Unauthorized response.
async fn unauthorized_route() -> impl Responder {
  let (code, description) = unauthorized();
  HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap()).body(description)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Starting HTTP server at http://127.0.0.1:8095");

  HttpServer::new(|| {
    App::new()
      .route("/ok", web::get().to(ok_route))
      .route("/bad_request", web::get().to(bad_request_route))
      .route("/created", web::get().to(created_route))
      .route("/unauthorized", web::get().to(unauthorized_route))
  })
  .bind("127.0.0.1:8095")?
  .run()
  .await
}
