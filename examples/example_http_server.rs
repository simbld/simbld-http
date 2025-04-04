use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use simbld_http::helpers::response_helpers::{
    transform_to_json_with_metadata, transform_to_xml_with_metadata,
};
use simbld_http::helpers::unified_tuple_helper::UnifiedTuple;

use simbld_http::responses::ResponsesTypes;
use simbld_http::responses::{ResponsesClientCodes, ResponsesSuccessCodes};

/// Route for the 200 OK XML response with metadata.
async fn ok_route() -> HttpResponse {
    let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
    let xml_str = transform_to_xml_with_metadata(response);
    HttpResponse::Ok().body(xml_str)
}

/// Route for the 400 Bad Request response.
async fn bad_request_route() -> HttpResponse {
    let bad_request =
        UnifiedTuple::from(ResponsesTypes::ClientError(ResponsesClientCodes::BadRequest));

    HttpResponse::BadRequest().json(json!({
        "status": bad_request.standard_code,
        "name": bad_request.standard_name,
        "description": bad_request.unified_description,
    }))
}

/// Route for the 201 Created response.
async fn custom_created_route() -> HttpResponse {
    let response_code = ResponsesSuccessCodes::Created;
    let (code, description) = response_code.into();
    HttpResponse::Created()
        .json(json!({ "status": code, "name": "It's created !!!", "description": description }))
}

/// Route for the 401 Unauthorized response JSON with metadata.
async fn unauthorized_route() -> HttpResponse {
    let response = ResponsesClientCodes::Unauthorized;
    let json_str = transform_to_json_with_metadata(ResponsesTypes::ClientError(response));
    HttpResponse::Unauthorized().json(json_str)
}

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
