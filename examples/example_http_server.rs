//! # Standardized HTTP Responses Example
//!
//! This example demonstrates how to create standardized HTTP responses using the `simbld_http` library.
//! It includes examples of both success and error responses in different formats (JSON, XML)
//! and with custom metadata.

use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use simbld_http::helpers::response_helpers::{
    transform_to_json_with_metadata, transform_to_xml_with_metadata,
};
use simbld_http::helpers::unified_tuple_helper::UnifiedTuple;
use simbld_http::responses::ResponsesTypes;
use simbld_http::responses::{ResponsesClientCodes, ResponsesSuccessCodes};

/// Creates a 200 OK response with standardized data in XML format.
async fn ok_route() -> HttpResponse {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let xml_str = transform_to_xml_with_metadata(response);
    HttpResponse::Ok().body(xml_str)
}

/// Creates a 400 Bad Request response with standardized error data.
async fn bad_request_route() -> HttpResponse {
    let bad_request =
        UnifiedTuple::from(ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest));

    HttpResponse::BadRequest().json(json!({
        "status": bad_request.standard_code,
        "name": bad_request.standard_name,
        "description": bad_request.unified_description,
    }))
}

/// Creates a 201 Created response with custom message and standardized data.
async fn custom_created_route() -> HttpResponse {
    let response_code = ResponsesSuccessCodes::Created;
    let (code, description) = response_code.into();
    HttpResponse::Created()
        .json(json!({ "status": code, "name": "It's created !!!", "description": description }))
}

/// Creates a 401 Unauthorized response with standardized error data in JSON format.
async fn unauthorized_route() -> HttpResponse {
    let response = ResponsesClientCodes::Unauthorized;
    let json_str = transform_to_json_with_metadata(ResponsesTypes::ClientError(response));
    HttpResponse::Unauthorized().json(json_str)
}

/// Starts an HTTP server with routes demonstrating different response types.
///
/// The server provides the following endpoints:
/// - GET `/ok`: Returns a 200 OK response in XML format
/// - GET `/bad_request`: Returns a 400 Bad Request response
/// - GET `/created`: Returns a 201 Created response with custom message
/// - GET `/unauthorized`: Returns a 401 Unauthorized response
///
/// # Example URLs
/// ```
/// http://127.0.0.1:8095/ok
/// http://127.0.0.1:8095/bad_request
/// http://127.0.0.1:8095/created
/// http://127.0.0.1:8095/unauthorized
/// ```
///
/// # Returns
/// A Result that contains an IO error if the server fails to bind or run.
///
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting HTTP server at http://127.0.0.1:8095");

    HttpServer::new(|| {
        App::new()
            .route("/ok", web::get().to(ok_route))
            .route("/bad_request", web::get().to(bad_request_route))
            .route("/created", web::get().to(custom_created_route))
            .route("/unauthorized", web::get().to(unauthorized_route))
    })
    .bind("127.0.0.1:8095")?
    .run()
    .await
}
