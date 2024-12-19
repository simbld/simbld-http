use actix_web::http::StatusCode; // Import for StatusCode
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use simbld_http::responses::client::{invalid_token, page_expired};
use simbld_http::responses::local::missing_token;
use simbld_http::responses::success::authentication_successful;
use simbld_http::AuthMiddleware;

/// Checks if a token is valid or not.
async fn token_middleware(query: web::Query<TokenParams>) -> impl Responder {
  let response = match query.key.as_deref() {
    Some("validated") => {
      let (code, description) = authentication_successful();
      HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(description)
    },
    Some("expired") => {
      let (code, description) = page_expired();
      HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(description)
    },
    Some(_) => {
      let (code, description) = invalid_token();
      HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(description)
    },
    None => {
      let (code, description) = missing_token();
      HttpResponse::build(StatusCode::from_u16(code).unwrap()).body(description)
    },
  };

  response
}

#[derive(serde::Deserialize)]
struct TokenParams {
  key: Option<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  println!("Starting HTTP server at http://127.0.0.1:8069");

  HttpServer::new(|| {
    App::new().wrap(AuthMiddleware).route("/token", web::get().to(token_middleware))
  })
  .bind("127.0.0.1:8069")?
  .run()
  .await
}
