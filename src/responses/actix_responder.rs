//! # Custom HTTP Response Implementation
//!
//! This module provides a customizable HTTP response structure for Actix Web applications.
//! The `CustomResponse` type implements Actix's `Responder` trait, allowing it to be
//! directly returned from route handlers with full control over status codes, headers,
//! and response content.
//!
//! ## Usage
//!
//! Create a custom response with your desired status code, name, data, and description:
//!
//! ```no_run
//! use simbld_http::responses::CustomResponse;
//!
//! // Create a custom success response
//! let success_response = CustomResponse::new(
//!     200,
//!     "Success",
//!     "{\"message\": \"Operation completed\"}",
//!     "Request was successful"
//! );
//!
//! // Create a custom error response
//! let error_response = CustomResponse::new(
//!     404,
//!     "NotFound",
//!     "{\"error\": \"Resource not available\"}",
//!     "The requested resource could not be found"
//! );
//! ```
//!
//! Use it in Actix Web handlers:
//!
//! ```no_run
//! # use actix_web::{web, Responder};
//! # use simbld_http::responses::CustomResponse;
//!
//! async fn success_handler() -> impl Responder {
//!     CustomResponse::new(
//!         200,
//!         "Success",
//!         "{\"message\": \"Operation completed\"}",
//!         "Request was successful"
//!     )
//! }
//!
//! async fn not_found_handler() -> impl Responder {
//!     CustomResponse::new(
//!         404,
//!         "NotFound",
//!         "{\"error\": \"Resource not found\"}",
//!         "The requested resource could not be found"
//!     )
//! }
//! ```

use crate::helpers::http_code_helper::HttpCode;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

/// A customizable HTTP response for Actix Web applications.
///
/// `CustomResponse` provides a flexible way to create HTTP responses with
/// custom status codes, data payloads, and metadata. It implements Actix's
/// `Responder` trait, making it directly returnable from route handlers.
#[derive(Debug, Serialize, Clone)]
pub struct CustomResponse {
    /// The HTTP status code and associated metadata
    pub http_code: HttpCode,

    /// Name identifier for the response (useful for logging and debugging)
    pub name: String,

    /// The response payload (typically JSON or XML formatted as a string)
    pub data: String,

    /// Human-readable description of the response
    pub description: String,
}

impl CustomResponse {
    /// Creates a new CustomResponse with the provided information.
    ///
    /// # Arguments
    ///
    /// * `code` - HTTP status code (e.g., 200, 404, 500)
    /// * `name` - Name identifier for the response
    /// * `data` - Response payload (typically JSON or XML)
    /// * `description` - Human-readable description
    ///
    /// # Returns
    ///
    /// A new CustomResponse instance
    pub fn new(
        code: u16,
        name: impl Into<String>,
        data: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        // Convert the input parameters into strings
        let name_str = name.into();
        let data_str = data.into();
        let desc_str = description.into();

        // Leak the strings to create static references
        let leaked_name: &'static str = Box::leak(name_str.clone().into_boxed_str());
        let leaked_desc: &'static str = Box::leak(desc_str.clone().into_boxed_str());

        // Create a new HttpCode instance
        let resolved_http_code = HttpCode::new(code, leaked_name, leaked_desc, code, leaked_name);

        Self {
            http_code: resolved_http_code,
            name: name_str,
            data: data_str,
            description: desc_str,
        }
    }
}

/// Implements Actix's Responder trait for CustomResponse.
///
/// This allows CustomResponse instances to be returned directly from
/// route handlers, with automatic conversion to HttpResponse.
impl Responder for CustomResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        // Utiliser standard_code au lieu de code
        let mut response =
            HttpResponse::build(StatusCode::from_u16(self.http_code.standard_code).unwrap());

        // Ajouter les headers pertinents
        response.content_type("application/json");

        // Construire la réponse avec les données
        response.body(self.data)
    }
}

/// A handler that returns a pre-configured CustomResponse.
///
/// This handler can be used with a shared CustomResponse provided via
/// the app's application data.
///
/// # Arguments
///
/// * `custom_response` - A shared CustomResponse injected via web::Data
/// * `req` - The HTTP request
///
/// # Returns
///
/// An HttpResponse based on the provided CustomResponse
pub async fn custom_response_handler(
    custom_response: web::Data<CustomResponse>,
    req: HttpRequest,
) -> HttpResponse {
    let response = custom_response.get_ref().clone(); // Clone the struct
    response.respond_to(&req)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::HttpServer;
    use actix_web::{http::StatusCode, test, web, App};

    /// Example handler that uses the new constructor with four arguments.
    async fn example_response() -> impl Responder {
        CustomResponse::new(
            200,
            "Success",
            "{\"message\": \"Request was successful\", \"data\": \"Test data\"}",
            "Test response description",
        )
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| App::new().default_service(web::route().to(example_response)))
            .bind("127.0.0.1:8090")?
            .run()
            .await
    }

    #[actix_web::test]
    async fn test_custom_response_responder() {
        // Step 1: Create a custom response
        let custom_response = CustomResponse {
            http_code: HttpCode {
                standard_code: 200,
                standard_name: "OK",
                unified_description: "Success",
                internal_code: Some(200),
                internal_name: Some("OK"),
            },
            name: "".to_string(),
            data: "Test data".to_string(),
            description: "".to_string(),
        };

        // Step 2: Initialize an Actix-Web application with a handler
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(custom_response))
                .default_service(web::route().to(custom_response_handler)),
        )
        .await;

        // Step 3: Simulate an HTTP request
        let req = test::TestRequest::default().to_request();
        let resp = test::call_service(&app, req).await;

        // Step 4: Assert the response
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_example_response() {
        // Create an app with our function like handler
        let app = test::init_service(App::new().route("/", web::get().to(example_response))).await;

        // Simulate a request to our endpoint
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        // Check the status code
        assert_eq!(resp.status(), StatusCode::OK);

        // Check the content of the answer
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        // Make sure the body contains the expected data
        assert!(body_str.contains("Test data"));
        assert!(body_str.contains("Request was successful"));
    }
}
