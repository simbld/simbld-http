use crate::helpers::to_u16_helper::ToU16;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{EnumIter, EnumProperty};

#[derive(IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
#[repr(u16)]

pub enum ResponsesCrawlerCodes {
  #[strum(props(Description = "Parsing error: unfinished header"))]
  ParsingErrorUnfinishedHeader = 700,
  #[strum(props(Description = "Parsing error: header"))]
  ParsingErrorHeader = 710,
  #[strum(props(Description = "Parsing error: missing HTTP code"))]
  ParsingErrorMissingHTTPCode = 720,
  #[strum(props(Description = "Parsing error: body"))]
  ParsingErrorBody = 730,
  #[strum(props(Description = "Excluded by robots.txt file"))]
  ExcludedByRobotsTxtFile = 740,
  #[strum(props(Description = "Robots temporarily unavailable"))]
  RobotsTemporarilyUnavailable = 741,
  #[strum(props(Description = "Excluded by definition of exploration space"))]
  ExcludedByDefinitionOfExplorationSpace = 760,
  #[strum(props(Description = "Not allowed by local exploration space"))]
  NotAllowedByLocalExplorationSpace = 761,
  #[strum(props(Description = "Incorrect protocol or non-standard system port"))]
  IncorrectProtocolOrNonStandardSystemPort = 770,
  #[strum(props(Description = "Excluded by file type exclusions"))]
  ExcludedByFileTypeExclusions = 780,
  #[strum(props(Description = "Invalid card - Not a physical card"))]
  InvalidCard = 781,
  #[strum(props(
    Description = "Cannot disable physical card OR Print card request already requested"
  ))]
  CannotDisablePhysicalCard = 782,
  #[strum(props(Description = "Invalid URL"))]
  InvalidURL = 786,
  #[strum(props(Description = "No index meta tag"))]
  NoIndexMetaTag = 2004,
  #[strum(props(Description = "Programmable redirection"))]
  ProgrammableRedirection = 3020,
  #[strum(props(Description = "Redirected to another URL"))]
  RedirectedToAnotherURL = 3021,
}

impl ToU16 for ResponsesCrawlerCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}
