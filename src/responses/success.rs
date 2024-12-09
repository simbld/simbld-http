use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
#[repr(u16)]

pub enum ResponsesSuccessCodes {
  #[strum(props(
    Description = "Request processed successfully. Response will depend on the request method used, and the result will be either a representation of the requested resource or an empty response"
  ))]
  Ok = 200,
  #[strum(props(
    Description = "Request processed successfully and document created, with a new resource created, and the URI of the new resource returned, if available"
  ))]
  Created = 201,
  #[strum(props(
    Description = "Request processed, but with no guarantee of results, and no indication of the final status of the request, which will be processed asynchronously, such as a request to create a new resource"
  ))]
  Accepted = 202,
  #[strum(props(
    Description = "Information returned, but generated by an uncertified source, such as a proxy server, rather than the origin server, and may be incorrect, outdated, or otherwise unreliable"
  ))]
  NonAuthoritativeInformation = 203,
  #[strum(props(
    Description = "Request processed successfully but no information to return, and the response body is empty, useful as a header for a DELETE request, indicating that the resource has been deleted"
  ))]
  NoContent = 204,
  #[strum(props(
    Description = "Request processed successfully, current page can be deleted, and the client should reset the document view, useful as a header for a form submission, indicating that the form has been processed successfully"
  ))]
  ResetContent = 205,
  #[strum(props(
    Description = "Only part of the resource was transmitted, as the request used the Range header to retrieve a specific portion of the resource, and the response contains the requested range, or the server is unable to return the entire resource"
  ))]
  PartialContent = 206,
  #[strum(props(
    Description = "Multiple status responses, with a separate response code for each part of the request, and the response body contains XML that describes the status of each part of the request, useful for WebDAV RFC 4918"
  ))]
  MultiStatus = 207,
  #[strum(props(
    Description = "A WebDAV binding has been enumerated in a previous http code 207 and are not included here again, useful for WebDAV RFC 5842"
  ))]
  AlreadyReported = 208,
  #[strum(props(
    Description = "The client-side copy of the resource differs from the server-side copy (content or properties), the content of the response has a different meaning depending on the media type that is returned, and the response body may contain a representation of the requested resource, or some instructions on how to process the request"
  ))]
  ContentDifferent = 210,
  #[strum(props(
    Description = "The response provides a URL for accessing a resource that is the result of the requested action"
  ))]
  ContentLocation = 211,
  #[strum(props(
    Description = "The response contains the representation of an object’s data, and the response body contains the data of the object, such as a JSON object or XML document, and the response body may contain the requested resource"
  ))]
  ObjectData = 212,
  #[strum(props(
    Description = "The response indicates multiple instances of the requested resource exist, each with its own set of properties, and the response body contains an array of resources, each with its own set of properties"
  ))]
  MultipleResourceInstances = 213,
  #[strum(props(
    Description = "The response represents the result of a transformation or conversion applied to the resource, and the response body contains the transformed resource, such as a transcoded media file, or a formatted document"
  ))]
  TransformApplied = 214,
  #[strum(props(
    Description = "The requested resource has been deleted, and the response body contains the status of the deletion, and the response body may contain the requested resource"
  ))]
  ContentDeleted = 215,
  #[strum(props(
    Description = "The server has completed the resource request, responded to a POST request, and the response is a representation of the result of one or more instance manipulations applied to the current instance"
  ))]
  IMUsedPostRequest = 216,
  #[strum(props(
    Description = "The response contains the result of a partial modification to the resource, and the response body contains the modified resource, such as a JSON patch document or a binary diff, the response is a delta encoding of the requested resource, containing only the changes between the current and previous versions"
  ))]
  DeltaEncodingApplied = 217,
  #[strum(props(
    Description = "Everything is fine, and the response body contains a humorous or playful message, indicating that the server is aware of the situation and is not concerned, The server is returning this response to indicate that everything is working as expected, even though the situation may be unusual or unexpected, apache, unofficial"
  ))]
  ThisIsFine = 218,
  #[strum(props(
    Description = "The response contains the transferred content, and the response body contains the content that was transferred, such as a file or document, and the response body may contain the requested resource, the response indicates that the content has been transferred successfully to another instance, thus ending the current instance"
  ))]
  ContentTransferred = 219,
  #[strum(props(
    Description = "The server has started a load balancer, and the response body contains the status of the load balancer, indicating that the server has initiated a load balancer to distribute incoming requests across multiple servers, the server response is sent by a load balancer to notify the client that a new server load balancing process has started"
  ))]
  LoadBalancerStarted = 220,
  #[strum(props(
    Description = "The server has stopped a load balancer, and the response body contains the status of the load balancer, indicating that the server has terminated a load balancer process, the server response is sent by a load balancer to notify the client that the server load balancing process has ended, the server response is sent by a load balancer to notify the client that the server load balancing process has ended"
  ))]
  LoadBalancerEnded = 221,
  #[strum(props(
    Description = "The client authentication was successful, and the response body contains the authentication token or session information, indicating that the client has been successfully authenticated by the server, and the response body may contain the authentication token or session information"
  ))]
  AuthenticationSuccessful = 222,
  #[strum(props(
    Description = "The server has completed the resource request, responded to a GET request, and the response is a representation of the current instance, indicating that the server has completed the resource request and responded to a GET request, and the response body contains the current instance of the resource"
  ))]
  IMUsedGetRequest = 226,
  #[strum(props(
    Description = "The server is running low on storage space, and the response body contains the status of the storage space, indicating that the server is running low on storage space, and the response body may contain the status of the storage space, the server is temporarily unable to store the representation needed to complete the request."
  ))]
  LowOnStorageSpace = 250,
  #[strum(props(
    Description = "The server has recognized the request but cannot process it, and the response body contains the status of the request, indicating that the server has recognized the request but cannot process it, and the response body may contain the status of the request, the server is unable to process the request due to constraints or limitations, the server cannot produce a response that satisfies the range specified in the request’s Range header field"
  ))]
  EntityRecognizedNotProcessable = 252,
  #[strum(props(
    Description = "The resource is locked and cannot be accessed or modified, and the response body contains the status of the resource, indicating that the resource is locked and cannot be accessed or modified, and the response body may contain the status of the resource, the server has locked the resource to prevent access or modification"
  ))]
  ResourceAccessedLocked = 253,
  #[strum(props(
    Description = "The server does not recognize the request method or lacks the capability to fulfill it, and the response body contains the status of the request, indicating that the server does not recognize the request method or lacks the capability to fulfill it, and the response body may contain the status of the request, the server is unable to process the request due to an unsupported method"
  ))]
  MethodNotFound = 254,
  #[strum(props(
    Description = "The server has returned an extended status code, and the response body contains the extended status code, indicating that the server has returned an extended status code, and the response body may contain the extended status code, the server has provided additional information or context in the response"
  ))]
  ExtendedCode = 255,
  #[strum(props(
    Description = "The server has returned a miscellaneous persistent warning, and the response body contains the warning message, indicating that the server has returned a miscellaneous persistent warning, and the response body may contain the warning message, the server has encountered a warning condition that is not covered by other status codes"
  ))]
  MiscellaneousPersistentWarningStart = 299,
}

impl ToU16 for ResponsesSuccessCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesSuccessCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}
