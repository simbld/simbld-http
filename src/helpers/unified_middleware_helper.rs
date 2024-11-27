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
pub struct UnifiedMiddleware {
  allowed_origins: Vec<String>,
  rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
  max_requests: u64,
  window_duration: Duration,
}

impl UnifiedMiddleware {
  pub fn new(allowed_origins: Vec<String>, max_requests: u64, window_duration: Duration) -> Self {
    Self {
      allowed_origins,
      rate_limiters: Arc::new(Mutex::new(HashMap::new())),
      max_requests,
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
      max_requests: self.max_requests,
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
