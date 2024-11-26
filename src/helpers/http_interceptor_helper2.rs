use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures_util::future::{ok, ready, LocalBoxFuture, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct HttpInterceptor2;

impl<S, B> Transform<S, ServiceRequest> for HttpInterceptor2
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
    ok(InterceptorMiddleware {
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
    Box::pin(async move {
      let mut res = fut.await?;

      // Add HTTP description header
      if let Some(description) =
        crate::helpers::response_helpers::get_description_by_code(res.status().as_u16())
      {
        res.headers_mut().insert(
          actix_web::http::header::HeaderName::from_static("x-status-description"),
          actix_web::http::header::HeaderValue::from_str(description).unwrap(),
        );
      }

      Ok(res)
    })
  }
}
