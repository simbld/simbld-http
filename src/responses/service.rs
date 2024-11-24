use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{EnumIter, EnumProperty};

#[derive(IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
#[repr(u16)]

pub enum ResponsesServiceCodes {
  #[strum(props(
    Description = "An error occurred while reading the response or data from the server"
  ))]
  ReadingError = 611,
  #[strum(props(
    Description = "A connection issue occurred, preventing successful communication with the server"
  ))]
  ConnectionError = 612,
  #[strum(props(
    Description = "The reading operation exceeded the allowed time limit, resulting in a timeout"
  ))]
  ReadingTimeExpired = 613,
  #[strum(props(
    Description = "The SSL handshake failed, potentially due to invalid certificates or incompatible protocols"
  ))]
  SSLHandshakeFailed = 614,
  #[strum(props(Description = "A generic error occurred while reading the response or data"))]
  AnotherReadingError = 615,
  #[strum(props(
    Description = "An anomaly was detected in the Full Body Analyzer process, likely due to unexpected input"
  ))]
  FBAAnomaly = 616,
  #[strum(props(
    Description = "An error in the implementation or logic caused the request to fail"
  ))]
  CodingError = 617,
  #[strum(props(
    Description = "The server issued a redirect response but did not provide a valid redirect URL"
  ))]
  RedirectWithoutRedirectURL = 618,
  #[strum(props(
    Description = "The DNS lookup for the specified domain failed, indicating a potential network or configuration issue"
  ))]
  DNSLookupFailed = 680,
  #[strum(props(
    Description = "The provided URL is syntactically incorrect and cannot be processed"
  ))]
  SyntacticallyIncorrectURL = 690,
  #[strum(props(
    Description = "The connection to the server was lost unexpectedly during communication"
  ))]
  LostConnection = 691,
  #[strum(props(
    Description = "The operation timed out while attempting to write data to the server"
  ))]
  WriteTimeout = 692,
  #[strum(props(
    Description = "The requested operation failed during a selection or matching process"
  ))]
  SelectionFailed = 693,
  #[strum(props(
    Description = "An error occurred while attempting to write data to the destination"
  ))]
  WriteError = 694,
  #[strum(props(
    Description = "A block header was incomplete or malformed, preventing further processing"
  ))]
  IncompleteBlockHeader = 695,
  #[strum(props(
    Description = "An unexpected error occurred, often indicative of an unforeseen issue or bug"
  ))]
  UnexpectedError = 699,
}

impl ToU16 for ResponsesServiceCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesServiceCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}
