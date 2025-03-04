use actix_web::{web, App, HttpServer};
use simbld_http::helpers::to_u16_trait::ToU16;
use simbld_http::responses::actix_responder::custom_response_handler;
use simbld_http::responses::CustomResponse;
use simbld_http::ResponsesSuccessCodes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let custom_response = CustomResponse::new(
        ResponsesSuccessCodes::Ok.to_u16(),
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
