use crate::generate_responses_functions;

generate_responses_functions! {
  "",
    ResponsesClientCodes,
  BadRequest => (400, "Bad Request", "The server cannot or will not process the request due to something that is perceived to be a client error (e.g., malformed request syntax, invalid request message framing, or deceptive request routing).", 400, "Bad Request"),
  Unauthorized => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 401, "Unauthorized"),
  PaymentRequired => (402, "Payment Required", "The initial purpose of this code was for digital payment systems, however this status code is rarely used and no standard convention exists.", 402, "Payment Required"),
  Forbidden => (403, "Forbidden", "The client does not have access rights to the content; that is, it is unauthorized, so the server is refusing to give the requested resource. Unlike 401 Unauthorized, the client's identity is known to the server.", 403, "Forbidden"),
  NotFound => (404, "Not Found", "The server cannot find the requested resource. In the browser, this means the URL is not recognized. In an API, this can also mean that the endpoint is valid but the resource itself does not exist. Servers may also send this response instead of 403 Forbidden to hide the existence of a resource from an unauthorized client.", 404, "Not Found"),
  MethodNotAllowed => (405, "Method Not Allowed", "The HTTP method is not supported for the target resource", 405, "Method Not Allowed"),
  NotAcceptable => (406, "Not Acceptable", "This response is sent when the web server, after performing server-driven content negotiation, doesn't find any content that conforms to the criteria given by the user agent.", 406, "Not Acceptable"),
  ProxyAuthenticationRequired => (407, "Proxy Authentication Required", "The client must authenticate with a proxy first", 407, "Proxy Authentication Required"),
  RequestTimeout => (408, "Request Timeout", "This response is sent on an idle connection by some servers, even without any previous request by the client. It means that the server would like to shut down this unused connection. This response is used much more since some browsers use HTTP pre-connection mechanisms to speed up browsing. Some servers may shut down a connection without sending this message.", 408, "Request Timeout"),
  Conflict => (409, "Conflict", "This response is sent when a request conflicts with the current state of the server. In WebDAV remote web authoring, 409 responses are errors sent to the client so that a user might be able to resolve a conflict and resubmit the request.", 409, "Conflict"),
  Gone => (410, "Gone", "This response is sent when the requested content has been permanently deleted from server, with no forwarding address. Clients are expected to remove their caches and links to the resource. The HTTP specification intends this status code to be used for 'limited-time, promotional services'. APIs should not feel compelled to indicate resources that have been deleted with this status code.", 410, "Gone"),
  LengthRequired => (411, "Length Required", "Server rejected the request because the Content-Length header field is not defined and the server requires it.", 411, "Length Required"),
  PreconditionFailed => (412, "Precondition Failed", "In conditional requests, the client has indicated preconditions in its headers which the server does not meet.", 412, "Precondition Failed"),
  ContentTooLarge => (413, "Payload Too Large", "The request or resource is too large for the server to handle", 413, "Content Too Large"),
  URITooLong => (414, "URI Too Long", "The URI requested by the client is longer than the server is willing to interpret.", 414, "URI Too Long"),
  UnsupportedMediaType => (415, "Unsupported Media Type", "The media format of the requested data is not supported by the server, so the server is rejecting the request.", 415, "Unsupported Media Type"),
  RangeNotSatisfiable => (416, "Range Not Satisfiable", "The range specified by the request's Range header field cannot be satisfied; the range may exceed the size of the data coming from the targeted URI.", 416, "Range Not Satisfiable"),
  ExpectationFailed => (417, "Expectation Failed", "This response code means that the expectations indicated by the Expect request header field could not be met by the server.", 417, "Expectation Failed"),
  ImATeapot => (418, "I'm a teapot", "The waiter refuses to brew coffee with a teapot, RFC 2324.", 418, "I'm a teapot"),
  PageExpired => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 419, "PageExpired"),
  MethodFailure => (405, "Method Not Allowed", "The HTTP method is not supported for the target resource", 420, "MethodFailure"),
  MisdirectedRequest => (421, "Misdirected Request", "The request was sent to a server unable to produce a response. This code may be sent by a server that has not been configured to produce responses subject to the combination of schemas and identities included in the request URI.", 421, "Misdirected Request"),
  UnprocessableEntity => (422, "Unprocessable Entity", "The request was successfully created but could not be processed due to semantic errors, WebDAV RFC 4918.", 422, "Unprocessable Entity"),
  Locked => (423, "Locked", "The resource that is currently being viewed is locked.", 423, "Locked"),
  FailedDependency => (424, "Failed Dependency", "The query failed because a previous query failed.", 424, "Failed Dependency"),
  TooEarly => (422, "Unprocessable Entity", "Indicate that the server does not want to process a request that could be replayed.", 425, "Too Early"),
  UpgradeRequired => (426, "Upgrade Required", "The server refuses to process the request using the current protocol but may agree to do so if the client opts for another protocol. The server must send an Upgrade header in the 426 response to indicate the requested protocol(s) (Section 6.7 of [RFC7230]).", 426, "Upgrade Required"),
  PreconditionRequired => (428, "Precondition Required", "The origin server requires the request to be conditional. This is intended to prevent the 'loss of update' problem, where a client retrieves the state of a resource with GET, modifies it, and returns it to the server with PUT while a third party modifies the state of the resource. server, which leads to a conflict.", 428, "Precondition Required"),
  TooManyRequests => (429, "Too Many Requests", "The user has sent too many requests in a given amount of time (rate limiting).", 429, "Too Many Requests"),
  RequestHeaderFieldsTooLarge => (431, "Request Header Fields Too Large", "The server is unwilling to process the request because the header fields are too long. The request can be returned after reducing the size of the headers.", 431, "Request Header Fields Too Large"),
  LoginRequired => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 432, "Login Required"),
  OriginError => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 433, "Origin Error"),
  DestinationError => (400, "Bad Request", "The request was rejected due to a destination server/config issue", 434, "DestinationError"),
  TooLarge => (413, "Payload Too Large", "The request or resource is too large for the server to handle", 435, "TooLarge"),
  SSLCertificateError => (400, "Bad Request", "An invalid or untrusted SSL certificate was encountered", 436, "SSLCertificateError"),
  SSLCertificateRequired => (400, "Bad Request", "The server requires a valid SSL certificate for secure connections", 437, "SSLCertificateRequired"),
  NoCertificate => (400, "Bad Request", "No SSL certificate was provided by the client", 438, "NoCertificate"),
  LoginTimeout => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 440, "LoginTimeout"),
  OverDataQuota => (413, "Payload Too Large", "The request or resource is too large for the server to handle", 441, "OverDataQuota"),
  NoResponse => (400, "Bad Request", "The server closed the connection without sending a response", 444, "NoResponse"),
  RetryWith => (428, "Precondition Required", "The user has sent too many requests in a given amount of time (rate limiting).", 449, "RetryWith"),
  BlockedByWindowsParentalControls => (403, "Forbidden", "A Microsoft extension. This error is given when Windows Parental Controls are turned on and are blocking access to the given webpage.", 450, "BlockedByWindowsParentalControls"),
  UnavailableForLegalReasons => (451, "Unavailable For Legal Reasons", "A server operator has received a legal demand to deny access to a resource or to a set of resources that includes the requested resource.", 451, "UnavailableForLegalReasons"),
  TooManyRecipients => (429, "Too Many Requests",  "The user has sent too many requests in a given amount of time (rate limiting) or too many recipients or addresses used", 452, "TooManyRecipients"),
  MethodNotValidInThisState => (405, "Method Not Allowed", "The HTTP method is not supported for the target resource", 453, "MethodNotValidInThisState"),
  UnrecoverableError => (400, "Bad Request", "The server has encountered a situation it doesn't know how to handle.", 456, "UnrecoverableError"),
  ClientClosedConnexionPrematurely => (400, "Bad Request", "The client closed the connection before the server could send a response.", 499, "ClientClosedConnexionPrematurely"),
  TooManyForwardedIPAddresses => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 445, "TooManyForwardedIPAddresses"),
  InternetSecurityError => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 446, "InternetSecurityError"),
  RequestHeaderTooLarge => (431, "Request Header Fields Too Large", "The server is unwilling to process the request because the header fields are too long. The request can be returned after reducing the size of the headers.", 494, "RequestHeaderTooLarge"),
  CertError => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 495, "CertError"),
  NoCert => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 496, "NoCert"),
  HTTPToHTTPS => (400, "Bad Request", "The request was rejected due to an origin server/client IP issue", 497, "HTTPToHTTPS"),
  InvalidToken => (401, "Unauthorized", "Although the HTTP standard specifies 'unauthorized', semantically this response means 'unauthenticated'. That is, the client must authenticate itself to get the requested response.", 498, "InvalidToken"),
  ClientClosedRequest => (400, "Bad Request", "The client closed the connection before the server could send a response.", 499, "ClientClosedRequest"),
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::ResponsesClientCodes;
  
  #[test]
    fn test_to_u16() {
        let response = ResponsesClientCodes::BadRequest;
        assert_eq!(response.to_u16(), 400);
    }

    #[test]
    fn test_from_u16() {
        let status = ResponsesClientCodes::from_u16(404);
        assert_eq!(status, Some(ResponsesClientCodes::NotFound));
    }

    #[test]
    fn test_invalid_code_from_u16() {
        let status = ResponsesClientCodes::from_u16(999);
        assert_eq!(status, None);
    }

    #[test]
    fn test_as_tuple() {
        let code = ResponsesClientCodes::Forbidden;
        let http_code = code.to_http_code();
        let tuple = http_code.as_tuple();
        assert_eq!(
            tuple,
            (
                &403,
                &"Forbidden",
                &"The client does not have access rights to the content.",
                &403,
                &"Forbidden"
            )
        );
    }

    #[test]
    fn test_as_json() {
        let code = ResponsesClientCodes::ImATeapot;
        let json_result = code.as_json();
        let expected_json = serde_json::json!({
            "standard http code": {
                "code": 418,
                "name": "I'm a teapot"
            },
            "internal http code": {
                "code": 418,
                "name": "I'm a teapot"
            },
            "description": "The waiter refuses to brew coffee with a teapot, RFC 2324."
        });
        assert_eq!(json_result, expected_json);
    }
}
