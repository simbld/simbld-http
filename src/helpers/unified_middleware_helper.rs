use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::{Duration, Instant},
};

use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
};
use actix_web::{Error as ActixError, HttpResponse, ResponseError};
use futures_util::future::{ok, LocalBoxFuture, Ready};
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

// Middleware struct holding settings and state
pub struct UnifiedMiddleware {
    pub allowed_origins: Vec<String>,
    pub rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    pub max_requests: usize,
    pub window_duration: Duration,
    pub intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
}

// Enum for custom middleware errors
#[derive(Debug, Error)]
pub enum UnifiedError {
    #[error("An internal error occurred in the middleware.")]
    InternalMiddlewareError,
    #[error("Invalid request.")]
    InvalidRequest,
    #[error("Unauthorized access.")]
    Unauthorized,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

// Implementation of ResponseError, linking UnifiedError to standard HTTP responses
impl ResponseError for UnifiedError {
    fn status_code(&self) -> StatusCode {
        match self {
            UnifiedError::InternalMiddlewareError => StatusCode::INTERNAL_SERVER_ERROR,
            UnifiedError::InvalidRequest => StatusCode::BAD_REQUEST,
            UnifiedError::Unauthorized => StatusCode::UNAUTHORIZED,
            UnifiedError::Io(_) | UnifiedError::Json(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

// Middleware constructor
impl UnifiedMiddleware {
    pub fn new(
        allowed_origins: Vec<String>,
        max_requests: usize,
        window_duration: Duration,
        intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
    ) -> Self {
        UnifiedMiddleware {
            allowed_origins,
            rate_limiters: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration,
            intercept_dependencies,
        }
    }
}

// Middleware transformation implementation
impl<S, B> Transform<S, ServiceRequest> for UnifiedMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError; // UnifiedError now wrapped as ActixError
    type Transform = UnifiedMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(UnifiedMiddlewareService {
            service: Rc::new(service),
            allowed_origins: self.allowed_origins.clone(),
            rate_limiters: self.rate_limiters.clone(),
            max_requests: self.max_requests,
            window_duration: self.window_duration,
            intercept_dependencies: self.intercept_dependencies.clone(),
        })
    }
}

// Middleware service implementation
pub struct UnifiedMiddlewareService<S> {
    service: Rc<S>,
    allowed_origins: Vec<String>,
    rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    max_requests: usize,
    window_duration: Duration,
    intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
}

impl<S, B> Service<ServiceRequest> for UnifiedMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError; // UnifiedError wrapped as ActixError
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let origin = req.connection_info().host().to_owned();

        // Check if the origin is allowed
        if !self.allowed_origins.contains(&origin) {
            return Box::pin(async { Err(UnifiedError::Unauthorized.into()) });
        }

        // Intercept requests based on custom logic
        if !(self.intercept_dependencies)(&req) {
            return Box::pin(async { Err(UnifiedError::InvalidRequest.into()) });
        }

        // Rate limiting logic
        let remote_addr =
            req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|| "unknown".into());

        let now = Instant::now();
        let mut limiters = self.rate_limiters.lock().unwrap();
        let entry = limiters.entry(remote_addr).or_insert((0, now));

        if now.duration_since(entry.1) > self.window_duration {
            entry.0 = 1;
            entry.1 = now;
        } else {
            entry.0 += 1;
            if entry.0 > self.max_requests as u64 {
                return Box::pin(async { Err(UnifiedError::InvalidRequest.into()) });
            }
        }

        // Proceed to the next service in the pipeline
        let fut = self.service.call(req);
        Box::pin(async move { fut.await.map_err(|e| e.into()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    
    #[actix_web::test]
    async fn test_rate_limiting() {
        let middleware = UnifiedMiddleware::new(
            vec!["example.com".into()],
            2,
            Duration::from_secs(10),
            Rc::new(|req| req.path().starts_with("/api")),
        );

        let app = test::init_service(
            App::new()
                .wrap(middleware)
                .route("/", web::get().to(|| async { HttpResponse::Ok().body("OK") })),
        )
        .await;

        // First request should pass
        let request =
            test::TestRequest::with_uri("/").insert_header(("Host", "example.com")).to_request(); // New request
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Second request should pass within the limit
        let request =
            test::TestRequest::with_uri("/").insert_header(("Host", "example.com")).to_request(); // New request
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        // Third request should be blocked due to rate limiting
        let request =
            test::TestRequest::with_uri("/").insert_header(("Host", "example.com")).to_request(); // New request
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
