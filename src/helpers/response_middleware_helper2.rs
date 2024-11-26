use actix_service::{Service, Transform};
use actix_web::{
  dev::{ServiceRequest, ServiceResponse},
  Error,
};
use futures_util::future::{ok, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct ResponseMiddleware2;

impl<S, B> Transform<S, ServiceRequest> for ResponseMiddleware2
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = MiddlewareService<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(MiddlewareService {
      service,
    })
  }
}

pub struct MiddlewareService<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for MiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Future = Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

  fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(ctx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let fut = self.service.call(req);
    Box::pin(async move {
      let mut res = fut.await?;
      res.headers_mut().insert(
        actix_web::http::header::HeaderName::from_static("x-powered-by"),
        actix_web::http::header::HeaderValue::from_static("Simbld-HTTP"),
      );
      Ok(res)
    })
  }
}
