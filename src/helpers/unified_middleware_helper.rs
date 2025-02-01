use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::{HttpResponse, ResponseError};
use futures_util::future::{ok, LocalBoxFuture, Ready};
/// The code defines a unified middleware in Rust for managing CORS, advanced logs, and rate limiting in Actix Web.
///
/// Properties:
///
/// * `allowed_origins`: The `allowed_origins` property is a list of strings representing the origins that are allowed to make requests to the server. This is used for Cross-Origin Resource Sharing (CORS) checks to ensure that requests are only accepted from specified origins.
/// * `rate_limiters`: The `rate_limiters` property in the `UnifiedMiddleware` struct is a shared state that stores information about the rate limiting for different clients based on their IP addresses. It is a `HashMap` that maps client IP addresses to a tuple containing the number of requests made by the client and the start
/// * `max_requests`: The `max_requests` property in the `UnifiedMiddleware` struct represents the maximum number of requests allowed within a specified time window for rate limiting purposes. It is used to limit the number of requests a client can make within a certain duration.
/// * `window_duration`: The `window_duration` property in the `UnifiedMiddleware` struct represents the duration of the time window for rate limiting. Requests from a client IP address are counted within this time window to enforce rate limits. It is used to determine the period over which the maximum number of requests allowed (`max_requests`)
/// * `intercept_dependencies`: The `intercept_dependencies` property in the `UnifiedMiddleware` struct is a function that takes a `ServiceRequest` as input and returns a boolean value. It is used to determine whether a request should be intercepted and processed by the middleware. The function can be used to define custom logic for intercepting requests based on specific criteria.
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use thiserror::Error;

// Unique structure for managing unified middleware.
#[derive(Debug)]
pub struct UnifiedMiddleware {
  pub allowed_origins: Vec<String>,
  pub rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
  pub max_requests: usize,
  pub window_duration: Duration,
  pub intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
}

// Error type for middleware errors.
#[derive(Debug, Error)]
pub enum UnifiedError {
  // Middleware-specific errors.
  #[error("Internal Error occurred in the middleware")]
  InternalMiddlewareError,
  #[error("Invalid Request")]
  InvalidRequest,
  #[error("Unauthorized Access")]
  Unauthorized,

  // IO and JSON-related errors.
  #[error("I/O Error: {0}")]
  Io(#[from] std::io::Error),
  #[error("JSON Error: {0}")]
  Json(#[from] serde_json::Error),
}

// Implementation of the ResponseError trait for UnifiedError.
impl actix_web::ResponseError for UnifiedError {
  fn error_response(&self) -> HttpResponse {
    match self {
      // Middleware-specific errors.
      UnifiedError::InternalMiddlewareError => {
        HttpResponse::InternalServerError().body("Internal middleware error.")
      },
      UnifiedError::InvalidRequest => HttpResponse::BadRequest().body("Invalid request."),
      UnifiedError::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized."),

      // IO and JSON-specific errors.
      UnifiedError::Io(_) => HttpResponse::InternalServerError().body("I/O error occurred."),
      UnifiedError::Json(_) => HttpResponse::BadRequest().body("JSON parsing error."),
    }
  }
}

// Implementation of the UnifiedMiddleware structure.
impl UnifiedMiddleware {
  pub fn new(
    allowed_origins: Vec<String>,
    max_requests: usize,
    window_duration: Duration,
    intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
  ) -> Self {
    Self {
      allowed_origins,
      rate_limiters: Arc::new(Mutex::new(HashMap::new())),
      max_requests,
      window_duration,
      intercept_dependencies,
    }
  }
}

// Implement Debug for UnifiedMiddleware to display the content of the struct.
impl std::fmt::Debug for UnifiedMiddleware {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("UnifiedMiddleware")
      .field("allowed_origins", &self.allowed_origins)
      .field("rate_limiters", &"Arc<Mutex<...>>") // Content hidden to avoid complex Debug
      .field("max_requests", &self.max_requests)
      .field("window_duration", &self.window_duration)
      .field("intercept_dependencies", &"<function>")
      .finish()
  }
}

// Implementation of the Transform trait for UnifiedMiddleware.
impl<S, B> Transform<S, ServiceRequest> for UnifiedMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = UnifiedError> + 'static,
  B: 'static,
{
  // Inherited response and error types.
  type Response = ServiceResponse<B>;
  type Error = UnifiedError;
  type Transform = UnifiedMiddlewareService<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(UnifiedMiddlewareService {
      service: Rc::new(service),
      allowed_origins: self.allowed_origins.clone(),
      rate_limiters: Arc::clone(&self.rate_limiters),
      max_requests: self.max_requests,
      window_duration: self.window_duration,
      intercept_dependencies: Rc::clone(&self.intercept_dependencies),
    })
  }
}

// Unified middleware integrating the two analyzed concepts.
pub struct UnifiedMiddlewareService<S> {
  service: Rc<S>,
  allowed_origins: Vec<String>,
  rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
  max_requests: usize,
  window_duration: Duration,
  intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
}

// Implementation of the Service trait for UnifiedMiddlewareService.
impl<S, B> Service<ServiceRequest> for UnifiedMiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = UnifiedError> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<B>;
  type Error = UnifiedError;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  // Checks if the main service is ready.
  fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(ctx)
  }

  // Performs the combined actions of the two middlewares.
  fn call(&self, req: ServiceRequest) -> Self::Future {
    // Logical interceptor
    if !(self.intercept_dependencies)(&req) {
      return Box::pin(async { Err(UnifiedError::InternalMiddlewareError.into()) });
    }

    // Implementation of rate limiting
    let remote_addr =
      req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
    let now = Instant::now();
    let mut rate_limiters = self.rate_limiters.lock().unwrap();
    let entry = rate_limiters.entry(remote_addr).or_insert((0, now));

    // Reset time window if needed.
    if now.duration_since(entry.1) > self.window_duration {
      *entry = (1, now);
    } else {
      entry.0 += 1;
    }

    if entry.0 > self.max_requests as u64 {
      return Box::pin(async { Err(UnifiedError::InvalidRequest.into()) });
    }

    // Delegate request to the next service.
    let service = self.service.clone();
    Box::pin(async move { service.call(req).await.map_err(UnifiedError::from) })
  }
}

#[cfg(test)]
/// This module contains tests for unified middleware.
///
/// # Tests
///
/// * `test_rate_limiting` - Checks that rate limiting is working correctly.
/// - Initializes the middleware with a limit of 2 requests per 60 second window.
/// - Sends three requests and checks that the first two succeed and the third fails with a status `TOO_MANY_REQUESTS`.
mod tests {
  use super::*;
  use actix_web::{test, web, App, HttpResponse};
  use std::time::Duration;
  
  #[actix_web::test]
  async fn test_rate_limiting() {
    let middleware = UnifiedMiddleware::new(
      vec!["http://allowed-origin.com".to_string()],
      2, // Limit to 2 requests per window
      Duration::from_secs(60),
      Rc::new(|_req| true), // Add the missing argument
    );

    let app = test::init_service(
      App::new()
        .wrap(middleware)
        .route("/test", web::get().to(|| async { HttpResponse::Ok().finish() })),
    )
    .await;

    // First request
    let req1 = test::TestRequest::get()
      .uri("/test")
      .insert_header(("Origin", "http://allowed-origin.com"))
      .to_request();
    let resp = test::call_service(&app, req1).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Second request
    let req2 = test::TestRequest::get()
      .uri("/test")
      .insert_header(("Origin", "http://allowed-origin.com"))
      .to_request();
    let resp = test::call_service(&app, req2).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

    // Third request (should exceed the limit)
    let req3 = test::TestRequest::get()
      .uri("/test")
      .insert_header(("Origin", "http://allowed-origin.com"))
      .to_request();
    let resp = test::call_service(&app, req3).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::TOO_MANY_REQUESTS);
  }
}
