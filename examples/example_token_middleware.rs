//! # Protected Route with Authentication Middleware Example
//!
//! This example demonstrates how to use the `AuthMiddleware` from the `simbld_http` crate
//! to protect routes in an Actix Web application. The middleware validates request tokens
//! and controls access to protected resources.

use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer};
use simbld_http::AuthMiddleware;
use std::io;

/// Starts an HTTP server with a protected route that requires authentication.
///
/// # Authentication Flow
/// The server uses `AuthMiddleware` to validate access tokens provided as query parameters.
/// The middleware intercepts requests to all routes and checks the validity of the token.
///
/// # Protected Route
/// - Path: `/protected`
/// - Method: GET
/// - Access control: Requires a valid authentication token
/// - Success response: Returns "Access Granted" with HTTP 200 status
///
/// # Testing URLs
/// The application can be tested with these URLs:
/// - `http://127.0.0.1:8069/protected?key=validated` - Valid token (successful access)
/// - `http://127.0.0.1:8069/protected?key=expired` - Expired token (should be rejected)
/// - `http://127.0.0.1:8069/protected?key=mistake` - Invalid token (should be rejected)
/// - `http://127.0.0.1:8069/protected` - Missing token (should be rejected)
///
/// # Returns
/// Returns an `io::Result<()>` to propagate any server binding or execution errors.
///
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
