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
