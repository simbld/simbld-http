use simbld_http::helpers::to_u16_helper::ToU16;
use simbld_http::responses::{
  wrapper::ResponseWrapper, ResponsesClientCodes, ResponsesCrawlerCodes,
  ResponsesInformationalCodes, ResponsesLocalApiCodes, ResponsesRedirectionCodes,
  ResponsesServerCodes, ResponsesServiceCodes, ResponsesSuccessCodes, ResponsesTypes,
};
use strum::EnumProperty;

#[test]
fn test_snake_case_function_generation() {
  ResponseWrapper::<ResponsesSuccessCodes>::generate_responses();
  let response = ResponsesSuccessCodes::Ok;
  assert_eq!(response.to_u16(), 200);
  assert_eq!(response.get_str("Description").unwrap_or("No description"), "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response");
}

#[test]
fn test_informational_codes() {
  let response = ResponsesInformationalCodes::Processing;
  assert_eq!(response.to_u16(), 102);
  assert_eq!(
        response.get_str("Description").unwrap_or("No description"),
        "Indicates the server is processing the request but has not yet finished, used to prevent timeout errors in asynchronous operations, webdav RFC 2518"
    );
}

#[test]
fn test_success_codes() {
  let response = ResponsesSuccessCodes::Accepted;
  assert_eq!(response.to_u16(), 202);
  assert_eq!(
    response.get_str("Description").unwrap_or("No description"),
        "Request processed, but with no guarantee of results, and no indication of the final status of the request, which will be processed asynchronously, such as a request to create a new resource"
    );
}

#[test]
fn test_redirection_codes() {
  let response = ResponsesRedirectionCodes::TooManyRedirects;
  assert_eq!(response.to_u16(), 310);
  assert_eq!(
    response.get_str("Description").unwrap_or("No description"),
        "The client has been redirected too many times, possibly causing a redirection loop. This status code is used to prevent infinite redirection loops"
    );
}

#[test]
fn test_client_codes() {
  let response = ResponsesClientCodes::Gone;
  assert_eq!(response.to_u16(), 410);
  assert_eq!(
    response.get_str("Description").unwrap_or("No description"),
        "The requested resource is no longer available and has been permanently removed from the server and will not be available again"
    );
}

#[test]
fn test_server_codes() {
  let response = ResponsesServerCodes::NetworkAuthenticationRequired;
  assert_eq!(response.to_u16(), 511);
  assert_eq!(
    response.get_str("Description").unwrap_or("No description"),
        "The network connection requires authentication before accessing the requested resources. This is often used by captive portals to redirect users to a login page"
    );
}

#[test]
fn test_service_codes() {
  let response = ResponsesServiceCodes::ReadingError;
  assert_eq!(response.to_u16(), 611);
  assert_eq!(
    response.get_str("Description").unwrap_or("No description"),
    "An error occurred while reading the response or data from the server"
  );
}

#[test]
fn test_crawler_codes() {
  let response = ResponsesCrawlerCodes::ExcludedByRobotsTxtFile;
  assert_eq!(response.to_u16(), 740);
  assert_eq!(
    response.get_str("Description").unwrap_or("No description"),
    "Excluded by robots.txt file"
  );
}

#[test]
fn test_local_api_codes() {
  let response = ResponsesLocalApiCodes::RequestDenied;
  assert_eq!(response.to_u16(), 999);
  assert_eq!(
    response.get_str("Description").unwrap_or("No description"),
        "Unofficial HTTP status code LinkedIn that is returned by the server as a generic, or catch-all error code. The reason for the HTTP response varies based on the service or host"
    );
}

#[test]
fn test_response_types_mapping() {
  let response = ResponsesTypes::LocalApiError(ResponsesLocalApiCodes::RequestDenied);

  if let ResponsesTypes::LocalApiError(local_api_error) = response {
    assert_eq!(
            local_api_error.get_str("Description").unwrap_or("No description"),
            "Unofficial HTTP status code LinkedIn that is returned by the server as a generic, or catch-all error code. The reason for the HTTP response varies based on the service or host"
        );
  } else {
    panic!("Expected LocalApiError variant");
  }
}

#[test]
fn test_responses_types_from_u16() {
  let response = ResponsesTypes::from_u16(404);
  assert!(matches!(response, Some(ResponsesTypes::ClientError(_))));
}

#[test]
fn test_responses_types_to_u16() {
  let response = ResponsesTypes::ClientError(ResponsesClientCodes::NotFound);
  assert_eq!(response.to_u16(), 404);
}

#[test]
fn test_responses_types_description() {
  let response = ResponsesTypes::ClientError(ResponsesClientCodes::NotFound);
  assert_eq!(
    response.description(),
    "The server cannot find the requested resource, indicating a non-existent or inaccessible URI"
  );
}
