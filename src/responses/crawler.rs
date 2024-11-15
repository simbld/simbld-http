pub enum ResponsesCrawlerCodes {
  ParsingErrorUnfinishedHeader = 700,
  ParsingErrorHeader = 710,
  ParsingErrorMissingHTTPCode = 720,
  ParsingErrorBody = 730,
  ExcludedByRobotsTxtFile = 740,
  RobotsTemporarilyUnavailable = 741,
  ExcludedByDefinitionOfExplorationSpace = 760,
  NotAllowedByLocalExplorationSpace = 761,
  IncorrectProtocolOrNonStandardSystemPort = 770,
  ExcludedByFileTypeExclusions = 780,
  InvalidCard = 781,
  CannotDisablePhysicalCard = 782,
  InvalidURL = 786,
  NoIndexMetaTag = 2004,
  ProgrammableRedirection = 3020,
  RedirectedToAnotherURL = 3021,
}

impl ResponsesCrawlerCodes {
  pub fn to_u16(&self) -> u16 {
    *self as u16
  }

  pub fn description(&self) -> &'static str {
    match self {
      ResponsesCrawlerCodes::ParsingErrorUnfinishedHeader => "Parsing error: unfinished header",
      ResponsesCrawlerCodes::ParsingErrorHeader => "Parsing error: header",
      ResponsesCrawlerCodes::ParsingErrorMissingHTTPCode => "Parsing error: missing HTTP code",
      ResponsesCrawlerCodes::ParsingErrorBody => "Parsing error: body",
      ResponsesCrawlerCodes::ExcludedByRobotsTxtFile => "Excluded by robots.txt file",
      ResponsesCrawlerCodes::RobotsTemporarilyUnavailable => "Robots temporarily unavailable",
      ResponsesCrawlerCodes::ExcludedByDefinitionOfExplorationSpace => {
        "Excluded by definition of exploration space"
      },
      ResponsesCrawlerCodes::NotAllowedByLocalExplorationSpace => {
        "Not allowed by local exploration space"
      },
      ResponsesCrawlerCodes::IncorrectProtocolOrNonStandardSystemPort => {
        "Incorrect protocol or non-standard system port"
      },
      ResponsesCrawlerCodes::ExcludedByFileTypeExclusions => "Excluded by file type exclusions",
      ResponsesCrawlerCodes::InvalidCard => "Invalid card - Not a physical card",
      ResponsesCrawlerCodes::CannotDisablePhysicalCard => {
        "Cannot disable physical card OR Print card request already requested"
      },
      ResponsesCrawlerCodes::InvalidURL => "Invalid URL",
      ResponsesCrawlerCodes::NoIndexMetaTag => "No index meta tag",
      ResponsesCrawlerCodes::ProgrammableRedirection => "Programmable redirection",
      ResponsesCrawlerCodes::RedirectedToAnotherURL => "Redirected to another URL",
    }
  }
}
