use actix_web::{web, App, HttpServer};
use simbld_http::responses::actix_responder::custom_response_handler;
use simbld_http::responses::CustomResponse;
use simbld_http::ResponsesSuccessCodes;

/// # CustomResponse Handler Example
///
/// This example demonstrates how to create and use a pre-configured CustomResponse
/// with the custom_response_handler utility in an Actix Web application.
///
/// ## Features Demonstrated
/// - Creating a CustomResponse instance with specific parameters
/// - Sharing a pre-configured response across different request handlers
/// - Using the custom_response_handler utility function from simbld_http
/// - Integrating CustomResponse with Actix Web's dependency injection
///
/// ## Configuration
/// The example creates a standard 200 OK response with:
/// - Status code from ResponsesSuccessCodes::Ok
/// - Title: "Success"
/// - Data: "Test data"
/// - Description: "Request was successful"
///
/// ## Endpoints
/// - `GET /`: Returns the pre-configured custom response
///
/// ## How It Works
/// 1. A CustomResponse is created at application startup
/// 2. The response is registered as application data using web::Data
/// 3. All requests to the root endpoint are handled by custom_response_handler
/// 4. The handler automatically uses the pre-configured response
///
/// ## Usage
/// Run the application with `cargo run` and access:
/// - `GET http://127.0.0.1:8088/` - Returns the pre-configured successful response
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let custom_response = CustomResponse::new(
        ResponsesSuccessCodes::Ok.get_code(),
        "Success",
        "Test data",
        "Request was successful",
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(custom_response.clone()))
            .route("/", web::get().to(custom_response_handler))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
