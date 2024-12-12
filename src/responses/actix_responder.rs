use crate::responses::ResponsesTypes;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

#[derive(Clone)]
pub struct CustomResponse {
  pub code: ResponsesTypes,
}

impl Responder for CustomResponse {
  type Body = actix_web::body::BoxBody;

  fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
    let (code, description) = self.code.get_response_description();
    HttpResponse::build(actix_web::http::StatusCode::from_u16(code).unwrap()).body(description)
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
  use crate::responses::ResponsesSuccessCodes;
  use actix_web::{http::StatusCode, test, App};

  #[actix_web::test]
  async fn test_custom_response_responder() {
    // Step 1: Create a custom response
    let custom_response = CustomResponse {
      code: ResponsesTypes::Success(ResponsesSuccessCodes::Ok),
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

    // Step 4: Call the service with the request
    let resp = test::call_service(&app, req).await;

    // Step 5: Check the response
    assert_eq!(resp.status(), StatusCode::OK);
  }
}
