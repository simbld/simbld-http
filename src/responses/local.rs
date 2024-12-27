/// The code defines an enum representing local API response codes with associated descriptions and provides functions to retrieve code-description pairs.
use crate::helpers::{from_u16_helper::FromU16, to_u16_helper::ToU16};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde_json::json;
use strum::EnumProperty;
use strum_macros::{Display, EnumIter, EnumProperty};

#[derive(
  Display, IntoPrimitive, TryFromPrimitive, EnumProperty, EnumIter, Debug, Copy, Clone, PartialEq,
)]
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
  #[strum(props(Description = "The Facebook profile name provided is invalid"))]
  InvalidFacebook = 950,
  #[strum(props(Description = "The Twitter profile name provided is invalid"))]
  InvalidTwitter = 951,
  #[strum(props(Description = "The Instagram profile name provided is invalid"))]
  InvalidInstagram = 952,
  #[strum(props(Description = "The LinkedIn profile name provided is invalid"))]
  InvalidLinkedin = 953,
  #[strum(props(Description = "The GitHub profile name provided is invalid"))]
  InvalidGithub = 954,
  #[strum(props(Description = "The GitLab profile name provided is invalid"))]
  InvalidGitlab = 955,
  #[strum(props(Description = "The Bitbucket profile name provided is invalid"))]
  InvalidBitbucket = 956,
  #[strum(props(Description = "The Google profile name provided is invalid"))]
  InvalidGoogle = 957,
  #[strum(props(Description = "The YouTube profile name provided is invalid"))]
  InvalidYoutube = 958,
  #[strum(props(Description = "The Twitch profile name provided is invalid"))]
  InvalidTwitch = 959,
  #[strum(props(Description = "The Discord profile name provided is invalid"))]
  InvalidDiscord = 960,
  #[strum(props(Description = "The Slack profile name provided is invalid"))]
  InvalidSlack = 961,
  #[strum(props(Description = "The Telegram profile name provided is invalid"))]
  InvalidTelegram = 962,
  #[strum(props(Description = "The WhatsApp profile name provided is invalid"))]
  InvalidWhatsapp = 963,
  #[strum(props(Description = "The Skype profile name or ID provided is invalid"))]
  InvalidSkype = 964,
  #[strum(props(Description = "The Snapchat profile name provided is invalid"))]
  InvalidSnapchat = 965,
  #[strum(props(Description = "The Pinterest profile name provided is invalid"))]
  InvalidPinterest = 966,
  #[strum(props(Description = "The Tumblr profile name provided is invalid"))]
  InvalidTumblr = 967,
  #[strum(props(Description = "The Flickr profile name provided is invalid"))]
  InvalidFlickr = 968,
  #[strum(props(Description = "The Vimeo profile name provided is invalid"))]
  InvalidVimeo = 969,
  #[strum(props(Description = "The SoundCloud profile name provided is invalid"))]
  InvalidSoundcloud = 970,
  #[strum(props(Description = "The Spotify profile name provided is invalid"))]
  InvalidSpotify = 971,
  #[strum(props(Description = "The TikTok profile name provided is invalid"))]
  InvalidTiktok = 972,
  #[strum(props(Description = "The Vine profile name provided is invalid"))]
  InvalidVine = 973,
  #[strum(props(Description = "The reddit profile provided is invalid"))]
  InvalidReddit = 974,
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
  InvalidCardInformation = 981,
  #[strum(props(Description = "The operation to disable a physical card is not allowed"))]
  CannotDisablePhysicalCard = 982,
  #[strum(props(Description = "The token is missing from the request"))]
  MissingToken = 983,
  #[strum(props(Description = "User not found or does not exist in the system"))]
  UserNotFound = 984,
  #[strum(props(Description = "User already exists in the system"))]
  AlreadyExists = 985,
  #[strum(props(
    Description = "Database error occurred, please try again later or contact support"
  ))]
  DatabaseError = 986,
  #[strum(props(
    Description = "Password hashing error occurred, please try again later or contact support"
  ))]
  HashingError = 987,
  #[strum(props(Description = "The login has an error or does not exist"))]
  InvalidLogin = 988,
  #[strum(props(Description = "The user has an error or does not exist"))]
  InvalidUser = 989,
  #[strum(props(Description = "The user ID has an error or does not exist"))]
  InvalidUserId = 990,
  #[strum(props(Description = "The user role has an error or does not exist"))]
  InvalidUserRole = 991,
  #[strum(props(Description = "The credentials has an error or does not exist"))]
  InvalidCredentials = 992,
  #[strum(props(Description = "The google meet has an error or does not exist"))]
  InvalidGoogleMeet = 993,
  #[strum(props(Description = "The pseudonym has an error or does not exist"))]
  InvalidPseudonym = 994,
  #[strum(props(Description = "The tag has an error or does not exist"))]
  InvalidTag = 995,
  #[strum(props(Description = "The authorization code has an error or does not exist"))]
  InvalidAuthorizationCode = 996,
  #[strum(props(
    Description = "Unofficial HTTP  catch-all error code. The reason for the HTTP response varies based on the service or host"
  ))]
  RequestDenied = 999,
}

/// implementation of a custom trait `ToU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “to_u16” method which converts a value from the enumeration into a “u16” type. Self accesses the value of the enum In the implementation, it calls the `into()` method to perform the conversion, which relies on the `Into<u16>` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl ToU16 for ResponsesLocalApiCodes {
  fn to_u16(self) -> u16 {
    self.into() // Conversion`Into<u16>`
  }
}

/// implementation of a custom trait `FromU16` for the `ResponsesLocalApiCodes` enumeration. We provide a “from_u16” method which converts a value from the enumeration into an `Option<Self>` type. The method uses the `try_from` method to perform the conversion, which relies on the `TryFromPrimitive` trait implemented for enum variants. The conversion is possible thanks to the IntoPrimitive derivative in the enum
impl FromU16 for ResponsesLocalApiCodes {
  fn from_u16(code: u16) -> Option<Self> {
    Self::try_from(code).ok() // Conversion`TryFrom<u16>`
  }
}

/// implementation of a custom trait `Into` for the `ResponsesLocalApiCodes` enumeration. We provide an “into” method which converts a value from the enumeration into a tuple containing a `u16` and a `&'static str`. The method calls the `to_u16` method to get the status code and the `get_str` method to get the description. The `unwrap_or` method is used to provide a default value in case the description is not found. The method returns the tuple containing the status code and the description
impl Into<(u16, &'static str)> for ResponsesLocalApiCodes {
  fn into(self) -> (u16, &'static str) {
    let code: u16 = self.to_u16();
    let description = self.get_str("Description").unwrap_or("No description");
    (code, description)
  }
}

/// Functions return raw data as a tuple for further processing or formats containing HTTP status code, status message and description of various client error responses.
pub fn approved_no_action_required_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::ApprovedNoActionRequired;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Approved No Action Required", description)
}

pub fn approved_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::Approved;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Approved", description)
}

pub fn duplicated_transaction_id_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::DuplicatedTransactionId;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Duplicated Transaction ID", description)
}

pub fn validation_errors_provided_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::ValidationErrorsProvided;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Validation Errors Provided", description)
}

pub fn operation_not_allowed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::OperationNotAllowed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Operation Not Allowed", description)
}

pub fn operation_not_supported_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::OperationNotSupported;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Operation Not Supported", description)
}

pub fn transaction_timeout_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::TransactionTimeout;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Transaction Timeout", description)
}

pub fn authentification_failed_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::AuthentificationFailed;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Authentification Failed", description)
}

pub fn do_not_honor_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::DoNotHonor;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Do Not Honor", description)
}

pub fn insufficient_funds_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InsufficientFunds;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Insufficient Funds", description)
}

pub fn incorrect_pin_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::IncorrectPIN;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Incorrect PIN", description)
}

pub fn invalid_transaction_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTransaction;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Transaction", description)
}

pub fn invalid_amount_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidAmount;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Amount", description)
}

pub fn invalid_card_number_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCardNumber;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card Number", description)
}

pub fn invalid_cvv_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCVV;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid CVV", description)
}

pub fn invalid_card_holder_name_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCardHolderName;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card Holder Name", description)
}

pub fn invalid_card_holder_last_name_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCardHolderLastName;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card Holder Last Name", description)
}

pub fn invalid_card_holder_first_name_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCardHolderFirstName;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card Holder First Name", description)
}

pub fn invalid_card_holder_id_number_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCardHolderIdNumber;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card Holder ID Number", description)
}

pub fn invalid_card_holder_phone_number_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCardHolderPhoneNumber;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card Holder Phone Number", description)
}

pub fn card_already_active_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::CardAlreadyActive;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Card Already Active", description)
}

pub fn card_not_active_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::CardNotActive;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Card Not Active", description)
}

pub fn expired_card_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::ExpiredCard;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Expired Card", description)
}

pub fn lost_card_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::LostCard;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Lost Card", description)
}

pub fn stolen_card_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::StolenCard;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Stolen Card", description)
}

pub fn invalid_last_name_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidLastName;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Last Name", description)
}

pub fn invalid_first_name_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidFirstName;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid First Name", description)
}

pub fn invalid_id_number_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidIdNumber;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid ID Number", description)
}

pub fn invalid_phone_number_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidPhoneNumber;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Phone Number", description)
}

pub fn invalid_email_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidEmail;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Email", description)
}

pub fn invalid_initials_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidInitials;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Initials", description)
}

pub fn invalid_address_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidAddress;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Address", description)
}

pub fn invalid_city_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCity;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid City", description)
}

pub fn invalid_postal_code_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidPostalCode;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Postal Code", description)
}

pub fn invalid_country_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCountry;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Country", description)
}

pub fn invalid_password_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidPassword;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Password", description)
}

pub fn invalid_username_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidUsername;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Username", description)
}

pub fn invalid_role_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidRole;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Role", description)
}

pub fn invalid_status_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidStatus;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Status", description)
}

pub fn invalid_date_of_birth_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidDateOfBirth;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Date of Birth", description)
}

pub fn invalid_majority_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidMajority;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Majority", description)
}

pub fn invalid_marital_status_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidMaritalStatus;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Marital Status", description)
}

pub fn invalid_nationality_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidNationality;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Nationality", description)
}

pub fn invalid_language_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidLanguage;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Language", description)
}

pub fn invalid_currency_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCurrency;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Currency", description)
}

pub fn invalid_time_zone_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTimeZone;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Time Zone", description)
}

pub fn invalid_profile_picture_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidProfilePicture;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Profile Picture", description)
}

pub fn invalid_cover_picture_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCoverPicture;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Cover Picture", description)
}

pub fn invalid_bio_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidBio;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Bio", description)
}

pub fn invalid_website_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidWebsite;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Website", description)
}

pub fn invalid_facebook_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidFacebook;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Facebook", description)
}

pub fn invalid_twitter_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTwitter;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Twitter", description)
}

pub fn invalid_instagram_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidInstagram;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Instagram", description)
}

pub fn invalid_linkedin_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidLinkedin;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid LinkedIn", description)
}

pub fn invalid_github_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidGithub;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid GitHub", description)
}

pub fn invalid_gitlab_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidGitlab;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid GitLab", description)
}

pub fn invalid_bitbucket_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidBitbucket;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Bitbucket", description)
}

pub fn invalid_google_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidGoogle;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Google", description)
}

pub fn invalid_youtube_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidYoutube;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid YouTube", description)
}

pub fn invalid_twitch_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTwitch;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Twitch", description)
}

pub fn invalid_discord_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidDiscord;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Discord", description)
}

pub fn invalid_slack_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidSlack;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Slack", description)
}

pub fn invalid_telegram_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTelegram;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Telegram", description)
}

pub fn invalid_whatsapp_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidWhatsapp;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid WhatsApp", description)
}

pub fn invalid_skype_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidSkype;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Skype", description)
}

pub fn invalid_snapchat_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidSnapchat;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Snapchat", description)
}

pub fn invalid_pinterest_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidPinterest;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Pinterest", description)
}

pub fn invalid_tumblr_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTumblr;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Tumblr", description)
}

pub fn invalid_flickr_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidFlickr;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Flickr", description)
}

pub fn invalid_vimeo_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidVimeo;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Vimeo", description)
}

pub fn invalid_soundcloud_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidSoundcloud;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid SoundCloud", description)
}

pub fn invalid_spotify_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidSpotify;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Spotify", description)
}

pub fn invalid_tiktok_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTiktok;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid TikTok", description)
}

pub fn invalid_vine_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidVine;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Vine", description)
}

pub fn invalid_reddit_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidReddit;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Reddit", description)
}

pub fn invalid_expiration_date_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidExpirationDate;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Expiration Date", description)
}

pub fn session_key_not_present_in_header_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::SessionKeyNotPresentInHeader;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Session Key Not Present In Header", description)
}

pub fn session_key_present_and_not_decryptable_parsable_tuple() -> (u16, &'static str, &'static str)
{
  let code = ResponsesLocalApiCodes::SessionKeyPresentAndNotDecryptableParsable;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Session Key Present And Not Decryptable Parsable", description)
}

pub fn reference_has_no_linked_cards_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::ReferenceHasNoLinkedCards;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Reference Has No Linked Cards", description)
}

pub fn card_already_linked_to_a_different_reference_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::CardAlreadyLinkedToADifferentReference;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Card Already Linked To A Different Reference", description)
}

pub fn excluded_by_file_type_exclusions_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::ExcludedByFileTypeExclusions;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Excluded By File Type Exclusions", description)
}

pub fn invalid_card_information_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCardInformation;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Card Information", description)
}

pub fn cannot_disable_physical_card_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::CannotDisablePhysicalCard;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Cannot Disable Physical Card", description)
}

pub fn missing_token_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::MissingToken;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Missing Token", description)
}

pub fn user_not_found_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::UserNotFound;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "User Not Found", description)
}

pub fn already_exists_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::AlreadyExists;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Already Exists", description)
}

pub fn database_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::DatabaseError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Database Error", description)
}

pub fn hashing_error_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::HashingError;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Hashing Error", description)
}

pub fn invalid_login_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidLogin;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Login", description)
}

pub fn invalid_user_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidUser;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid User", description)
}

pub fn invalid_user_id_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidUserId;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid User ID", description)
}

pub fn invalid_user_role_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidUserRole;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid User Role", description)
}

pub fn invalid_credentials_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidCredentials;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Credentials", description)
}

pub fn invalid_google_meet_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidGoogleMeet;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Google Meet", description)
}

pub fn invalid_pseudonym_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidPseudonym;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Pseudonym", description)
}

pub fn invalid_tag_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidTag;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Tag", description)
}

pub fn invalid_authorization_code_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::InvalidAuthorizationCode;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Invalid Authorization Code", description)
}

pub fn request_denied_tuple() -> (u16, &'static str, &'static str) {
  let code = ResponsesLocalApiCodes::RequestDenied;
  let description = code.get_str("Description").unwrap_or("No description");
  (code.to_u16(), "Request Denied", description)
}

/// Functions return formatted data as JSON containing HTTP status code, status message and description of various informational responses.
pub fn approved_no_action_required() -> serde_json::Value {
  let (code, name, desc) = approved_no_action_required_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn approved() -> serde_json::Value {
  let (code, name, desc) = approved_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn duplicated_transaction_id() -> serde_json::Value {
  let (code, name, desc) = duplicated_transaction_id_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn validation_errors_provided() -> serde_json::Value {
  let (code, name, desc) = validation_errors_provided_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn operation_not_allowed() -> serde_json::Value {
  let (code, name, desc) = operation_not_allowed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn operation_not_supported() -> serde_json::Value {
  let (code, name, desc) = operation_not_supported_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn transaction_timeout() -> serde_json::Value {
  let (code, name, desc) = transaction_timeout_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn authentification_failed() -> serde_json::Value {
  let (code, name, desc) = authentification_failed_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn do_not_honor() -> serde_json::Value {
  let (code, name, desc) = do_not_honor_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn insufficient_funds() -> serde_json::Value {
  let (code, name, desc) = insufficient_funds_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn incorrect_pin() -> serde_json::Value {
  let (code, name, desc) = incorrect_pin_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_transaction() -> serde_json::Value {
  let (code, name, desc) = invalid_transaction_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_amount() -> serde_json::Value {
  let (code, name, desc) = invalid_amount_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_card_number() -> serde_json::Value {
  let (code, name, desc) = invalid_card_number_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_cvv() -> serde_json::Value {
  let (code, name, desc) = invalid_cvv_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_card_holder_name() -> serde_json::Value {
  let (code, name, desc) = invalid_card_holder_name_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_card_holder_last_name() -> serde_json::Value {
  let (code, name, desc) = invalid_card_holder_last_name_tuple();
  json!({ "status": code, "name": name, "description": desc })
}
pub fn invalid_card_holder_first_name() -> serde_json::Value {
  let (code, name, desc) = invalid_card_holder_first_name_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_card_holder_id_number() -> serde_json::Value {
  let (code, name, desc) = invalid_card_holder_id_number_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_card_holder_phone_number() -> serde_json::Value {
  let (code, name, desc) = invalid_card_holder_phone_number_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn card_already_active() -> serde_json::Value {
  let (code, name, desc) = card_already_active_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn card_not_active() -> serde_json::Value {
  let (code, name, desc) = card_not_active_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn expired_card() -> serde_json::Value {
  let (code, name, desc) = expired_card_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn lost_card() -> serde_json::Value {
  let (code, name, desc) = lost_card_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn stolen_card() -> serde_json::Value {
  let (code, name, desc) = stolen_card_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_last_name() -> serde_json::Value {
  let (code, name, desc) = invalid_last_name_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_first_name() -> serde_json::Value {
  let (code, name, desc) = invalid_first_name_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_id_number() -> serde_json::Value {
  let (code, name, desc) = invalid_id_number_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_phone_number() -> serde_json::Value {
  let (code, name, desc) = invalid_phone_number_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_email() -> serde_json::Value {
  let (code, name, desc) = invalid_email_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_initials() -> serde_json::Value {
  let (code, name, desc) = invalid_initials_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_address() -> serde_json::Value {
  let (code, name, desc) = invalid_address_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_city() -> serde_json::Value {
  let (code, name, desc) = invalid_city_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_postal_code() -> serde_json::Value {
  let (code, name, desc) = invalid_postal_code_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_country() -> serde_json::Value {
  let (code, name, desc) = invalid_country_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_password() -> serde_json::Value {
  let (code, name, desc) = invalid_password_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_username() -> serde_json::Value {
  let (code, name, desc) = invalid_username_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_role() -> serde_json::Value {
  let (code, name, desc) = invalid_role_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_status() -> serde_json::Value {
  let (code, name, desc) = invalid_status_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_date_of_birth() -> serde_json::Value {
  let (code, name, desc) = invalid_date_of_birth_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_majority() -> serde_json::Value {
  let (code, name, desc) = invalid_majority_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_marital_status() -> serde_json::Value {
  let (code, name, desc) = invalid_marital_status_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_nationality() -> serde_json::Value {
  let (code, name, desc) = invalid_nationality_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_language() -> serde_json::Value {
  let (code, name, desc) = invalid_language_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_currency() -> serde_json::Value {
  let (code, name, desc) = invalid_currency_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_time_zone() -> serde_json::Value {
  let (code, name, desc) = invalid_time_zone_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_profile_picture() -> serde_json::Value {
  let (code, name, desc) = invalid_profile_picture_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_cover_picture() -> serde_json::Value {
  let (code, name, desc) = invalid_cover_picture_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_bio() -> serde_json::Value {
  let (code, name, desc) = invalid_bio_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_website() -> serde_json::Value {
  let (code, name, desc) = invalid_website_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_facebook() -> serde_json::Value {
  let (code, name, desc) = invalid_facebook_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_twitter() -> serde_json::Value {
  let (code, name, desc) = invalid_twitter_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_instagram() -> serde_json::Value {
  let (code, name, desc) = invalid_instagram_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_linkedin() -> serde_json::Value {
  let (code, name, desc) = invalid_linkedin_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_github() -> serde_json::Value {
  let (code, name, desc) = invalid_github_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_gitlab() -> serde_json::Value {
  let (code, name, desc) = invalid_gitlab_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_bitbucket() -> serde_json::Value {
  let (code, name, desc) = invalid_bitbucket_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_google() -> serde_json::Value {
  let (code, name, desc) = invalid_google_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_youtube() -> serde_json::Value {
  let (code, name, desc) = invalid_youtube_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_twitch() -> serde_json::Value {
  let (code, name, desc) = invalid_twitch_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_discord() -> serde_json::Value {
  let (code, name, desc) = invalid_discord_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_slack() -> serde_json::Value {
  let (code, name, desc) = invalid_slack_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_telegram() -> serde_json::Value {
  let (code, name, desc) = invalid_telegram_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_whatsapp() -> serde_json::Value {
  let (code, name, desc) = invalid_whatsapp_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_skype() -> serde_json::Value {
  let (code, name, desc) = invalid_skype_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_snapchat() -> serde_json::Value {
  let (code, name, desc) = invalid_snapchat_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_pinterest() -> serde_json::Value {
  let (code, name, desc) = invalid_pinterest_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_tumblr() -> serde_json::Value {
  let (code, name, desc) = invalid_tumblr_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_flickr() -> serde_json::Value {
  let (code, name, desc) = invalid_flickr_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_vimeo() -> serde_json::Value {
  let (code, name, desc) = invalid_vimeo_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_soundcloud() -> serde_json::Value {
  let (code, name, desc) = invalid_soundcloud_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_spotify() -> serde_json::Value {
  let (code, name, desc) = invalid_spotify_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_tiktok() -> serde_json::Value {
  let (code, name, desc) = invalid_tiktok_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_vine() -> serde_json::Value {
  let (code, name, desc) = invalid_vine_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_reddit() -> serde_json::Value {
  let (code, name, desc) = invalid_reddit_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_expiration_date() -> serde_json::Value {
  let (code, name, desc) = invalid_expiration_date_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn session_key_not_present_in_header() -> serde_json::Value {
  let (code, name, desc) = session_key_not_present_in_header_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn session_key_present_and_not_decryptable_parsable() -> serde_json::Value {
  let (code, name, desc) = session_key_present_and_not_decryptable_parsable_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn reference_has_no_linked_cards() -> serde_json::Value {
  let (code, name, desc) = reference_has_no_linked_cards_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn card_already_linked_to_a_different_reference() -> serde_json::Value {
  let (code, name, desc) = card_already_linked_to_a_different_reference_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn excluded_by_file_type_exclusions() -> serde_json::Value {
  let (code, name, desc) = excluded_by_file_type_exclusions_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_card_information() -> serde_json::Value {
  let (code, name, desc) = invalid_card_information_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn cannot_disable_physical_card() -> serde_json::Value {
  let (code, name, desc) = cannot_disable_physical_card_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn missing_token() -> serde_json::Value {
  let (code, name, desc) = missing_token_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn user_not_found() -> serde_json::Value {
  let (code, name, desc) = user_not_found_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn already_exists() -> serde_json::Value {
  let (code, name, desc) = already_exists_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn database_error() -> serde_json::Value {
  let (code, name, desc) = database_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn hashing_error() -> serde_json::Value {
  let (code, name, desc) = hashing_error_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_login() -> serde_json::Value {
  let (code, name, desc) = invalid_login_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_user() -> serde_json::Value {
  let (code, name, desc) = invalid_user_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_user_id() -> serde_json::Value {
  let (code, name, desc) = invalid_user_id_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_user_role() -> serde_json::Value {
  let (code, name, desc) = invalid_user_role_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_credentials() -> serde_json::Value {
  let (code, name, desc) = invalid_credentials_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_google_meet() -> serde_json::Value {
  let (code, name, desc) = invalid_google_meet_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_pseudonym() -> serde_json::Value {
  let (code, name, desc) = invalid_pseudonym_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_tag() -> serde_json::Value {
  let (code, name, desc) = invalid_tag_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn invalid_authorization_code() -> serde_json::Value {
  let (code, name, desc) = invalid_authorization_code_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

pub fn request_denied() -> serde_json::Value {
  let (code, name, desc) = request_denied_tuple();
  json!({ "status": code, "name": name, "description": desc })
}

// Unit tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generated_function_approved_no_action_required() {
    let response = ResponsesLocalApiCodes::ApprovedNoActionRequired;
    let (code, description) = response.into();
    assert_eq!(code, 900);
    assert_eq!(description, "The operation was approved and no further action is needed");
  }

  #[test]
  fn test_to_16_approved() {
    let response = ResponsesLocalApiCodes::Approved;
    let code = response.to_u16();
    assert_eq!(code, 901);
  }

  #[test]
  fn test_duplicated_transaction_id() {
    assert_eq!(
      duplicated_transaction_id_tuple(),
      (
        902,
        "Duplicated Transaction ID",
        "The transaction ID is a duplicate; the transaction has already been processed"
      )
    );
  }

  #[test]
  fn test_from_u16_validation_errors_provided() {
    let response = ResponsesLocalApiCodes::from_u16(903);
    assert_eq!(response, Some(ResponsesLocalApiCodes::ValidationErrorsProvided));
  }

  #[test]
  fn test_operation_not_allowed() {
    let (code, name, response) = operation_not_allowed_tuple();
    assert_eq!(code, 904);
    assert_eq!(name, "Operation Not Allowed");
    assert_eq!(response, "The requested operation is not permitted");
  }

  #[test]
  fn test_operation_not_supported() {
    let (code, name, response) = operation_not_supported_tuple();
    assert_eq!(code, 905);
    assert_eq!(name, "Operation Not Supported");
    assert_eq!(
      response,
      json!({ "status": 905, "name": "Operation Not Supported", "description": "The requested operation is not supported by the system"})
    );
  }
}
