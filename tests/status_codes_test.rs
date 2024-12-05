use simbld_http::responses::wrapper::ResponseWrapper;
use simbld_http::responses::ResponsesClientCodes;
use simbld_http::responses::ResponsesCrawlerCodes;
use simbld_http::responses::ResponsesInformationalCodes;
use simbld_http::responses::ResponsesLocalApiCodes;
use simbld_http::responses::ResponsesRedirectionCodes;
use simbld_http::responses::ResponsesServerCodes;
use simbld_http::responses::ResponsesServiceCodes;
use simbld_http::responses::ResponsesSuccessCodes;

#[test]
fn test_generate_responses_status_codes() {
  ResponseWrapper::<ResponsesInformationalCodes>::generate_responses();
  ResponseWrapper::<ResponsesInformationalCodes>::generate_responses_with_metadata();
  ResponseWrapper::<ResponsesSuccessCodes>::generate_responses();
  ResponseWrapper::<ResponsesSuccessCodes>::generate_responses_with_metadata();
  ResponseWrapper::<ResponsesRedirectionCodes>::generate_responses();
  ResponseWrapper::<ResponsesRedirectionCodes>::generate_responses_with_metadata();
  ResponseWrapper::<ResponsesClientCodes>::generate_responses();
  ResponseWrapper::<ResponsesClientCodes>::generate_responses_with_metadata();
  ResponseWrapper::<ResponsesServerCodes>::generate_responses();
  ResponseWrapper::<ResponsesServerCodes>::generate_responses_with_metadata();
  ResponseWrapper::<ResponsesServiceCodes>::generate_responses();
  ResponseWrapper::<ResponsesServiceCodes>::generate_responses_with_metadata();
  ResponseWrapper::<ResponsesCrawlerCodes>::generate_responses();
  ResponseWrapper::<ResponsesCrawlerCodes>::generate_responses_with_metadata();
  ResponseWrapper::<ResponsesLocalApiCodes>::generate_responses();
  ResponseWrapper::<ResponsesLocalApiCodes>::generate_responses_with_metadata();
}
