use actix_service::{Service, Transform};
use actix_web::{
  dev::{ServiceRequest, ServiceResponse},
  http::header::{HeaderName, HeaderValue},
  Error,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};
use std::time::Instant;
use uuid::Uuid;

pub struct HttpInterceptor;

impl<S, B> Transform<S, ServiceRequest> for HttpInterceptor
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = HttpInterceptorService<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(HttpInterceptorService {
      service,
    })
  }
}

pub struct HttpInterceptorService<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for HttpInterceptorService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(ctx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let fut = self.service.call(req);
    let start_time = Instant::now();
    let request_id = Uuid::new_v4().to_string();

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
