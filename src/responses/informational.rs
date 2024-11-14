pub enum ResponsesInformationalCodes {
  Continue = 100,
  SwitchingProtocols = 101,
  Processing = 102,
  EarlyHints = 103,
  ConnectionResetByPeer = 104,
  NameNotResolved = 105,
  NoResponse = 106,
  ProxyAuthenticationRequired = 107,
  RetryWith = 109,
  ResponseIsStale = 110,
  RevalidationFailed = 111,
}

impl ResponsesInformationalCodes {
  pub fn to_u16(&self) -> u16 {
    *self as u16
  }

  pub fn description(&self) -> &'static str {
    match self {
      ResponsesInformationalCodes::Continue => "The server has received the initial part of the request, the headers, and asks the client to continue, proceed to send the body of the request, a POST request",
      ResponsesInformationalCodes::SwitchingProtocols => "The server is complying with a request to switch protocols, used in WebSocket connections",
      ResponsesInformationalCodes::Processing => "The server is processing the request but no response is available yet, used to prevent timeout errors in asynchronous operations",
      ResponsesInformationalCodes::EarlyHints => "Experimental: The server provides preliminary hints to the client, such as preloading resources while the final response is being prepared",
      ResponsesInformationalCodes::ConnectionResetByPeer => "The connection was forcibly closed by a peer, possibly due to a protocol error, a timeout, or a network issue",
      ResponsesInformationalCodes::NameNotResolved => "The server could not resolve the domain name provided in the request, indicating a DNS lookup failure, The requested hostname cannot be resolved to an IP address",
      ResponsesInformationalCodes::NoResponse => "The server did not provide a response, possibly due to a timeout or a connection issue, The server didn’t send any response within the timeout period.
      This status code is not specified in any RFCs, but it is used in some scenarios to indicate that the server closed the connection without sending any response",
      ResponsesInformationalCodes::ProxyAuthenticationRequired => "The client must authenticate with a proxy server before accessing the requested resource",
      ResponsesInformationalCodes::RetryWith => "Le serveur indique que le client doit réessayer la demande avec les modifications appropriées ou des informations supplémentaires, des informations d'identification nouvelles ou différentes, utiliser un protocole différent ou à un emplacement différent",
      ResponsesInformationalCodes::ResponseIsStale => "The response returned by the server is stale and should be revalidated, indicating that the cached response is outdated or expired",
      ResponsesInformationalCodes::RevalidationFailed => "The server attempted to validate a cached response but failed, indicating the cached response is invalid or expired",
    }
  }
}
