use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::Error;
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};

pub struct HttpInterceptor;

impl<S, B> Transform<S, ServiceRequest> for HttpInterceptor
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = HttpInterceptorMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(HttpInterceptorMiddleware {
      service: Rc::new(service),
    })
  }
}

pub struct HttpInterceptorMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for HttpInterceptorMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(cx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let service = Rc::clone(&self.service);
    let fut = service.call(req);
    let start_time = std::time::Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();

    Box::pin(async move {
      let mut res = fut.await?;

      res.headers_mut().insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&request_id).unwrap(),
      );

      let duration = start_time.elapsed().as_millis();
      res.headers_mut().insert(
        HeaderName::from_static("x-response-time-ms"),
        HeaderValue::from_str(&duration.to_string()).unwrap(),
      );

      let status_code = res.status().as_u16();
      if let Some(description) =
        crate::helpers::response_helpers::get_description_by_code(status_code)
      {
        res.headers_mut().insert(
          HeaderName::from_static("x-status-description"),
          HeaderValue::from_str(description).unwrap(),
        );
      }

      Ok(res)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::{test, web, App, HttpResponse};

  #[actix_web::test]
  async fn test_http_interceptor() {
    let app = test::init_service(
      App::new()
        .wrap(HttpInterceptor)
        .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello World") })),
    )
    .await;

    let req = test::TestRequest::with_uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello World");
  }

  #[actix_web::test]
async fn test_http_interceptor_adds_header() {
    let app = test::init_service(
        App::new()
            .wrap(HttpInterceptor)
            .route("/", web::get().to(|| async { HttpResponse::Ok().finish() })),
    )
    .await;

    let req = test::TestRequest::with_uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    let header_value = resp
        .headers()
        .get("x-status-description")
        .expect("Header 'x-status-description' is missing")
        .to_str()
        .unwrap();
    assert_eq!(
        header_value,
        "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
    );
}
}
