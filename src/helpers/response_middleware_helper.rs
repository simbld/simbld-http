use actix_service::{Service, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};

pub struct ResponseMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ResponseMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
  S::Future: 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = Error;
  type Transform = ResponseMiddlewareService<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(ResponseMiddlewareService {
      service,
    })
  }
}

pub struct ResponseMiddlewareService<S> {
  service: S,
}

impl<S, B> Service<ServiceRequest> for ResponseMiddlewareService<S>
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

      // Get the status code
      let status_code = res.status().as_u16();

      // Add the description to the headers
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
