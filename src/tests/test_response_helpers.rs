use simbld_http::helpers::response_helper;
use simbld_http::responses::{ResponsesSuccessCodes, ResponsesTypes};

#[test]
fn test_find_by_code() {
  let response = response_helper::find_by_code(200);
  assert!(response.is_some());
  assert_eq!(response.unwrap().to_u16(), 200);
}

#[test]
fn test_find_description_by_code() {
  let description = response_helper::find_description_by_code(200);
  assert!(description.is_some());
  assert_eq!(description.unwrap(), "Request succeeded");
}

#[test]
fn test_transform_to_json() {
  let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
  let json = response_helper::transform_to_json(response);
  assert_eq!(json, r#"{"code":200,"description":"Request succeeded"}"#);
}

#[test]
fn test_transform_to_xml() {
  let response = ResponsesTypes::Success(ResponsesSuccessCodes::Ok);
  let xml = response_helper::transform_to_xml(response);
  assert_eq!(
    xml,
    r#"<response><code>200</code><description>Request succeeded</description></response>"#
  );
}
