/// The code defines a unified middleware in Rust for managing CORS, advanced logs, and rate limiting in Actix Web.
///
/// Properties:
///
/// * `allowed_origins`: The `allowed_origins` property is a list of strings representing the origins that are allowed to make requests to the server. This is used for Cross-Origin Resource Sharing (CORS) checks to ensure that requests are only accepted from specified origins.
/// * `rate_limiters`: The `rate_limiters` property in the `UnifiedMiddleware` struct is a shared state that stores information about the rate limiting for different clients based on their IP addresses. It is a `HashMap` that maps client IP addresses to a tuple containing the number of requests made by the client and the start
/// * `max_requests`: The `max_requests` property in the `UnifiedMiddleware` struct represents the maximum number of requests allowed within a specified time window for rate limiting purposes. It is used to limit the number of requests a client can make within a certain duration.
/// * `window_duration`: The `window_duration` property in the `UnifiedMiddleware` struct represents the duration of the time window for rate limiting. Requests from a client IP address are counted within this time window to enforce rate limits. It is used to determine the period over which the maximum number of requests allowed (`max_requests`)
use actix_service::{Service, Transform};
use actix_web::body::{BoxBody, EitherBody, MessageBody};
use actix_web::{
  dev::{ServiceRequest, ServiceResponse},
  http::header::*,
  Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use log::{debug, error, info, warn};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// Unified middleware managing CORS, advanced logs, and rate limiting.

#[derive(Debug)]
pub struct UnifiedMiddleware {
  pub allowed_origins: Vec<String>,
  pub rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
  pub max_requests: usize,
  pub window_duration: std::time::Duration,
}
impl UnifiedMiddleware {
  pub fn new(allowed_origins: Vec<String>, max_requests: u64, window_duration: Duration) -> Self {
    Self {
      allowed_origins,
      rate_limiters: Arc::new(Mutex::new(HashMap::new())),
      max_requests: max_requests.try_into().unwrap(),
      window_duration,
    }
  }
}

impl<S, B> Transform<S, ServiceRequest> for UnifiedMiddleware
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: MessageBody + 'static,
{
  type Response = ServiceResponse<EitherBody<B, BoxBody>>;
  type Error = Error;
  type Transform = UnifiedMiddlewareService<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(UnifiedMiddlewareService {
      service,
      allowed_origins: self.allowed_origins.clone(),
      rate_limiters: Arc::clone(&self.rate_limiters),
      max_requests: self.max_requests as u64,
      window_duration: self.window_duration,
    })
  }
}

pub struct UnifiedMiddlewareService<S> {
  service: S,
  allowed_origins: Vec<String>,
  rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
  max_requests: u64,
  window_duration: Duration,
}
impl<S, B> Service<ServiceRequest> for UnifiedMiddlewareService<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: MessageBody + 'static,
{
  type Response = ServiceResponse<EitherBody<B, BoxBody>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

  fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.service.poll_ready(ctx)
  }

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let origin = req.headers().get(ORIGIN).and_then(|h| h.to_str().ok()).unwrap_or("");
    let client_ip = req.connection_info().realip_remote_addr().unwrap_or("unknown").to_string();

    // CORS check
    if !self.allowed_origins.contains(&origin.to_string()) {
      warn!("Origin not allowed: {}", origin);
      return Box::pin(async move {
        let response = HttpResponse::Forbidden()
          .json(json!({ "error": "Origin not allowed" }))
          .map_into_right_body();
        Ok(req.into_response(response))
      });
    }

    // Rate limiting
    let now = Instant::now();

    {
      let mut rate_limiters = self.rate_limiters.lock().unwrap();
      let entry = rate_limiters.entry(client_ip.clone()).or_insert((0, now));

      let (ref mut request_count, ref mut window_start) = *entry;

      if now.duration_since(*window_start) > self.window_duration {
        *window_start = now;
        *request_count = 0;
      }

      if *request_count >= self.max_requests {
        warn!("Rate limit exceeded for client: {}", client_ip);
        return Box::pin(async move {
          let response = HttpResponse::TooManyRequests()
            .json(json!({ "error": "Too many requests" }))
            .map_into_right_body();
          Ok(req.into_response(response))
        });
      }

      *request_count += 1;
    }

    // Logs
    info!("Request received from {}: {} {}", client_ip, req.method(), req.uri());

    let fut = self.service.call(req);

    Box::pin(async move {
      let start_time = Instant::now();
      let res = fut.await;

      let elapsed_time = start_time.elapsed().as_millis();
      match res {
        Ok(response) => {
          debug!("Response sent to {}: {} ({} ms)", client_ip, response.status(), elapsed_time);
          // Convert the response body into `EitherBody::Left`
          Ok(response.map_into_left_body())
        },
        Err(e) => {
          error!("Error handling request from {}: {}", client_ip, e);
          // Pass the error upwards
          Err(e) // Pas besoin d'utiliser `e.into()` ici, car `e` est déjà du bon type.
        },
      }
    })
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
