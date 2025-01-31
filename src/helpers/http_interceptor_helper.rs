/// Intercepts HTTP requests and responses to add custom headers.
///
/// Purpose: Intercept requests and responses to add cross-functional functionality such as logging.
/// How it works: It intercepts every request and response, allowing headers to be added or information to be logged.
/// Special feature: It uses Rc to share the service between the different middleware instances.
///
/// This struct implements the `Service` trait, allowing it to intercept
/// HTTP requests and responses. It adds the following headers to the response:
/// - `x-request-id`: A unique identifier for the request.
/// - `x-response-time-ms`: The time taken to process the request in milliseconds.
/// - `x-status-description`: A description of the status code, if available.
///
/// # Types
/// - `Response`: The type of the response, which is `ServiceResponse<B>`.
/// - `Error`: The type of the error, which is `Error`.
/// - `Future`: The type of the future, which is `LocalBoxFuture<'static, Result<Self::Response, Self::Error>>`.
///
/// # Methods
/// - `poll_ready`: Checks if the service is ready to accept a request.
/// - `call`: Intercepts the request, processes it, and adds custom headers to the response.
///
/// # Arguments
/// - `cx`: The context for the poll_ready method.
/// - `req`: The service request to be intercepted.
///
/// # Returns
/// - `poll_ready`: A `Poll` indicating if the service is ready.
/// - `call`: A future that resolves to the intercepted response with custom headers.
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::Error;
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};


#[derive(Debug)]
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
    ok(HttpInterceptorMiddleware { service: Rc::new(service) })
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
        crate::helpers::response_helpers::get_description_by_code(status_code).as_ref()
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
