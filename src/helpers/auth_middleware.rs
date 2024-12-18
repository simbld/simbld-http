/// The code defines an Actix Web middleware for authentication based on token parameters in URL requests.
///
/// Properties:
///
/// * `key`: The code you provided is an implementation of an Actix Web middleware for authentication. It checks the token parameter in the URL query string and based on its value, it generates different HTTP responses for authentication success, expired token, invalid token, or missing token.
use actix_web::{
  body::{BoxBody, EitherBody, MessageBody},
  dev::{Service, ServiceRequest, ServiceResponse, Transform},
  http::StatusCode,
  web, Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use serde::Deserialize;
use std::task::{Context, Poll};

// Parameters for URL requests
#[derive(Deserialize)]
pub struct TokenParams {
  pub key: Option<String>,
}

pub struct AuthMiddleware;

pub struct AuthMiddlewareService<S> {
  service: S,
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: MessageBody + 'static,
{
  type Response = ServiceResponse<EitherBody<B, BoxBody>>;
  type Error = Error;
  type Transform = AuthMiddlewareService<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(AuthMiddlewareService {
      service,
    })
  }
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: MessageBody + 'static,
{
  type Response = ServiceResponse<EitherBody<B, BoxBody>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(&self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    Poll::Ready(Ok(()))
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    // Check the request parameters
    let query = web::Query::<TokenParams>::from_query(req.query_string()).ok();
    let token = query.and_then(|q| q.key.clone()); // Clone the token to avoid invalid references

    let response = match token.as_deref() {
      Some("validated") => {
        HttpResponse::build(StatusCode::from_u16(222).unwrap()).body("Authentication Successful")
      },
      Some("expired") => {
        HttpResponse::build(StatusCode::from_u16(419).unwrap()).body("Page Expired")
      },
      Some(_) => HttpResponse::build(StatusCode::from_u16(498).unwrap()).body("Invalid Token"),
      None => HttpResponse::build(StatusCode::from_u16(983).unwrap()).body("Missing Token"),
    };

    if response.status() == StatusCode::from_u16(222).unwrap() {
      let fut = self.service.call(req);
      return Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) });
    }

    Box::pin(async move { Ok(req.into_response(response.map_into_right_body())) })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::{test, App};

  #[actix_web::test]
  async fn test_auth_middleware() {
    let app = test::init_service(App::new().wrap(AuthMiddleware).route(
      "/protected",
      web::get().to(|| async {
        HttpResponse::build(StatusCode::from_u16(222).unwrap()).body("Access Granted")
      }),
    ))
    .await;

    // Test case 1: Valid token
    let req_valid = test::TestRequest::get().uri("/protected?key=validated").to_request();
    let resp_valid = test::call_service(&app, req_valid).await;
    assert_eq!(
      resp_valid.status(),
      StatusCode::from_u16(222).unwrap(),
      "Expected status code 222 for a validated token."
    );

    // Test case 2: Expired token
    let req_expired = test::TestRequest::get().uri("/protected?key=expired").to_request();
    let resp_expired = test::call_service(&app, req_expired).await;
    assert_eq!(
      resp_expired.status(),
      StatusCode::from_u16(419).unwrap(),
      "Expected status code 419 for an expired token."
    );

    // Test case 3: Invalid token
    let req_invalid = test::TestRequest::get().uri("/protected?key=invalid").to_request();
    let resp_invalid = test::call_service(&app, req_invalid).await;
    assert_eq!(
      resp_invalid.status(),
      StatusCode::from_u16(498).unwrap(),
      "Expected status code 498 for an invalid token."
    );

    // Test case 4: Missing token
    let req_missing = test::TestRequest::get().uri("/protected").to_request();
    let resp_missing = test::call_service(&app, req_missing).await;
    assert_eq!(
      resp_missing.status(),
      StatusCode::from_u16(983).unwrap(),
      "Expected status code 983 for a missing token."
    );
  }
}
