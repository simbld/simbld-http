use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error as ActixError, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use thiserror::Error;

/// Middleware for Unified handling of requests
#[derive(Debug)]
pub struct UnifiedMiddleware {
    pub allowed_origins: Vec<String>,
    pub rate_limiters: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<String, (u64, std::time::Instant)>>,
    >,
    pub max_requests: usize,
    pub window_duration: std::time::Duration,
    pub intercept_dependencies: std::rc::Rc<dyn Fn(&ServiceRequest) -> bool>,
    #[allow(dead_code)]
    condition: Box<dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static>,
}

#[derive(Debug, Error)]
pub enum UnifiedError {
    #[error("An internal error occurred in the middleware.")]
    InternalMiddlewareError,
    #[error("Invalid request.")]
    InvalidRequest,
    #[error("Unauthorized access.")]
    Unauthorized,
}

impl actix_web::ResponseError for UnifiedError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            UnifiedError::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            UnifiedError::InvalidRequest => actix_web::http::StatusCode::TOO_MANY_REQUESTS,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_message = format!("{}", self);
        HttpResponse::build(self.status_code()).body(error_message)
    }
}

impl UnifiedMiddleware {
    pub fn new(
        allowed_origins: Vec<String>,
        max_requests: usize,
        window_duration: std::time::Duration,
        intercept_dependencies: Vec<String>,
    ) -> Self {
        UnifiedMiddleware {
            allowed_origins,
            rate_limiters: std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::HashMap::new(),
            )),
            max_requests,
            window_duration,
            intercept_dependencies,
            condition: Box::new(|_req| true),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for UnifiedMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Transform = UnifiedMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(UnifiedMiddlewareService {
            service: std::rc::Rc::new(service),
            allowed_origins: self.allowed_origins.clone(),
            rate_limiters: std::sync::Arc::clone(&self.rate_limiters),
            max_requests: self.max_requests,
            window_duration: self.window_duration,
        })
    }
}

pub struct UnifiedMiddlewareService<S> {
    service: std::rc::Rc<S>,
    allowed_origins: Vec<String>,
    rate_limiters: std::sync::Arc<
        std::sync::Mutex<std::collections::HashMap<String, (u64, std::time::Instant)>>,
    >,
    max_requests: usize,
    window_duration: std::time::Duration,
}

impl<S, B> Service<ServiceRequest> for UnifiedMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let allowed_origins = self.allowed_origins.clone();
        let rate_limiters = std::sync::Arc::clone(&self.rate_limiters);
        let max_requests = self.max_requests;
        let window_duration = self.window_duration;
        let service = std::rc::Rc::clone(&self.service);

        Box::pin(async move {
            // Check if the origin is allowed
            if let Some(host) = req.headers().get("Host").and_then(|h| h.to_str().ok()) {
                if !allowed_origins.contains(&host.to_string()) {
                    return Err(ActixError::from(UnifiedError::Unauthorized));
                }
            } else {
                return Err(ActixError::from(UnifiedError::Unauthorized));
            }

            // Rate Limiting Logic
            let client_ip = req
                .peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            let mut limiters = rate_limiters.lock().unwrap();
            let current_time = std::time::Instant::now();

            let (requests, timestamp) =
                limiters.entry(client_ip.clone()).or_insert_with(|| (0, current_time));

            if current_time.duration_since(*timestamp) > window_duration {
                println!("Resetting rate limiter for client: {}", client_ip);
                *requests = 0;
                *timestamp = current_time;
            }

            if *requests >= max_requests as u64 {
                return Err(ActixError::from(UnifiedError::InvalidRequest));
            }

            *requests += 1;

            // Pass through middleware
            service.call(req).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::{App, HttpResponse};

    #[actix_web::test]
    async fn test_rate_limiting() {
        // Initialize middleware with only 2 requests allowed per 10 seconds
        let middleware = UnifiedMiddleware::new(
            vec!["localhost".to_string()],
            2,                                  // Max requests
            std::time::Duration::from_secs(10), // Time window
            std::rc::Rc::new(|_req| true),      // Always intercept
        );

        // Create Actix app with middleware applied
        let app = test::init_service(
            App::new()
                .wrap(middleware)
                .route("/", actix_web::web::get().to(|| async { HttpResponse::Ok().finish() })),
        )
        .await;

        // First request: should pass
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "localhost")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        // Second request: should pass
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "localhost")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        // Third request: should be rejected due to rate limiting
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "localhost")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::TOO_MANY_REQUESTS);
    }

    #[actix_web::test]
    async fn test_allowed_origins() {
        // Initialize middleware with "localhost" as the only allowed origin
        let middleware = UnifiedMiddleware::new(
            vec!["localhost".to_string()],
            10, // High request limit for testing (will not hit limit here)
            std::time::Duration::from_secs(10),
            std::rc::Rc::new(|_req| true), // Always intercept
        );

        // Create Actix app with middleware applied
        let app = test::init_service(
            App::new()
                .wrap(middleware)
                .route("/", actix_web::web::get().to(|| async { HttpResponse::Ok().finish() })),
        )
        .await;

        // Request from an allowed origin (localhost): should pass
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "localhost")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        // Request from a disallowed origin (example.com): should fail
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "example.com")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_reset_rate_limiting_window() {
        // Initialize middleware with a very short time window (2 seconds)
        let middleware = UnifiedMiddleware::new(
            vec!["localhost".to_string()],
            1,                                 // Only 1 request allowed per window
            std::time::Duration::from_secs(2), // Very short window to test reset logic
            std::rc::Rc::new(|_req| true),     // Always intercept
        );

        // Create Actix app with middleware applied
        let app = test::init_service(
            App::new()
                .wrap(middleware)
                .route("/", actix_web::web::get().to(|| async { HttpResponse::Ok().finish() })),
        )
        .await;

        // First request: should pass
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "localhost")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        // Second request within same window: should fail
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "localhost")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::TOO_MANY_REQUESTS);

        // Wait for the window duration to pass
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        // Third request after window resets: should pass
        let req =
            test::TestRequest::with_uri("/").insert_header(("Host", "localhost")).to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
    }
}
