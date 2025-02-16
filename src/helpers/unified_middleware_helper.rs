use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::HttpResponse;
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use thiserror::Error;

/// The code defines a unified middleware in Rust for managing CORS, advanced logs, and rate limiting in Actix Web.
///
/// Properties:
///
/// * `allowed_origins`: The `allowed_origins` property is a list of strings representing the origins that are allowed to make requests to the server. This is used for Cross-Origin Resource Sharing (CORS) checks to ensure that requests are only accepted from specified origins.
/// * `rate_limiters`: The `rate_limiters` property in the `UnifiedMiddleware` struct is a shared state that stores information about the rate limiting for different clients based on their IP addresses. It is a `HashMap` that maps client IP addresses to a tuple containing the number of requests made by the client and the start
/// * `max_requests`: The `max_requests` property in the `UnifiedMiddleware` struct represents the maximum number of requests allowed within a specified time window for rate limiting purposes. It is used to limit the number of requests a client can make within a certain duration.
/// * `window_duration`: The `window_duration` property in the `UnifiedMiddleware` struct represents the duration of the time window for rate limiting. Requests from a client IP address are counted within this time window to enforce rate limits. It is used to determine the period over which the maximum number of requests allowed (`max_requests`)
/// * `intercept_dependencies`: The `intercept_dependencies` property in the `UnifiedMiddleware` struct is a function that takes a `ServiceRequest` as input and returns a boolean value. It is used to determine whether a request should be intercepted and processed by the middleware. The function can be used to define custom logic for intercepting requests based on specific criteria.

/// Unified middleware structure for managing CORS, advanced logging, and rate limiting.
pub struct UnifiedMiddleware {
    pub allowed_origins: Vec<String>,
    pub rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    pub max_requests: usize,
    pub window_duration: Duration,
    pub intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
}

/// Custom error type for middleware errors.
#[derive(Debug, Error)]
pub enum UnifiedError {
    // Middleware-specific errors
    #[error("An internal error occurred in the middleware.")]
    InternalMiddlewareError,
    #[error("Invalid request.")]
    InvalidRequest,
    #[error("Unauthorized access.")]
    Unauthorized,

    // IO and JSON-related errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Implementation of the `ResponseError` trait for UnifiedError.
impl actix_web::ResponseError for UnifiedError {
    /// Generates HTTP responses based on the error type.
    fn error_response(&self) -> HttpResponse {
        match self {
            UnifiedError::InternalMiddlewareError => HttpResponse::InternalServerError()
                .body("An internal error occurred in the middleware."),
            UnifiedError::InvalidRequest => HttpResponse::BadRequest().body("Invalid request."),
            UnifiedError::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized."),
            UnifiedError::Io(_) => {
                HttpResponse::InternalServerError().body("An I/O error occurred.")
            },
            UnifiedError::Json(_) => {
                HttpResponse::BadRequest().body("A JSON parsing error occurred.")
            },
        }
    }
}

/// Methods for the UnifiedMiddleware structure.
impl UnifiedMiddleware {
    /// Creates a new UnifiedMiddleware instance.
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

/// Custom implementation of the Debug trait for UnifiedMiddleware to hide complex details.
impl std::fmt::Debug for UnifiedMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnifiedMiddleware")
            .field("allowed_origins", &self.allowed_origins)
            .field("rate_limiters", &"Arc<Mutex<...>>") // Hide content for brevity
            .field("max_requests", &self.max_requests)
            .field("window_duration", &self.window_duration)
            .field("intercept_dependencies", &"<function>")
            .finish()
    }
}

/// Implementation of the Transform trait for UnifiedMiddleware.
impl<S, B> Transform<S, ServiceRequest> for UnifiedMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = UnifiedError> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = UnifiedError;
    type Transform = UnifiedMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    /// Creates a new middleware transformation for the given service.
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

/// Service wrapper integrating the logic of UnifiedMiddleware.
pub struct UnifiedMiddlewareService<S> {
    service: Rc<S>,
    allowed_origins: Vec<String>,
    rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    max_requests: usize,
    window_duration: Duration,
    intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
}

/// Implementation of the `Service` trait for UnifiedMiddlewareService.
impl<S, B> Service<ServiceRequest> for UnifiedMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = UnifiedError> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = UnifiedError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    /// Checks if the main service is ready to process requests.
    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    /// Processes a request using the middleware.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Logical request interception
        if !(self.intercept_dependencies)(&req) {
            return Box::pin(async { Err(UnifiedError::InternalMiddlewareError.into()) });
        }

        // Rate limiting logic
        let remote_addr = req
            .peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let now = Instant::now();
        let mut rate_limiters = self.rate_limiters.lock().unwrap();
        let entry = rate_limiters.entry(remote_addr).or_insert((0, now));

        // Reset time window if exceeded
        if now.duration_since(entry.1) > self.window_duration {
            *entry = (1, now);
        } else {
            entry.0 += 1;
        }

        if entry.0 > self.max_requests as u64 {
            return Box::pin(async { Err(UnifiedError::InvalidRequest.into()) });
        }

        // Delegate the request to the next service in the chain
        let service = self.service.clone();
        Box::pin(async move { service.call(req).await.map_err(UnifiedError::from) })
    }
}

#[cfg(test)]
/// Test module for UnifiedMiddleware.
mod tests {
  use super::*;
  use actix_web::{test, web, App, HttpResponse};
  
  #[actix_web::test]
    async fn test_rate_limiting() {
        // Middleware with a rate limit of 2 requests per 60 seconds
        let middleware = UnifiedMiddleware::new(
            vec!["https://allowed-origin.com".to_string()],
            2,
            Duration::from_secs(60),
            Rc::new(|_req| true),
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
            .insert_header(("Origin", "https://allowed-origin.com"))
            .to_request();
        let resp = test::call_service(&app, req1).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        // Second request
        let req2 = test::TestRequest::get()
            .uri("/test")
            .insert_header(("Origin", "https://allowed-origin.com"))
            .to_request();
        let resp = test::call_service(&app, req2).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        // Third request (exceeds the limit)
        let req3 = test::TestRequest::get()
            .uri("/test")
            .insert_header(("Origin", "https://allowed-origin.com"))
            .to_request();
        let resp = test::call_service(&app, req3).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::TOO_MANY_REQUESTS);
    }
}
