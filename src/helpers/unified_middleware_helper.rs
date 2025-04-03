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
    pub allowed_origins: HashSet<String>,
    pub rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    pub max_requests: usize,
    pub window_duration: Duration,
    pub intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
    pub condition: Rc<Box<dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static>>,
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

impl UnifiedMiddleware {
    pub fn new(
        allowed_origins: String,
        rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
        max_requests: usize,
        window_duration: Duration,
        intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
        condition: Option<Box<dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static>>,
    ) -> Self {
        let origins: HashSet<String> =
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
    allowed_origins: HashSet<String>,
    rate_limiters: Arc<Mutex<HashMap<String, (u64, Instant)>>>,
    max_requests: usize,
    window_duration: Duration,
    intercept_dependencies: Rc<dyn Fn(&ServiceRequest) -> bool>,
    condition: Rc<Box<dyn for<'a> Fn(&'a ServiceRequest) -> bool + 'static>>,
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

            // Check the origin if it is a CORS request
            if (*intercept)(&req) {
                if let Some(origin) = req.headers().get(header::ORIGIN) {
                    if let Ok(origin_str) = origin.to_str() {
                        if !allowed_origins.contains(origin_str) && !allowed_origins.contains("*") {
                            return Err(UnifiedError::Unauthorized.into());
                        }
                    }
                }

                // Check the rate limiting
                let client_ip = match req.connection_info().realip_remote_addr() {
                    Some(ip) => ip.to_string(),
                    None => "unknown".to_string(),
                };

                let mut should_limit = false;
                {
                    let mut limiters =
                        rate_limiters.lock().map_err(|_| UnifiedError::InternalMiddlewareError)?;

                    let now = Instant::now();
                    let entry = limiters.entry(client_ip).or_insert_with(|| (0, now));

                    if now.duration_since(entry.1) > window_duration {
                        // Reset the counter if the window has expired
                        *entry = (1, now);
                    } else {
                        // increment the counter
                        entry.0 += 1;
                        if entry.0 > max_requests as u64 {
                            should_limit = true;
                        }
                    }
                }

                if should_limit {
                    return Err(ActixError::from(UnifiedError::InvalidRequest));
                }
            }

            service.call(req).await
        })
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
