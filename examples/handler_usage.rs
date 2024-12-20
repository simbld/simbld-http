use actix_web::{web, App, HttpServer};
use simbld_http::responses::actix_responder::custom_response_handler;
use simbld_http::responses::{CustomResponse, ResponsesTypes};
use simbld_http::ResponsesSuccessCodes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let custom_response = CustomResponse {
    code: ResponsesTypes::Success(ResponsesSuccessCodes::Ok),
  };

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(custom_response.clone()))
      .route("/", web::get().to(custom_response_handler))
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
