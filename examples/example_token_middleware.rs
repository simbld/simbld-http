use actix_web::http::StatusCode;
/// The above Rust code sets up an HTTP server using Actix-web framework with a route protected by an authentication middleware.
///
/// Returns:
///
/// The code provided is setting up an Actix Web server that listens on `http://127.0.0.1:8069`. It defines a route `/protected` that is protected by the `AuthMiddleware` middleware. When a GET request is made to `/protected`, it responds with a message "You have a valid token -> Access granted".
///
/// http://127.0.0.1:8069/protected?key=validated
/// http://127.0.0.1:8069/protected?key=expired
/// http://127.0.0.1:8069/protected?key=mistake
/// http://127.0.0.1:8069/protected
///
use actix_web::{web, App, HttpResponse, HttpServer};
use simbld_http::AuthMiddleware;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
  println!("Starting HTTP server at http://127.0.0.1:8069");

  HttpServer::new(|| {
    App::new()
      .wrap(AuthMiddleware)
      // We protect this route via middleware
      .route(
        "/protected",
        web::get().to(|| async {
          HttpResponse::build(StatusCode::from_u16(200).unwrap())
            .insert_header(("X-HTTP-Status-Code", "200"))
            .body("Access Granted")
        }),
      )
  })
  .bind("127.0.0.1:8069")?
  .run()
  .await
}
