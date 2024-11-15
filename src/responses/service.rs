pub enum ResponsesServiceCodes {
  ReadingError = 611,
  ConnectionError = 612,
  ReadingTimeExpired = 613,
  SSLHandshakeFailed = 614,
  AnotherReadingError = 615,
  FBAAnomaly = 616,
  CodingError = 617,
  RedirectWithoutRedirectURL = 618,
  DNSLookupFailed = 680,
  SyntacticallyIncorrectURL = 690,
  LostConnection = 691,
  WriteTimeout = 692,
  SelectionFailed = 693,
  WriteError = 694,
  IncompleteBlockHeader = 695,
  UnexpectedError = 699,
}

impl ResponsesServiceCodes {
  pub fn to_u16(&self) -> u16 {
    *self as u16
  }

  pub fn description(&self) -> &'static str {
    match self {
      ResponsesServiceCodes::ReadingError => "An error occurred while reading the response or data from the server",
      ResponsesServiceCodes::ConnectionError => "A connection issue occurred, preventing successful communication with the server",
      ResponsesServiceCodes::ReadingTimeExpired => "The reading operation exceeded the allowed time limit, resulting in a timeout",
      ResponsesServiceCodes::SSLHandshakeFailed => "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols",
      ResponsesServiceCodes::AnotherReadingError => "A generic error occurred while reading the response or data",
      ResponsesServiceCodes::FBAAnomaly => "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input",
      ResponsesServiceCodes::CodingError => "An error in the implementation or logic caused the request to fail",
      ResponsesServiceCodes::RedirectWithoutRedirectURL => "The server issued a redirect response but did not provide a valid redirect URL",
      ResponsesServiceCodes::DNSLookupFailed => "The DNS lookup for the specified domain failed, indicating a potential network or configuration issue",
      ResponsesServiceCodes::SyntacticallyIncorrectURL => "The provided URL is syntactically incorrect and cannot be processed",
      ResponsesServiceCodes::LostConnection => "The connection to the server was lost unexpectedly during communication",
      ResponsesServiceCodes::WriteTimeout => "The operation timed out while attempting to write data to the server",
      ResponsesServiceCodes::SelectionFailed => "The requested operation failed during a selection or matching process",
      ResponsesServiceCodes::WriteError => "An error occurred while attempting to write data to the destination",
      ResponsesServiceCodes::IncompleteBlockHeader => "A block header was incomplete or malformed, preventing further processing",
      ResponsesServiceCodes::UnexpectedError => "An unexpected error occurred, often indicative of an unforeseen issue or bug",
    }
  }
}
