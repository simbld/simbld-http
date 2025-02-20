use crate::generate_responses_functions;

generate_responses_functions! {
  "",
  ResponsesSuccessCodes,
  Ok => (200, "OK", "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response", 200, "OK"),
  Created => (201, "Created", "Request processed successfully and document created, with a new resource created, and the URI of the new resource returned, if available", 201, "Created"),
  Accepted => (202, "Accepted", "Request processed, but with no guarantee of results, and no indication of the final status of the request, which will be processed asynchronously, such as a request to create a new resource", 202, "Accepted"),
  NonAuthoritativeInformation => (203, "Non-Authoritative Information", "Information returned, but generated by an uncertified source, such as a proxy server, rather than the origin server, and may be incorrect, outdated, or otherwise unreliable", 203, "NonAuthoritativeInformation"),
  NoContent => (204, "No Content", "Request processed successfully but no information to return, and the response body is empty, useful as a header for a DELETE request, indicating that the resource has been deleted", 204, "NoContent"),
  ResetContent => (205, "Reset Content", "Request processed successfully, current page can be deleted, and the client should reset the document view, useful as a header for a form submission, indicating that the form has been processed successfully", 205, "ResetContent"),
  PartialContent => (206, "Partial Content", "Only part of the resource was transmitted, as the request used the Range header to retrieve a specific portion of the resource, and the response contains the requested range, or the server is unable to return the entire resource", 206, "PartialContent"),
  MultiStatus => (207, "Multi-Status", "Multiple status responses, with a separate response code for each part of the request, and the response body contains XML that describes the status of each part of the request, useful for WebDAV RFC 4918", 207, "Multi-Status"),
  AlreadyReported => (208, "Already Reported", "A WebDAV binding has been enumerated in a previous http code 207 and are not included here again, useful for WebDAV RFC 5842", 208, "Already Reported"),
  IMUsedGetRequest => (226, "IM Used", "The server has completed the resource request, responded to a GET request, and the response is a representation of the current instance, indicating that the server has completed the resource request and responded to a GET request, and the response body contains the current instance of the resource", 226, "IM Used Get Request"),
  ContentDifferent => (200, "OK", "The client-side copy of the resource differs from the server-side copy (content or properties), the content of the response has a different meaning depending on the media type that is returned, and the response body may contain a representation of the requested resource, or some instructions on how to process the request", 210, "Content Different"),
  ContentLocation => (200, "OK", "The response provides a URL for accessing a resource that is the result of the requested action", 211, "Content Location"),
  ObjectData => (200, "OK", "The response contains the representation of an object’s data, and the response body contains the data of the object, such as a JSON object or XML document, and the response body may contain the requested resource", 212, "Object Data"),
  MultipleResourceInstances => (200, "OK", "The response indicates multiple instances of the requested resource exist, each with its own set of properties, and the response body contains an array of resources, each with its own set of properties", 213, "Multiple Resource Instances"),
  TransformApplied => (200, "OK", "The response represents the result of a transformation or conversion applied to the resource, and the response body contains the transformed resource, such as a transcode media file, or a formatted document", 214, "Transform Applied"),
  ContentDeleted => (200, "OK", "The requested resource has been deleted, and the response body contains the status of the deletion, and the response body may contain the requested resource", 215, "Content Deleted"),
  IMUsedPostRequest => (200, "OK", "The server has completed the resource request, responded to a POST request, and the response is a representation of the result of one or more instance manipulations applied to the current instance", 216, "IM Used Post Request"),
  DeltaEncodingApplied => (200, "OK", "The response contains the result of a partial modification to the resource, and the response body contains the modified resource, such as a JSON patch document or a binary diff, the response is a delta encoding of the requested resource, containing only the changes between the current and previous versions", 217, "Delta Encoding Applied"),
  ThisIsFine => (200, "OK", "Everything is fine, and the response body contains a humorous or playful message, indicating that the server is aware of the situation and is not concerned, The server is returning this response to indicate that everything is working as expected, even though the situation may be unusual or unexpected, apache, unofficial", 218, "This Is Fine"),
  ContentTransferred => (200, "OK", "The response contains the transferred content, and the response body contains the content that was transferred, such as a file or document, and the response body may contain the requested resource, the response indicates that the content has been transferred successfully to another instance, thus ending the current instance", 219, "Content Transferred"),
  LoadBalancerStarted => (200, "OK", "The server has started a load balancer, and the response body contains the status of the load balancer, indicating that the server has initiated a load balancer to distribute incoming requests across multiple servers, the server response is sent by a load balancer to notify the client that a new server load balancing process has started", 220, "Load Balancer Started"),
  LoadBalancerEnded => (200, "OK", "The server has stopped a load balancer, and the response body contains the status of the load balancer, indicating that the server has terminated a load balancer process, the server response is sent by a load balancer to notify the client that the server load balancing process has ended, the server response is sent by a load balancer to notify the client that the server load balancing process has ended", 221, "Load Balancer Ended"),
  AuthenticationSuccessful => (200, "OK", "The client authentication was successful, and the response body contains the authentication token or session information, indicating that the client has been successfully authenticated by the server, and the response body may contain the authentication token or session information", 222, "Authentication Successful"),
  LowOnStorageSpace => (200, "OK", "The server is running low on storage space, and the response body contains the status of the storage space, indicating that the server is running low on storage space, and the response body may contain the status of the storage space, the server is temporarily unable to store the representation needed to complete the request.", 250, "Low On Storage Space"),
  EntityRecognizedNotProcessable => (200, "OK", "The server has recognized the request but cannot process it, and the response body contains the status of the request, indicating that the server has recognized the request but cannot process it, and the response body may contain the status of the request, the server is unable to process the request due to constraints or limitations, the server cannot produce a response that satisfies the range specified in the request’s Range header field", 252, "Entity Recognized Not Processable"),
  ResourceAccessedLocked => (200, "OK", "The resource is locked and cannot be accessed or modified, and the response body contains the status of the resource, indicating that the resource is locked and cannot be accessed or modified, and the response body may contain the status of the resource, the server has locked the resource to prevent access or modification", 253, "Resource Accessed Locked"),
  MethodNotFound => (200, "OK", "The server does not recognize the request method or lacks the capability to fulfill it, and the response body contains the status of the request, indicating that the server does not recognize the request method or lacks the capability to fulfill it, and the response body may contain the status of the request, the server is unable to process the request due to an unsupported method", 254, "Method Not Found"),
  ExtendedCode => (200, "OK", "The server has returned an extended status code, and the response body contains the extended status code, indicating that the server has returned an extended status code, and the response body may contain the extended status code, the server has provided additional information or context in the response", 255, "Extended Code"),
  MiscellaneousPersistentWarningStart => (200, "OK", "The server has returned a miscellaneous persistent warning, and the response body contains the warning message, indicating that the server has returned a miscellaneous persistent warning, and the response body may contain the warning message, the server has encountered a warning condition that is not covered by other status codes", 299, "Miscellaneous Persistent Warning Start"),
}

#[cfg(test)]
mod tests {
    use crate::helpers::unified_tuple_helper::UnifiedTuple;
    use crate::responses::ResponsesSuccessCodes;
    use serde_json::json;
    
    #[test]
    fn test_success_codes_to_u16() {
        assert_eq!(ResponsesSuccessCodes::Ok.to_u16(), 200);
        assert_eq!(ResponsesSuccessCodes::Created.to_u16(), 201);
        assert_eq!(ResponsesSuccessCodes::Accepted.to_u16(), 202);
        assert_eq!(ResponsesSuccessCodes::NonAuthoritativeInformation.to_u16(), 203);
    }

    #[test]
    fn test_success_codes_from_u16() {
        assert_eq!(ResponsesSuccessCodes::from_u16(201), Some(ResponsesSuccessCodes::Created));
        assert_eq!(ResponsesSuccessCodes::from_u16(202), Some(ResponsesSuccessCodes::Accepted));
        assert_eq!(
            ResponsesSuccessCodes::from_u16(203),
            Some(ResponsesSuccessCodes::NonAuthoritativeInformation)
        );
        assert_eq!(ResponsesSuccessCodes::from_u16(9999), None);
    }

    #[test]
    fn test_load_balancer_started_codes_as_tuple() {
        let code = ResponsesSuccessCodes::LoadBalancerStarted;
        let tuple = UnifiedTuple {
            standard_code: 200,
            standard_name: "OK",
            unified_description: "The server has started a load balancer, and the response body contains the status of the load balancer, indicating that the server has initiated a load balancer to distribute incoming requests across multiple servers, the server response is sent by a load balancer to notify the client that a new server load balancing process has started",
            internal_code: Some(220),
            internal_name: Option::from("Load Balancer Started"),
        };
        let code_as_tuple = code.as_tuple();
        assert_eq!(code_as_tuple, tuple);
    }

    #[test]
    fn test_content_transferred_codes_as_json() {
        let response_code = ResponsesSuccessCodes::ContentTransferred;
        let json_result = response_code.as_json();
        let expected_json = json!({
            "standard_http_code": {
                "code": 200,
                "name": "OK"
            },
            "internal_http_code": {
                "code": 219,
                "name": "Content Transferred"
            },
            "description": "The response contains the transferred content, and the response body contains the content that was transferred, such as a file or document, and the response body may contain the requested resource, the response indicates that the content has been transferred successfully to another instance, thus ending the current instance"
        });
        assert_eq!(
            serde_json::to_string(&json_result).unwrap(),
            serde_json::to_string(&expected_json).unwrap()
        );
    }

    #[test]
    fn test_success_codes_into_tuple() {
        let (std_code, std_name): (u16, &'static str) = ResponsesSuccessCodes::Created.into();
        assert_eq!(std_code, 201);
        assert_eq!(std_name, "Created");
    }

    #[test]
    fn test_bad_request_duplicate_standard_codes() {
        // These two codes have the same standard HTTP code (400) but different internal codes
        assert_eq!(
            ResponsesSuccessCodes::from_u16(254),
            Some(ResponsesSuccessCodes::MethodNotFound)
        );
        assert_eq!(ResponsesSuccessCodes::from_u16(255), Some(ResponsesSuccessCodes::ExtendedCode));
    }
}
