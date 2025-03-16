use actix_web::{web, App, HttpServer, Responder};
use simbld_http::helpers::{to_u16_trait::ToU16, unified_middleware_helper::UnifiedMiddleware};
use simbld_http::responses::{CustomResponse, ResponsesSuccessCodes::Ok};

async fn example_response() -> impl Responder {
    CustomResponse::new(
        Ok.to_u16(),
        format!("{} {} {}", "Success", "Test data", "Request was successful"),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(UnifiedMiddleware::new(vec!["*".to_string()], 100, std::time::Duration::from_secs(60), /* Rc<(dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static)> */))
            .route("/", web::get().to(example_response))
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
