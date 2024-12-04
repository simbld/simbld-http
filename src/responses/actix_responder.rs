use crate::responses::ResponsesServerCodes;
use actix_web::{HttpResponse, Responder};

pub struct CustomResponse {
  pub code: ResponsesServerCodes,
}

impl Responder for CustomResponse {
  type Body = actix_web::body::BoxBody;

  fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
    match self.code {
      ResponsesServerCodes::InternalServerError => {
        HttpResponse::InternalServerError().body("Internal Server Error")
      },

      _ => HttpResponse::InternalServerError().body("Unknown Error"),
    }
  }
}
