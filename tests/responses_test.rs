#[cfg(test)]
mod tests {
    use crate::tests::response_helpers::convert_to_enum;
    use crate::tests::response_helpers::create_response_with_types;
    use crate::tests::response_helpers::create_response_xml;
    use crate::tests::response_helpers::filter_codes_by_range;
    use crate::tests::response_helpers::filter_codes_by_range_with_metadata;
    use crate::tests::response_helpers::get_description_by_code;
    use crate::tests::response_helpers::list_codes_and_descriptions_short;
    use crate::tests::response_helpers::list_codes_and_descriptions_with_metadata;
    use crate::tests::response_helpers::transform_to_xml;
    use crate::tests::response_helpers::transform_to_xml_filtered;
    use crate::tests::response_helpers::transform_to_xml_with_metadata;
    use serde_json::json;
    use simbld_http::helpers::response_helpers;
    use simbld_http::helpers::response_helpers::{
        get_enriched_response_with_metadata, get_response_by_code, is_origin_allowed,
    };
    use simbld_http::helpers::response_with_cookie_helper::{
        bad_request_with_cookie, ok_with_cookie,
    };
    use simbld_http::helpers::response_with_headers_helper::{
        bad_request_with_headers, ok_with_headers,
    };
    use simbld_http::responses::ResponsesTypes;
    use simbld_http::ResponsesCrawlerCodes::ExcludedByRobotsTxtFile;
    use simbld_http::{ResponsesClientCodes, ResponsesServerCodes};
    use simbld_http::{ResponsesCrawlerCodes, ResponsesInformationalCodes, ResponsesSuccessCodes};
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_get_code() {
        let response = ResponsesInformationalCodes::Processing;
        assert_eq!(response.get_code(), 102);
    }

    #[test]
    fn test_as_tuple_result() {
        // Case where the internal and standard codes are identical
        let tuple_result = ResponsesTypes::Success(ResponsesSuccessCodes::Ok).as_tuple();
        assert_eq!(tuple_result.standard_code, 200);
        assert_eq!(tuple_result.standard_name, "OK");
        assert_eq!(tuple_result.unified_description, "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response");
        assert_eq!(tuple_result.internal_code, None);
        assert_eq!(tuple_result.internal_name, None);
    }

    #[test]
    fn test_as_tuple_result_diff() {
        // Case where the internal and standard codes are different
        let tuple_result_diff = ResponsesTypes::CrawlerError(ExcludedByRobotsTxtFile).as_tuple();
        assert_eq!(tuple_result_diff.standard_code, 403);
        assert_eq!(tuple_result_diff.standard_name, "Forbidden");
        assert_eq!(tuple_result_diff.unified_description, "Excluded by robots.txt file");
        assert_eq!(tuple_result_diff.internal_code, Some(740));
        assert_eq!(tuple_result_diff.internal_name, Some("Excluded by Robots.txt file"));
    }

    #[test]
    fn test_as_json() {
        let json_value = ResponsesTypes::CrawlerError(ExcludedByRobotsTxtFile).as_json();
        let expected_json = json!({
            "type": "Crawler responses",
            "details": {
                "standard http code": {
                    "code": 403,
                    "name": "Forbidden"
                },
                "internal http code": {
                    "code": 740,
                    "name": "Excluded by Robots.txt file"
                },
                "description": "Excluded by robots.txt file",
            }
        });
        assert_eq!(json_value, expected_json);
    }

    #[test]
    fn test_from_u16() {
        assert_eq!(
            ResponsesTypes::from_u16(700),
            Some(ResponsesTypes::CrawlerError(ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader))
        );
        assert_eq!(ResponsesTypes::from_u16(999), None);
    }

    #[test]
    fn test_get_advance_response_get_description() {
        let client_error = ResponsesTypes::ClientError(ResponsesClientCodes::SSLCertificateError);
        let (standard_code, unified_description) =
            response_helpers::get_advance_response_get_description(client_error);

        assert_eq!(standard_code, 400);
        assert_eq!(unified_description, "An invalid or untrusted SSL certificate was encountered");
    }

    #[test]
    fn test_get_response_by_code() {
        let response = get_response_by_code(200);
        assert!(response.is_some());
        assert_eq!(response.unwrap().get_code(), 200);
    }

    #[test]
    fn test_get_description_by_code() {
        let description = get_description_by_code(200);
        assert!(description.is_some());
        assert_eq!(
            description.unwrap(),
            "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
        );
    }
    fn transform_to_json(response: ResponsesTypes) -> String {
        match response {
            ResponsesTypes::Success(success_code) => {
                match success_code {
                    ResponsesSuccessCodes::Ok => {
                        json!({
                        "code": 200,
                        "description": "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
                    }).to_string()
                    },
                    ResponsesSuccessCodes::Created => {
                        json!({
                        "code": 201,
                        "description": "Request has been fulfilled and resulted in a new resource being created"
                    }).to_string()
                    },
                    _ => {
                        json!({
                        "code": 200,
                        "description": "Success"
                    }).to_string()
                    }
                }
            },
            ResponsesTypes::ClientError(client_error_code) => {
                match client_error_code {
                    ResponsesClientCodes::BadRequest => {
                        json!({
                        "code": 400,
                        "description": "The server cannot or will not process the request due to an apparent client error"
                    }).to_string()
                    },

                    _ => {
                        json!({
                        "code": 400,
                        "description": "Client Error"
                    }).to_string()
                    }
                }
            },
            ResponsesTypes::ServerError(server_error_code) => {
                match server_error_code {
                    ResponsesServerCodes::InternalServerError => {
                        json!({
                        "code": 500,
                        "description": "A generic error message, given when an unexpected condition was encountered"
                    }).to_string()
                    },

                    _ => {
                        json!({
                        "code": 500,
                        "description": "Server Error"
                    }).to_string()
                    }
                }
            },
            _ => {
                json!({
                "code": 0,
                "description": "Unknown response type"
            }).to_string()
            }
        }
    }

    #[test]
    fn test_response_to_json() {
        let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
        let json_str = transform_to_json(response);
        let expected_json = json!({
        "code": 200,
        "description": "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
    }).to_string();
        assert_eq!(json_str, expected_json);
    }

    fn transform_to_json_with_metadata(response: ResponsesTypes) -> String {
        let code = response.get_code();
        let description = response.get_description();
        let status_family = match response {
            ResponsesTypes::Informational(_) => "Informational",
            ResponsesTypes::Success(_) => "Success",
            ResponsesTypes::Redirection(_) => "Redirection",
            ResponsesTypes::ClientError(_) => "ClientError",
            ResponsesTypes::ServerError(_) => "ServerError",
            ResponsesTypes::LocalApiError(_) => "LocalApiError",
            ResponsesTypes::CrawlerError(_) => "CrawlerError",
            ResponsesTypes::ServiceError(_) => "ServiceError",
        };

        let now = chrono::Utc::now();
        let requested_at = now.to_rfc3339();

        json!({
            "code": code,
            "description": description,
            "status_family": status_family,
            "requested_at": requested_at
        })
        .to_string()
    }

    #[test]
    fn test_transform_to_json_with_metadata() {
        let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
        let json_str = transform_to_json_with_metadata(response);
        assert!(json_str.contains("\"requested_at\""));
        assert!(json_str.contains("\"status_family\":\"Success\""));
    }

    fn transform_to_json_filtered(response: ResponsesTypes) -> Result<String, &'static str> {
        let code = response.get_code();
        let description = response.get_description();

        let is_standard_code = match code {
            100..=103 => true,
            200..=208 | 226 => true,
            300..=308 => true,
            400..=418 | 421..=426 | 428 | 429 | 431 | 451 => true,
            500..=511 => true,
            _ => false,
        };

        if !is_standard_code && false {
            return Err("Not a standard HTTP status code");
        }

        Ok(json!({
            "code": code,
            "description": description,
            "is_standard_code": is_standard_code
        })
        .to_string())
    }

    #[test]
    fn test_transform_to_json_filtered() {
        let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
        let json_str = transform_to_json_filtered(response).unwrap();
        assert!(json_str.contains("\"is_standard_code\":true"));
    }

    #[test]
    fn test_transform_to_xml() {
        let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
        let xml_str = transform_to_xml(response);
        let expected_xml = r#"<response><code>200</code><description>Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response</description></response>"#;
        assert_eq!(xml_str, expected_xml);
    }

    #[test]
    fn test_transform_to_xml_with_metadata() {
        let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
        let xml_str = transform_to_xml_with_metadata(response);
        assert!(xml_str.contains("<requested_at>"));
        assert!(xml_str.contains("<status_family>Success</status_family>"));
    }

    #[test]
    fn test_transform_to_xml_filtered() {
        let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
        let xml_str = transform_to_xml_filtered(response).unwrap();
        assert!(xml_str.contains("<is_standard_code>true</is_standard_code>"));
    }

    #[test]
    fn test_filter_codes_by_range() {
        let codes = filter_codes_by_range(100, 103);
        let expected_codes = vec![
            (
                100,
                "The server has received the initial part of the request, the headers, and asks the client to continue request, proceed to send the body of the request, a POST request",
            ),
            (
                101,
                "The server is complying with a request to switch protocols, used in WebSocket connections",
            ),
            (
                102,
                "Indicates the server is processing the request but has not yet finished, used to prevent timeout errors in asynchronous operations, webdav RFC 2518",
            ),
            (
                103,
                "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared",
            ),
        ];
        assert_eq!(codes, expected_codes);
    }

    #[test]
    fn test_filter_codes_by_range_with_metadata() {
        let request_metadata = Some(std::collections::HashMap::from([("source", "test_case")]));
        let codes = filter_codes_by_range_with_metadata(200, 299, request_metadata);
        assert!(!codes.is_empty());
        assert!(codes[0].2.contains_key("source"));
    }

    #[test]
    fn test_filter_codes_by_range_by_length() {
        let codes = filter_codes_by_range(200, 201);
        assert_eq!(codes.len(), 2);
    }

    #[test]
    fn test_list_codes_and_descriptions_with_metadata() {
        let request_metadata = Some(std::collections::HashMap::from([("source", "unit_test")]));
        let descriptions = list_codes_and_descriptions_with_metadata("Success", request_metadata);
        assert!(!descriptions.is_empty());
        assert!(descriptions[0].2.contains_key("source"));
    }

    #[test]
    fn test_list_codes_and_descriptions_short() {
        let informational = list_codes_and_descriptions_short("Informational");
        assert!(!informational.is_empty());
        assert_eq!(informational[0].0, 100);

        let success = list_codes_and_descriptions_short("Success");
        assert!(!success.is_empty());
        assert_eq!(success[0].0, 200);

        let unknown = list_codes_and_descriptions_short("UnknownFamily");
        assert!(unknown.is_empty());
    }

    #[test]
    fn test_create_response_with_types() {
        let response = Some(ResponsesTypes::Success(ResponsesSuccessCodes::Ok));
        let json_response = create_response_with_types(response, Some(r#"{"key": "value"}"#));
        println!("JSON Response: {}", json_response);
        assert!(json_response.contains("\"description\":\"Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response\""));
        assert!(json_response.contains("\"key\":\"value\""));
    }

    #[test]
    fn test_create_response() {
        fn create_response_json(code: u16, description: &str, data: &str) -> String {
            let data_value: serde_json::Value =
                serde_json::from_str(data).unwrap_or(serde_json::Value::Null);

            let response = json!({
                "code": code,
                "description": description,
                "data": data_value
            });

            response.to_string()
        }

        let code = 200;
        let description = "OK";
        let data = r#"{"message": "Success"}"#;
        let response = create_response_json(code, description, data);

        let response_value: serde_json::Value = serde_json::from_str(&response).unwrap();
        let expected_value = json!({
            "code": 200,
            "description": "OK",
            "data": {
                "message": "Success"
            }
        });

        assert_eq!(response_value, expected_value);
    }

    #[test]
    fn test_create_response_without_json() {
        let code = 200;
        let description = "OK";
        let data = r#"{"message": "Success"}"#;
        let response = create_response_xml(code, description, data);

        assert!(response.contains("<code>200</code>"));
        assert!(response.contains("<description>OK</description>"));
        assert!(response.contains("<data>{\"message\": \"Success\"}</data>"));
        assert!(response.contains("<response>"));
    }

    #[test]
    fn test_create_response_xml() {
        let code = 200;
        let description = "OK";
        let data = "<message>Success</message>";
        let response = create_response_xml(code, description, data);
        let expected_response = format!(
            r#"<response><code>{}</code><description>{}</description><data>{}</data></response>"#,
            code, description, data
        );
        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_convert_to_enum() {
        let code = 200;
        let response_type = convert_to_enum(code);
        assert!(response_type.is_some());
        assert_eq!(response_type.unwrap().get_code(), 200);
    }

    #[test]
    fn test_get_enriched_response_with_metadata() {
        let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
        let enriched_response = get_enriched_response_with_metadata(
            response,
            Some("https://example.com"),
            Duration::from_millis(150),
        );
        assert!(enriched_response.contains("\"code\":200"));
        assert!(
            enriched_response.contains("\"Access-Control-Allow-Origin\":\"https://example.com\"")
        );
    }

    #[test]
    fn test_is_origin_allowed() {
        let allowed_origins = vec!["https://example.com", "https://localhost"];
        assert!(is_origin_allowed("https://example.com", &allowed_origins));
        assert!(!is_origin_allowed("https://unauthorized.com", &allowed_origins));
    }

    #[test]
    fn test_ok_with_cookie() {
        let response = ok_with_cookie(("session_id", "abc123"));
        assert!(response.contains("\"status\":\"OK\""));
        assert!(response.contains("\"code\":200"));
        assert!(response.contains("\"cookie\":{\"key\":\"session_id\",\"value\":\"abc123\"}"));
    }

    #[test]
    fn test_bad_request_with_cookie() {
        let response = bad_request_with_cookie(("error_id", "xyz789"));
        assert!(response.contains("\"status\":\"Bad Request\""));
        assert!(response.contains("\"code\":400"));
        assert!(response.contains("\"cookie\":{\"key\":\"error_id\",\"value\":\"xyz789\"}"));
    }

    #[test]
    fn test_ok_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        let response = ok_with_headers(headers.clone());
        assert!(response.contains("\"status\":\"OK\""));
        assert!(response.contains("\"code\":200"));
        assert!(response.contains("\"description\":\"Request processed successfully\""));
        assert!(response.contains("\"headers\":{"));
        assert!(response.contains("\"Content-Type\":\"application/json\""));
    }

    #[test]
    fn test_bad_request_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        let response = bad_request_with_headers(headers.clone());
        assert!(response.contains("\"status\":\"Bad Request\""));
        assert!(response.contains("\"code\":400"));
        assert!(response.contains(
            "\"description\":\"The server cannot process the request due to malformed syntax\""
        ));
        assert!(response.contains("\"headers\":{"));
        assert!(response.contains("\"Content-Type\":\"application/json\""));
    }
}
