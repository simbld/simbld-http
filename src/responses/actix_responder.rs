use crate::helpers::http_code_helper::HttpCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

/// The code defines a custom response struct in Rust for handling HTTP responses in Actix-web.
///
/// Properties (explanation in English for clarity):
/// * `code`: The HTTP status code (u16).
/// * `name`: The name associated with the response (String).
/// * `data`: Extra data payload included in the response (String).
/// * `description`: A string describing the response further (String).

#[derive(Clone, Debug)]
pub struct CustomResponse {
    pub http_code: HttpCode,
    pub name: String,
    pub data: String,
    pub description: String,
}

impl CustomResponse {
    /// Creates a new `CustomResponse` from an HTTP code (standard or internal)
    /// and some relevant data.
    ///
    /// # Example
    /// ```
    /// use simbld_http::responses::CustomResponse;
    /// use simbld_http::helpers::http_code_helper::HttpCode;
    ///
    /// let response = CustomResponse::new(200, ("Ok"), ("Success Message"), ("Success"));
    /// assert_eq!(response.http_code, 200);
    /// assert_eq!(response.name, "OK".to_string());
    /// assert_eq!(response.data, "Success Message".to_string());
    /// assert_eq!(response.description, "Success".to_string());
    /// ```
    pub fn new(
        code: u16,
        name: impl Into<String>,
        data: impl Into<String>,
        description: Into<String>,
    ) -> Self {
    }
}

/// We implement the `Responder` trait for `CustomResponse`,
/// so that we can return `CustomResponse` directly in Actix handlers.
impl Responder for CustomResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(
            actix_web::http::StatusCode::from_u16(self.http_code.standard_code).unwrap(),
        )
        .json(json!({
            "status": self.http_code.standard_code,
            "name": self.http_code.standard_name,
            "description": self.http_code.unified_description,
            "data": self.data,
        }))
    }
}

/// Handler compatible with Actix-Web.
///
/// * Takes a `web::Data<CustomResponse>` (shared state) and a `HttpRequest`
/// * Clones the `CustomResponse`
/// * Calls `.respond_to(&req)` to build the final `HttpResponse`.
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
    use actix_web::{http::StatusCode, test, web, App};

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
            data: "Test data".to_string(),
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
}
