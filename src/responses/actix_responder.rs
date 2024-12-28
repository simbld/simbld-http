/// The code defines a custom response struct and a handler function for Actix-Web that generates an HTTP response based on the custom response.
///
/// Arguments:
///
/// * `custom_response`: The `custom_response` parameter in the `custom_response_handler` function is of type `web::Data<CustomResponse>`. It represents a shared state containing an instance of the `CustomResponse` struct, which holds a response code of type `ResponsesTypes`. This parameter allows access to the `
/// * `req`: The `req` parameter in the `custom_response_handler` function is of type `HttpRequest`. It represents the incoming HTTP request that the handler function is processing. The `HttpRequest` type provides access to various details of the incoming request such as headers, method, URI, and other request-related information.
///
/// Returns:
///
/// The `custom_response_handler` function returns an `actix_web::HttpResponse`. This function takes a `web::Data<CustomResponse>` and an `HttpRequest` as input parameters, retrieves the `CustomResponse` from the `web::Data`, clones it, and then calls the `respond_to` method on the cloned `CustomResponse` to generate an `HttpResponse` based on the response code
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

#[derive(Clone)]
pub struct CustomResponse {
  pub code: u16,
  pub name: &'static str,
  pub description: &'static str,
}

impl CustomResponse {
  pub fn new(code: u16, name: &'static str, description: &'static str) -> Self {
    CustomResponse {
      code,
      name,
      description,
    }
  }
}

impl Responder for CustomResponse {
  type Body = actix_web::body::BoxBody;

  fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
    HttpResponse::build(actix_web::http::StatusCode::from_u16(self.code).unwrap())
      .json(json!({ "status": self.code, "name": self.name, "description": self.description }))
  }
}

// Handler compatible with Actix-Web
pub async fn custom_response_handler(
  custom_response: web::Data<CustomResponse>,
  req: HttpRequest,
) -> HttpResponse {
  let response = custom_response.get_ref().clone(); // Clone CustomResponse
  response.respond_to(&req)
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::{http::StatusCode, test, App};

  #[actix_web::test]
  async fn test_custom_response_responder() {
    // Step 1: Create a custom response
    let custom_response = CustomResponse {
      code: 200,
      name: "OK",
      description: "Success",
    };

    // Step 2: Initialize an Actix-Web application with a handler
    let app = test::init_service(
      App::new()
        .app_data(web::Data::new(custom_response))
        .default_service(web::route().to(custom_response_handler)),
    )
    .await;

    // Step 3: Simulate an HTTP request
    let req = test::TestRequest::default().to_request();
    let resp = test::call_service(&app, req).await;

    // Step 4: Assert the response
    assert_eq!(resp.status(), StatusCode::OK);
  }
}
