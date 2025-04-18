use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::http_interceptor_helper::HttpInterceptor;
use simbld_http::responses::CustomResponse;
use simbld_http::{ResponsesClientCodes, ResponsesSuccessCodes};

async fn example_success() -> impl Responder {
    ResponsesSuccessCodes::Ok.into_response()
}

async fn example_bad_request() -> impl Responder {
    let (code, name, data, desc) = ResponsesClientCodes::BadRequest.get_all_data();
    CustomResponse::new(code, name, data, desc)
}

/// # HttpInterceptor and CustomResponse Example
///
/// This example demonstrates how to use HttpInterceptor middleware and CustomResponse
/// with Actix Web to create standardized API responses.
///
/// ## Features Demonstrated
/// - Integration of HttpInterceptor middleware in an Actix Web application
/// - Using predefined response codes from ResponsesSuccessCodes and ResponsesClientCodes
/// - Creating custom API responses with standardized format
/// - Converting response codes directly to responses
/// - Building custom responses with explicit parameters
///
/// ## Endpoints
/// - `GET /success`: Returns a standard 200 OK response
/// - `POST /bad_request`: Returns a 400 Bad Request response
///
/// ## Response Format
/// The responses follow a standardized format defined by CustomResponse:
/// - HTTP status code
/// - Response name/title
/// - Response data (typically JSON)
/// - Description message
///
/// ## Usage
/// Run the application with `cargo run` and access:
/// - `GET http://127.0.0.1:8087/success` - Returns a successful response
/// - `POST http://127.0.0.1:8087/bad_request` - Returns an error response
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(HttpInterceptor)
            .route("/success", web::get().to(example_success))
            .route("/bad_request", web::post().to(example_bad_request))
    })
    .bind("127.0.0.1:8087")?
    .run()
    .await
}
