use actix_web::{web, App, HttpResponse, HttpServer};
use serde_json::json;
use simbld_http::helpers::response_helpers::{
  transform_to_json_with_metadata, transform_to_xml_with_metadata,
};
use simbld_http::responses::client::bad_request_tuple;
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
  let (code, name, desc) = bad_request_tuple();
  HttpResponse::BadRequest().json(json!({ "status": code, "name": name, "description": desc }))
}

/// Route for the 201 Created response.
async fn created_route() -> HttpResponse {
  let code = ResponsesSuccessCodes::Created;
  let (code, description) = code.into();
  HttpResponse::Created()
    .json(json!({ "status": code, "name": "Created", "description": description }))
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
      .route("/created", web::get().to(created_route))
      .route("/unauthorized", web::get().to(unauthorized_route))
  })
  .bind("127.0.0.1:8095")?
  .run()
  .await
}
