use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    error::Error as ActixError,
    http::{header, StatusCode},
    HttpResponse,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use thiserror::Error;

pub type ConditionFunction = Rc<Box<dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static>>;
pub type InterceptFunction = Rc<dyn Fn(&ServiceRequest) -> bool>;
pub type RateLimiters = Arc<Mutex<HashMap<String, (u64, Instant)>>>;
pub type AllowedOrigins = HashSet<String>;

impl std::fmt::Debug for UnifiedMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnifiedMiddleware")
            .field("allowed_origins", &self.allowed_origins)
            .field("max_requests", &self.max_requests)
            .field("window_duration", &self.window_duration)
            .finish()
    }
}

pub struct UnifiedMiddleware {
    pub allowed_origins: AllowedOrigins,
    pub rate_limiters: RateLimiters,
    pub max_requests: usize,
    pub window_duration: Duration,
    pub intercept_dependencies: InterceptFunction,
    pub condition: ConditionFunction,
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
    fn status_code(&self) -> StatusCode {
        match self {
            UnifiedError::InternalMiddlewareError => StatusCode::INTERNAL_SERVER_ERROR,
            UnifiedError::InvalidRequest => StatusCode::BAD_REQUEST,
            UnifiedError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(format!("{{\"error\": \"{}\"}}", self))
    }
}

pub type OptionalConditionFunction =
    Option<Box<dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static>>;

impl UnifiedMiddleware {
    /// Creates a new middleware with complete and flexible configuration.
    ///
    /// # arguments
    ///
    /// * `Allowed_origins' - Authorized Kid Origins, separated by commas (ex:" http: //example.com,http: // Localhost: 3000 ")
    /// * `rate_limiters` - Storage of rate limiters by IP
    /// * `Max_requests' - Maximum number of requests authorized in the time window
    /// * `Window_Duration` - Duration of the window for the rate limiter
    /// * `intercept_dependencies' - function that determines if the request must be intercepted
    /// * `Condition ' - Additional condition to apply the middleware
    ///
    pub fn new(
        allowed_origins: String,
        rate_limiters: RateLimiters,
        max_requests: usize,
        window_duration: Duration,
        intercept_dependencies: InterceptFunction,
        condition: OptionalConditionFunction,
    ) -> Self {
        let origins: AllowedOrigins =
            allowed_origins.split(',').map(|s| s.trim().to_string()).collect();

        let default_condition: Box<dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static> =
            Box::new(|_| true);

        Self {
            allowed_origins: origins,
            rate_limiters,
            max_requests,
            window_duration,
            intercept_dependencies,
            condition: Rc::new(condition.unwrap_or(default_condition)),
        }
    }

    /// Simplified version for current use cases.
    ///
    /// This function automatically initializes data and functions structures
    /// necessary with reasonable default values.
    ///
    /// # arguments
    ///
    /// * `Allowed_origins` - List of authorized Cors
    /// * `Max_requests' - Maximum number of requests authorized in the time window
    /// * `Window_Duration` - Duration of the window for the rate limiter
    ///
    pub fn simple(
        allowed_origins: Vec<String>,
        max_requests: usize,
        window_duration: Duration,
    ) -> Self {
        Self::new(
            allowed_origins.join(","),
            Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration,
            Rc::new(|_| true),
            Some(Box::new(|_| true)),
        )
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
        ready(Ok(UnifiedMiddlewareService {
            service: Rc::new(service),
            allowed_origins: self.allowed_origins.clone(),
            rate_limiters: self.rate_limiters.clone(),
            max_requests: self.max_requests,
            window_duration: self.window_duration,
            intercept_dependencies: self.intercept_dependencies.clone(),
            condition: self.condition.clone(),
        }))
    }
}

pub struct UnifiedMiddlewareService<S> {
    service: Rc<S>,
    allowed_origins: AllowedOrigins,
    rate_limiters: RateLimiters,
    max_requests: usize,
    window_duration: Duration,
    intercept_dependencies: InterceptFunction,
    condition: ConditionFunction,
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
        let service = self.service.clone();
        let condition = self.condition.clone();
        let intercept = self.intercept_dependencies.clone();
        let allowed_origins = self.allowed_origins.clone();
        let rate_limiters = self.rate_limiters.clone();
        let max_requests = self.max_requests;
        let window_duration = self.window_duration;

        Box::pin(async move {
            // Check if the conditions are met to apply the middleware
            if !(*condition)(&req) {
                return service.call(req).await;
            }

            if (*intercept)(&req) {
                // Check the origin if it is a CORS request
                check_origin(&req, &allowed_origins)?;

                // Check the rate limiting
                check_rate_limit(&req, rate_limiters, max_requests, window_duration)?;
            }

            service.call(req).await
        })
    }
}

// Function to check the origin of the request
fn check_origin(req: &ServiceRequest, allowed_origins: &AllowedOrigins) -> Result<(), ActixError> {
    if let Some(origin) = req.headers().get(header::ORIGIN) {
        if let Ok(origin_str) = origin.to_str() {
            if !allowed_origins.contains(origin_str) && !allowed_origins.contains("*") {
                return Err(UnifiedError::Unauthorized.into());
            }
        }
    }
    Ok(())
}

// Function to update the rate limiter for a specific client IP
fn check_rate_limit(
    req: &ServiceRequest,
    rate_limiters: RateLimiters,
    max_requests: usize,
    window_duration: Duration,
) -> Result<(), ActixError> {
    let client_ip = match req.connection_info().realip_remote_addr() {
        Some(ip) => ip.to_string(),
        None => "unknown".to_string(),
    };

    let should_limit =
        update_rate_limiter(&client_ip, rate_limiters, max_requests, window_duration)?;

    if should_limit {
        return Err(ActixError::from(UnifiedError::InvalidRequest));
    }

    Ok(())
}

// Function to update the rate limiter for a specific client IP
fn update_rate_limiter(
    client_ip: &str,
    rate_limiters: RateLimiters,
    max_requests: usize,
    window_duration: Duration,
) -> Result<bool, ActixError> {
    let mut limiters = rate_limiters.lock().map_err(|_| UnifiedError::InternalMiddlewareError)?;

    let now = Instant::now();
    let entry = limiters.entry(client_ip.to_string()).or_insert_with(|| (0, now));

    if now.duration_since(entry.1) > window_duration {
        // Reinitialize the entry if the window has expired
        *entry = (1, now);
        Ok(false)
    } else {
        // Increment the request count
        entry.0 += 1;
        Ok(entry.0 > max_requests as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        dev::Service,
        http::StatusCode,
        test::{init_service, TestRequest},
        web, App, HttpResponse,
    };
    use std::{sync::Arc, time::Duration};

    async fn test_handler() -> HttpResponse {
        HttpResponse::Ok().body("test success")
    }

    #[actix_web::test]
    async fn test_rate_limiting() {
        let rate_limiters = Arc::new(Mutex::new(HashMap::new()));
        let max_requests = 2;
        let window_duration = Duration::from_secs(1);

        let middleware = UnifiedMiddleware::new(
            "*".to_string(),
            rate_limiters,
            max_requests,
            window_duration,
            Rc::new(|_| true),
            None,
        );

        let app =
            init_service(App::new().wrap(middleware).route("/test", web::get().to(test_handler)))
                .await;

        // first requests - should be authorized
        for _ in 0..max_requests {
            let req = TestRequest::get().uri("/test").to_request();
            let resp = app.call(req).await.unwrap();
            assert_eq!(resp.status(), StatusCode::OK);
        }

        // request exceeding the limit - should be rejected
        let req = TestRequest::get().uri("/test").to_request();
        let resp = app.call(req).await;
        assert!(resp.is_err());
    }

    #[actix_web::test]
    async fn test_allowed_origins() {
        let rate_limiters = Arc::new(Mutex::new(HashMap::new()));
        let allowed_origins = "https://example.com,https://test.com".to_string();

        let middleware = UnifiedMiddleware::new(
            allowed_origins,
            rate_limiters,
            100,
            Duration::from_secs(60),
            Rc::new(|_| true),
            None,
        );

        let app =
            init_service(App::new().wrap(middleware).route("/test", web::get().to(test_handler)))
                .await;

        // Authorized origin
        let mut req = TestRequest::get().uri("/test");
        req = req.insert_header((header::ORIGIN, "https://example.com"));
        let resp = app.call(req.to_request()).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // not allowed origin
        let mut req = TestRequest::get().uri("/test");
        req = req.insert_header((header::ORIGIN, "https://unauthorized.com"));
        let resp = app.call(req.to_request()).await;
        assert!(resp.is_err());
    }

    #[actix_web::test]
    async fn test_reset_rate_limiting_window() {
        let rate_limiters = Arc::new(Mutex::new(HashMap::new()));
        let max_requests = 1;
        let window_duration = Duration::from_millis(10); // short duration for the test

        let middleware = UnifiedMiddleware::new(
            "*".to_string(),
            rate_limiters.clone(),
            max_requests,
            window_duration,
            Rc::new(|_| true),
            None,
        );

        let app =
            init_service(App::new().wrap(middleware).route("/test", web::get().to(test_handler)))
                .await;

        // First request - should be authorized
        let req = TestRequest::get().uri("/test").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // second immediate request - should be rejected
        let req = TestRequest::get().uri("/test").to_request();
        let resp = app.call(req).await;
        assert!(resp.is_err());

        // Wait until the window expires
        tokio::time::sleep(window_duration * 2).await;

        // new request after expiration - should be authorized
        let req = TestRequest::get().uri("/test").to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
