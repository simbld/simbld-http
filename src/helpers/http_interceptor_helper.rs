use actix_service::{Service, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

pub struct HttpInterceptor;

impl<S, B> Transform<S, ServiceRequest> for HttpInterceptor
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = InterceptorMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    futures_util::future::ok(InterceptorMiddleware {
      service,
    })
  }
}

pub struct InterceptorMiddleware<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for InterceptorMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
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

    Box::pin(async move {
      let mut res = fut.await?;

      // Inject the HTTP status code description
      let status_code = res.status().as_u16();
      if let Some(description) =
        crate::helpers::response_helpers::get_description_by_code(status_code)
      {
        res.headers_mut().insert(
          HeaderName::from_static("x-status-description"),
          HeaderValue::from_str(description)
            .unwrap_or_else(|_| HeaderValue::from_static("Unknown")),
        );
      }

      // Inject the processing time
      let duration = start_time.elapsed();
      res.headers_mut().insert(
        HeaderName::from_static("x-response-time-ms"),
        HeaderValue::from_str(&format!("{}", duration.as_millis())).unwrap(),
      );

      Ok(res)
    })
  }
}
