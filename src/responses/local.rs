use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use strum_macros::{EnumIter, EnumProperty, Display};

#[derive(Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone)]
#[repr(u16)]

pub enum ResponsesLocalApiCodes {
  #[strum(props(Description = "The operation was approved and no further action is needed"))]
  ApprovedNoActionRequired = 900,
  #[strum(props(Description = "The operation was successfully approved"))]
  Approved = 901,
  #[strum(props(
    Description = "The transaction ID is a duplicate; the transaction has already been processed"
  ))]
  DuplicatedTransactionId = 902,
  #[strum(props(
    Description = "Validation errors occurred. Please verify the provided values and try again"
  ))]
  ValidationErrorsProvided = 903,
  #[strum(props(Description = "The requested operation is not permitted"))]
  OperationNotAllowed = 904,
  #[strum(props(Description = "The requested operation is not supported by the system"))]
  OperationNotSupported = 905,
  #[strum(props(
    Description = "The transaction could not be completed due to a timeout (e.g., an authorization expired before capture)"
  ))]
  TransactionTimeout = 906,
  #[strum(props(Description = "Authentication failed due to incorrect or missing credentials"))]
  AuthentificationFailed = 907,
  #[strum(props(
    Description = "General decline with no specific reason provided, or insufficient funds"
  ))]
  DoNotHonor = 908,
  #[strum(props(
    Description = "The account does not have enough funds to complete the transaction"
  ))]
  InsufficientFunds = 909,
  #[strum(props(Description = "The provided PIN is incorrect"))]
  IncorrectPIN = 910,
  #[strum(props(Description = "The transaction request is invalid or unsupported"))]
  InvalidTransaction = 911,
  #[strum(props(Description = "The specified amount is invalid"))]
  InvalidAmount = 912,
  #[strum(props(
    Description = "The card number (PAN) is invalid or the card type is not accepted"
  ))]
  InvalidCardNumber = 913,
  #[strum(props(Description = "The provided CVV code is invalid"))]
  InvalidCVV = 914,
  #[strum(props(Description = "The expiration date (MMYY) is invalid or in the past"))]
  InvalidCardHolderName = 915,
  #[strum(props(Description = "The cardholder's last name is invalid"))]
  InvalidCardHolderLastName = 916,
  #[strum(props(Description = "The cardholder's first name is invalid"))]
  InvalidCardHolderFirstName = 917,
  #[strum(props(Description = "The cardholder's ID number is invalid"))]
  InvalidCardHolderIdNumber = 918,
  #[strum(props(Description = "The cardholder's phone number is invalid"))]
  InvalidCardHolderPhoneNumber = 919,
  #[strum(props(Description = "The card is already active and cannot be reactivated"))]
  CardAlreadyActive = 920,
  #[strum(props(Description = "The card is not active or cannot be found"))]
  CardNotActive = 921,
  #[strum(props(Description = "The card has expired and cannot be used"))]
  ExpiredCard = 922,
  #[strum(props(Description = "The card has been reported lost and cannot be used"))]
  LostCard = 923,
  #[strum(props(Description = "The card has been reported stolen and cannot be used"))]
  StolenCard = 924,
  #[strum(props(Description = "The last name provided is invalid"))]
  InvalidLastName = 925,
  #[strum(props(Description = "The first name provided is invalid"))]
  InvalidFirstName = 926,
  #[strum(props(Description = "The ID number provided is invalid"))]
  InvalidIdNumber = 927,
  #[strum(props(Description = "The phone number provided is invalid"))]
  InvalidPhoneNumber = 928,
  #[strum(props(Description = "The email address provided is invalid"))]
  InvalidEmail = 929,
  #[strum(props(Description = "The initials provided are invalid"))]
  InvalidInitials = 930,
  #[strum(props(Description = "The address provided is invalid"))]
  InvalidAddress = 931,
  #[strum(props(Description = "The city provided is invalid"))]
  InvalidCity = 932,
  #[strum(props(Description = "The postal code provided is invalid"))]
  InvalidPostalCode = 933,
  #[strum(props(Description = "The country provided is invalid"))]
  InvalidCountry = 934,
  #[strum(props(Description = "The password provided is invalid"))]
  InvalidPassword = 935,
  #[strum(props(Description = "The username provided is invalid"))]
  InvalidUsername = 936,
  #[strum(props(Description = "The role specified is invalid"))]
  InvalidRole = 937,
  #[strum(props(Description = "The status specified is invalid"))]
  InvalidStatus = 938,
  #[strum(props(Description = "The date of birth provided is invalid"))]
  InvalidDateOfBirth = 939,
  #[strum(props(Description = "The majority information provided is invalid"))]
  InvalidMajority = 940,
  #[strum(props(Description = "The marital status provided is invalid"))]
  InvalidMaritalStatus = 941,
  #[strum(props(Description = "The nationality provided is invalid"))]
  InvalidNationality = 942,
  #[strum(props(Description = "The language provided is invalid"))]
  InvalidLanguage = 943,
  #[strum(props(Description = "The currency provided is invalid"))]
  InvalidCurrency = 944,
  #[strum(props(Description = "The time zone specified is invalid"))]
  InvalidTimeZone = 945,
  #[strum(props(Description = "The profile picture is invalid or unsupported"))]
  InvalidProfilePicture = 946,
  #[strum(props(Description = "The cover picture is invalid or unsupported"))]
  InvalidCoverPicture = 947,
  #[strum(props(Description = "The bio provided is invalid"))]
  InvalidBio = 948,
  #[strum(props(Description = "The website URL provided is invalid"))]
  InvalidWebsite = 949,
  #[strum(props(Description = "The Facebook profile URL provided is invalid"))]
  InvalidFacebook = 950,
  #[strum(props(Description = "The Twitter handle provided is invalid"))]
  InvalidTwitter = 951,
  #[strum(props(Description = "The Instagram profile provided is invalid"))]
  InvalidInstagram = 952,
  #[strum(props(Description = "The LinkedIn profile provided is invalid"))]
  InvalidLinkedIn = 953,
  #[strum(props(Description = "The GitHub profile provided is invalid"))]
  InvalidGitHub = 954,
  #[strum(props(Description = "The GitLab profile provided is invalid"))]
  InvalidGitLab = 955,
  #[strum(props(Description = "The Bitbucket profile provided is invalid"))]
  InvalidBitbucket = 956,
  #[strum(props(Description = "The Google profile provided is invalid"))]
  InvalidGoogle = 957,
  #[strum(props(Description = "The YouTube profile provided is invalid"))]
  InvalidYouTube = 958,
  #[strum(props(Description = "The Twitch profile provided is invalid"))]
  InvalidTwitch = 959,
  #[strum(props(Description = "The Discord handle provided is invalid"))]
  InvalidDiscord = 960,
  #[strum(props(Description = "The Slack workspace or handle provided is invalid"))]
  InvalidSlack = 961,
  #[strum(props(Description = "The Telegram handle provided is invalid"))]
  InvalidTelegram = 962,
  #[strum(props(Description = "The WhatsApp number provided is invalid"))]
  InvalidWhatsApp = 963,
  #[strum(props(Description = "The Skype ID provided is invalid"))]
  InvalidSkype = 964,
  #[strum(props(Description = "The Snapchat handle provided is invalid"))]
  InvalidSnapchat = 965,
  #[strum(props(Description = "The Pinterest profile provided is invalid"))]
  InvalidPinterest = 966,
  #[strum(props(Description = "The Tumblr profile provided is invalid"))]
  InvalidTumblr = 967,
  #[strum(props(Description = "The Flickr profile provided is invalid"))]
  InvalidFlickr = 968,
  #[strum(props(Description = "The Vimeo profile provided is invalid"))]
  InvalidVimeo = 969,
  #[strum(props(Description = "The SoundCloud profile provided is invalid"))]
  InvalidSoundCloud = 970,
  #[strum(props(Description = "The Spotify profile provided is invalid"))]
  InvalidSpotify = 971,
  #[strum(props(Description = "The TikTok profile provided is invalid"))]
  InvalidTikTok = 972,
  #[strum(props(Description = "The Vine profile provided is invalid"))]
  InvalidVine = 973,
  #[strum(props(Description = "The Periscope profile provided is invalid"))]
  InvalidPeriscope = 974,
  #[strum(props(Description = "The expiration date (MMYY) is invalid or in the past"))]
  InvalidExpirationDate = 975,
  #[strum(props(Description = "The session key is missing from the request header"))]
  SessionKeyNotPresentInHeader = 976,
  #[strum(props(Description = "The session key provided is invalid, corrupted, or unparsable"))]
  SessionKeyPresentAndNotDecryptableParsable = 977,
  #[strum(props(Description = "The reference provided does not have any linked cards"))]
  ReferenceHasNoLinkedCards = 978,
  #[strum(props(
    Description = "The card is already linked to a different reference and cannot be re-linked"
  ))]
  CardAlreadyLinkedToADifferentReference = 979,
  #[strum(props(Description = "The uploaded file type is not allowed"))]
  ExcludedByFileTypeExclusions = 980,
  #[strum(props(Description = "The card information provided is invalid"))]
  InvalidCard = 981,
  #[strum(props(Description = "The operation to disable a physical card is not allowed"))]
  CannotDisablePhysicalCard = 982,
  #[strum(props(
    Description = "Unofficial HTTP status code LinkedIn that is returned by the server as a generic, or catch-all error code. The reason for the HTTP response varies based on the service or host"
  ))]
  RequestDenied = 999,
}

impl ToU16 for ResponsesLocalApiCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

impl FromU16 for ResponsesLocalApiCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}
