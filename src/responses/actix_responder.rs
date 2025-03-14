use crate::helpers::http_code_helper::HttpCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;

/// The code defines a custom response struct in Rust for handling HTTP responses in Actix-web.
///
/// Properties (explanation in English for clarity):
/// * `code`: The HTTP status code (u16).
/// * `name`: The name associated with the response (String).
/// * `data`: Extra data payload included in the response (String).
/// * `description`: A string describing the response further (String).

#[derive(Debug, Serialize, Clone)]
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
    /// let response = CustomResponse::new(200, "Ok", "Success Message", "Success");
    /// assert_eq!(response.http_code, 200);
    /// assert_eq!(response.name, "OK".to_string());
    /// assert_eq!(response.data, "Success Message".to_string());
    /// assert_eq!(response.description, "Success".to_string());
    /// ```
    pub fn new(
        code: u16,
        name: impl Into<String>,
        data: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        // Convertir les arguments en String
        let name_str: String = name.into();
        let description_str: String = description.into();

        // Créer l'HttpCode
        let resolved_http_code =
            HttpCode::new(code, name_str.clone(), description_str.clone(), code, name_str);

        // Retourner le CustomResponse en clonant name et description
        Self {
            http_code: resolved_http_code,
            name: name_str, // Pas besoin de clonage ici, car on a déjà transféré ownership
            data: data.into(),
            description: description_str, // Pas besoin de clonage ici non plus
        }
    }
}

/// We implement the `Responder` trait for `CustomResponse`,
/// so that we can return `CustomResponse` directly in Actix handlers.
impl Responder for CustomResponse {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(
            actix_web::http::StatusCode::from_u16(self.http_code.standard_code)
                .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
        )
        .json(json!({
            "status": self.http_code.standard_code,
            "name": self.name,
            "data": self.data,
            "description": self.description,
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
    use actix_web::HttpServer;
    use actix_web::{http::StatusCode, test, web, App};

    /// Example handler that uses the new constructor with four arguments.
    async fn example_response() -> impl Responder {
        CustomResponse::new(200, "Success", "Test data", "Request was successful")
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                // votre middleware ou configuration
                .default_service(web::route().to(example_response))
        })
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
}
