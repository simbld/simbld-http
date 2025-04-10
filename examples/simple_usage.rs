use actix_web::{web, App, HttpResponse, HttpServer};
use simbld_http::helpers::{
    http_interceptor_helper::HttpInterceptor, unified_middleware_helper::UnifiedMiddleware,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(UnifiedMiddleware::simple(
                vec!["allowed_origins".to_string()],
                100,
                std::time::Duration::from_secs(60),
            )) // Utilisation de la méthode simplifiée
            .wrap(HttpInterceptor)
            .route("/", web::get().to(home))
            .route("/custom", web::get().to(custom_example))
    })
    .bind("127.0.0.1:8086")?
    .run()
    .await
}

async fn home() -> impl actix_web::Responder {
    HttpResponse::Ok().finish()
}

async fn custom_example() -> impl actix_web::Responder {
    HttpResponse::BadRequest().finish()
}
