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
    (code, description) // Tuple
  }
}

/// The functions returns a tuple containing an unsigned 16-bit integer and a static string indicating that the operation was approved with no further action required.
pub fn approved_no_action_required_tuple() -> (u16, &'static str, &'static str) {
  (900, "Approved No Action Required", "The operation was approved and no further action is needed")
}

pub fn approved_tuple() -> (u16, &'static str, &'static str) {
  (901, "Approved", "The operation was successfully approved")
}

pub fn duplicated_transaction_id_tuple() -> (u16, &'static str, &'static str) {
  (
    902,
    "Duplicated Transaction ID",
    "The transaction ID is a duplicate; the transaction has already been processed",
  )
}

pub fn validation_errors_provided_tuple() -> (u16, &'static str, &'static str) {
  (
    903,
    "Validation Errors Provided",
    "Validation errors occurred. Please verify the provided values and try again",
  )
}

pub fn operation_not_allowed_tuple() -> (u16, &'static str, &'static str) {
  (904, "Operation Not Allowed", "The requested operation is not permitted")
}

pub fn operation_not_supported_tuple() -> (u16, &'static str, &'static str) {
  (905, "Operation Not Supported", "The requested operation is not supported by the system")
}

pub fn transaction_timeout_tuple() -> (u16, &'static str, &'static str) {
  (906, "Transaction Timeout", "The transaction could not be completed due to a timeout (e.g., an authorization expired before capture)")
}

pub fn authentification_failed_tuple() -> (u16, &'static str, &'static str) {
  (907, "Authentification Failed", "Authentication failed due to incorrect or missing credentials")
}

pub fn do_not_honor_tuple() -> (u16, &'static str, &'static str) {
  (908, "Do Not Honor", "General decline with no specific reason provided, or insufficient funds")
}

pub fn insufficient_funds_tuple() -> (u16, &'static str, &'static str) {
  (909, "Insufficient Funds", "The account does not have enough funds to complete the transaction")
}

pub fn incorrect_pin_tuple() -> (u16, &'static str, &'static str) {
  (910, "Incorrect PIN", "The provided PIN is incorrect")
}

pub fn invalid_transaction_tuple() -> (u16, &'static str, &'static str) {
  (911, "Invalid Transaction", "The transaction request is invalid or unsupported")
}

pub fn invalid_amount_tuple() -> (u16, &'static str, &'static str) {
  (912, "Invalid Amount", "The specified amount is invalid")
}

pub fn invalid_card_number_tuple() -> (u16, &'static str, &'static str) {
  (913, "Invalid Card Number", "The card number (PAN) is invalid or the card type is not accepted")
}

pub fn invalid_cvv_tuple() -> (u16, &'static str, &'static str) {
  (914, "Invalid CVV", "The provided CVV code is invalid")
}

pub fn invalid_card_holder_name_tuple() -> (u16, &'static str, &'static str) {
  (915, "Invalid Card Holder Name", "The cardholder's name is invalid")
}

pub fn invalid_card_holder_last_name_tuple() -> (u16, &'static str, &'static str) {
  (916, "Invalid Card Holder Last Name", "The cardholder's last name is invalid")
}

pub fn invalid_card_holder_first_name_tuple() -> (u16, &'static str, &'static str) {
  (917, "Invalid Card Holder First Name", "The cardholder's first name is invalid")
}

pub fn invalid_card_holder_id_number_tuple() -> (u16, &'static str, &'static str) {
  (918, "Invalid Card Holder ID Number", "The cardholder's ID number is invalid")
}

pub fn invalid_card_holder_phone_number_tuple() -> (u16, &'static str, &'static str) {
  (919, "Invalid Card Holder Phone Number", "The cardholder's phone number is invalid")
}

pub fn card_already_active_tuple() -> (u16, &'static str, &'static str) {
  (920, "Card Already Active", "The card is already active and cannot be reactivated")
}

pub fn card_not_active_tuple() -> (u16, &'static str, &'static str) {
  (921, "Card Not Active", "The card is not active or cannot be found")
}

pub fn expired_card_tuple() -> (u16, &'static str, &'static str) {
  (922, "Expired Card", "The card has expired and cannot be used")
}

pub fn lost_card_tuple() -> (u16, &'static str, &'static str) {
  (923, "Lost Card", "The card has been reported lost and cannot be used")
}

pub fn stolen_card_tuple() -> (u16, &'static str, &'static str) {
  (924, "Stolen Card", "The card has been reported stolen and cannot be used")
}

pub fn invalid_last_name_tuple() -> (u16, &'static str, &'static str) {
  (925, "Invalid Last Name", "The last name has an error or does not exist")
}

pub fn invalid_first_name_tuple() -> (u16, &'static str, &'static str) {
  (926, "Invalid First Name", "The first name has an error or does not exist")
}

pub fn invalid_id_number_tuple() -> (u16, &'static str, &'static str) {
  (927, "Invalid ID Number", "The ID number has an error or does not exist")
}

pub fn invalid_phone_number_tuple() -> (u16, &'static str, &'static str) {
  (928, "Invalid Phone Number", "The phone number has an error or does not exist")
}

pub fn invalid_email_tuple() -> (u16, &'static str, &'static str) {
  (929, "Invalid Email", "The email address has an error or does not exist")
}

pub fn invalid_initials_tuple() -> (u16, &'static str, &'static str) {
  (930, "Invalid Initials", "The initials phas an error or does not exist")
}

pub fn invalid_address_tuple() -> (u16, &'static str, &'static str) {
  (931, "Invalid Address", "The address has an error or does not exist")
}

pub fn invalid_city_tuple() -> (u16, &'static str, &'static str) {
  (932, "Invalid City", "The city has an error or does not exist")
}

pub fn invalid_postal_code_tuple() -> (u16, &'static str, &'static str) {
  (933, "Invalid Postal Code", "The postal code has an error or does not exist")
}

pub fn invalid_country_tuple() -> (u16, &'static str, &'static str) {
  (934, "Invalid Country", "The country has an error or does not exist")
}

pub fn invalid_password_tuple() -> (u16, &'static str, &'static str) {
  (935, "Invalid Password", "The password has an error or does not exist")
}

pub fn invalid_username_tuple() -> (u16, &'static str, &'static str) {
  (936, "Invalid Username", "The username has an error or does not exist")
}

pub fn invalid_role_tuple() -> (u16, &'static str, &'static str) {
  (937, "Invalid Role", "The role shas an error or does not exist")
}

pub fn invalid_status_tuple() -> (u16, &'static str, &'static str) {
  (938, "Invalid Status", "The status shas an error or does not exist")
}

pub fn invalid_date_of_birth_tuple() -> (u16, &'static str, &'static str) {
  (939, "Invalid Date of Birth", "The date of birth provided is invalid")
}

pub fn invalid_majority_tuple() -> (u16, &'static str, &'static str) {
  (940, "Invalid Majority", "The majority information provided is invalid")
}

pub fn invalid_marital_status_tuple() -> (u16, &'static str, &'static str) {
  (941, "Invalid Marital Status", "The marital status provided is invalid")
}

pub fn invalid_nationality_tuple() -> (u16, &'static str, &'static str) {
  (942, "Invalid Nationality", "The nationality has an error or does not exist")
}

pub fn invalid_language_tuple() -> (u16, &'static str, &'static str) {
  (943, "Invalid Language", "The language has an error or does not exist")
}

pub fn invalid_currency_tuple() -> (u16, &'static str, &'static str) {
  (944, "Invalid Currency", "The currency provided is invalid")
}

pub fn invalid_time_zone_tuple() -> (u16, &'static str, &'static str) {
  (945, "Invalid Time Zone", "The time zone shas an error or does not exist")
}

pub fn invalid_profile_picture_tuple() -> (u16, &'static str, &'static str) {
  (946, "Invalid Profile Picture", "The profile picture is invalid or unsupported")
}

pub fn invalid_cover_picture_tuple() -> (u16, &'static str, &'static str) {
  (947, "Invalid Cover Picture", "The cover picture is invalid or unsupported")
}

pub fn invalid_bio_tuple() -> (u16, &'static str, &'static str) {
  (948, "Invalid Bio", "The bio has an error or does not exist")
}

pub fn invalid_website_tuple() -> (u16, &'static str, &'static str) {
  (949, "Invalid Website", "The website URL has an error or does not exist")
}

pub fn invalid_facebook_tuple() -> (u16, &'static str, &'static str) {
  (950, "Invalid Facebook", "The Facebook profile name has an error or does not exist")
}

pub fn invalid_twitter_tuple() -> (u16, &'static str, &'static str) {
  (951, "Invalid Twitter", "The Twitter profile name has an error or does not exist")
}

pub fn invalid_instagram_tuple() -> (u16, &'static str, &'static str) {
  (952, "Invalid Instagram", "The Instagram profile name has an error or does not exist")
}

pub fn invalid_linkedin_tuple() -> (u16, &'static str, &'static str) {
  (953, "Invalid LinkedIn", "The LinkedIn profile name has an error or does not exist")
}

pub fn invalid_github_tuple() -> (u16, &'static str, &'static str) {
  (954, "Invalid GitHub", "The GitHub profile name has an error or does not exist")
}

pub fn invalid_gitlab_tuple() -> (u16, &'static str, &'static str) {
  (955, "Invalid GitLab", "The GitLab profile name has an error or does not exist")
}

pub fn invalid_bitbucket_tuple() -> (u16, &'static str, &'static str) {
  (956, "Invalid Bitbucket", "The Bitbucket profile name has an error or does not exist")
}

pub fn invalid_google_tuple() -> (u16, &'static str, &'static str) {
  (957, "Invalid Google", "The Google profile name has an error or does not exist")
}

pub fn invalid_youtube_tuple() -> (u16, &'static str, &'static str) {
  (958, "Invalid YouTube", "The YouTube profile name has an error or does not exist")
}

pub fn invalid_twitch_tuple() -> (u16, &'static str, &'static str) {
  (959, "Invalid Twitch", "The Twitch profile name has an error or does not exist")
}

pub fn invalid_discord_tuple() -> (u16, &'static str, &'static str) {
  (960, "Invalid Discord", "The Discord profile name has an error or does not exist")
}

pub fn invalid_slack_tuple() -> (u16, &'static str, &'static str) {
  (961, "Invalid Slack", "The Slack profile name has an error or does not exist")
}

pub fn invalid_telegram_tuple() -> (u16, &'static str, &'static str) {
  (962, "Invalid Telegram", "The Telegram profile name has an error or does not exist")
}

pub fn invalid_whatsapp_tuple() -> (u16, &'static str, &'static str) {
  (963, "Invalid WhatsApp", "The WhatsApp profile name has an error or does not exist")
}

pub fn invalid_skype_tuple() -> (u16, &'static str, &'static str) {
  (964, "Invalid Skype", "The Skype profile name or ID has an error or does not exist")
}

pub fn invalid_snapchat_tuple() -> (u16, &'static str, &'static str) {
  (965, "Invalid Snapchat", "The Snapchat profile name has an error or does not exist")
}

pub fn invalid_pinterest_tuple() -> (u16, &'static str, &'static str) {
  (966, "Invalid Pinterest", "The Pinterest profile name has an error or does not exist")
}

pub fn invalid_tumblr_tuple() -> (u16, &'static str, &'static str) {
  (967, "Invalid Tumblr", "The Tumblr profile name has an error or does not exist")
}

pub fn invalid_flickr_tuple() -> (u16, &'static str, &'static str) {
  (968, "Invalid Flickr", "The Flickr profile name has an error or does not exist")
}

pub fn invalid_vimeo_tuple() -> (u16, &'static str, &'static str) {
  (969, "Invalid Vimeo", "The Vimeo profile name has an error or does not exist")
}

pub fn invalid_soundcloud_tuple() -> (u16, &'static str, &'static str) {
  (970, "Invalid SoundCloud", "The SoundCloud profile name has an error or does not exist")
}

pub fn invalid_spotify_tuple() -> (u16, &'static str, &'static str) {
  (971, "Invalid Spotify", "The Spotify profile name has an error or does not exist")
}

pub fn invalid_tiktok_tuple() -> (u16, &'static str, &'static str) {
  (972, "Invalid TikTok", "The TikTok profile name has an error or does not exist")
}

pub fn invalid_vine_tuple() -> (u16, &'static str, &'static str) {
  (973, "Invalid Vine", "The Vine profile name has an error or does not exist")
}

pub fn invalid_reddit_tuple() -> (u16, &'static str, &'static str) {
  (974, "Invalid Reddit", "The Reddit profile has an error or does not exist")
}

pub fn invalid_expiration_date_tuple() -> (u16, &'static str, &'static str) {
  (975, "Invalid Expiration Date", "The expiration date (MMYY) is invalid or in the past")
}

pub fn session_key_not_present_in_header_tuple() -> (u16, &'static str, &'static str) {
  (976, "Session Key Not Present In Header", "The session key is missing from the request header")
}

pub fn session_key_present_and_not_decryptable_parsable_tuple() -> (u16, &'static str, &'static str)
{
  (
    977,
    "Session Key Present And Not Decryptable Parsable",
    "The session key provided is invalid, corrupted, or unparsable",
  )
}

pub fn reference_has_no_linked_cards_tuple() -> (u16, &'static str, &'static str) {
  (978, "Reference Has No Linked Cards", "The reference provided does not have any linked cards")
}

pub fn card_already_linked_to_a_different_reference_tuple() -> (u16, &'static str, &'static str) {
  (
    979,
    "Card Already Linked To A Different Reference",
    "The card is already linked to a different reference and cannot be re-linked",
  )
}

pub fn excluded_by_file_type_exclusions_tuple() -> (u16, &'static str, &'static str) {
  (980, "Excluded By File Type Exclusions", "The uploaded file type is not allowed")
}

pub fn invalid_card_information_tuple() -> (u16, &'static str, &'static str) {
  (981, "Invalid Card Information", "The card information provided is invalid")
}

pub fn cannot_disable_physical_card_tuple() -> (u16, &'static str, &'static str) {
  (982, "Cannot Disable Physical Card", "The operation to disable a physical card is not allowed")
}

pub fn missing_token_tuple() -> (u16, &'static str, &'static str) {
  (983, "Missing Token", "The token is missing from the request")
}

pub fn user_not_found_tuple() -> (u16, &'static str, &'static str) {
  (984, "User Not Found", "User not found or does not exist in the system")
}

pub fn already_exists_tuple() -> (u16, &'static str, &'static str) {
  (985, "Already Exists", "User already exists in the system")
}

pub fn database_error_tuple() -> (u16, &'static str, &'static str) {
  (986, "Database Error", "Database error occurred, please try again later or contact support")
}

pub fn hashing_error_tuple() -> (u16, &'static str, &'static str) {
  (
    987,
    "Hashing Error",
    "Password hashing error occurred, please try again later or contact support",
  )
}

pub fn invalid_login_tuple() -> (u16, &'static str, &'static str) {
  (988, "Invalid Login", "The login has an error or does not exist")
}

pub fn invalid_user_tuple() -> (u16, &'static str, &'static str) {
  (989, "Invalid User", "The user has an error or does not exist")
}

pub fn invalid_user_id_tuple() -> (u16, &'static str, &'static str) {
  (990, "Invalid User ID", "The user ID has an error or does not exist")
}

pub fn invalid_user_role_tuple() -> (u16, &'static str, &'static str) {
  (991, "Invalid User Role", "The user role has an error or does not exist")
}

pub fn invalid_credentials_tuple() -> (u16, &'static str, &'static str) {
  (992, "Invalid Credentials", "The credentials has an error or does not exist")
}

pub fn invalid_google_meet_tuple() -> (u16, &'static str, &'static str) {
  (993, "Invalid Google Meet", "The google meet has an error or does not exist")
}

pub fn invalid_pseudonym_tuple() -> (u16, &'static str, &'static str) {
  (994, "Invalid Pseudonym", "The pseudonym has an error or does not exist")
}

pub fn invalid_tag_tuple() -> (u16, &'static str, &'static str) {
  (995, "Invalid Tag", "The tag has an error or does not exist")
}

pub fn invalid_authorization_code_tuple() -> (u16, &'static str, &'static str) {
  (996, "Invalid Authorization Code", "The authorization code has an error or does not exist")
}

pub fn request_denied_tuple() -> (u16, &'static str, &'static str) {
  (999, "Request Denied", "Unofficial HTTP  catch-all error code. The reason for the HTTP response varies based on the service or host")
}

/// The functions returns a tuple containing a status code and a JSON value with status and description fields.
pub fn approved_no_action_required() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = approved_no_action_required_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn approved() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = approved_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn duplicated_transaction_id() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = duplicated_transaction_id_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn validation_errors_provided() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = validation_errors_provided_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn operation_not_allowed() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = operation_not_allowed_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn operation_not_supported() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = operation_not_supported_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn transaction_timeout() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = transaction_timeout_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn authentification_failed() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = authentification_failed_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn do_not_honor() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = do_not_honor_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn insufficient_funds() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = insufficient_funds_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn incorrect_pin() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = incorrect_pin_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_transaction() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_transaction_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_amount() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_amount_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_card_number() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_card_number_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_cvv() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_cvv_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_card_holder_name() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_card_holder_name_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_card_holder_last_name() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_card_holder_last_name_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_card_holder_first_name() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_card_holder_first_name_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_card_holder_id_number() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_card_holder_id_number_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_card_holder_phone_number() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_card_holder_phone_number_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn card_already_active() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = card_already_active_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn card_not_active() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = card_not_active_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn expired_card() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = expired_card_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn lost_card() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = lost_card_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn stolen_card() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = stolen_card_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_last_name() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_last_name_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_first_name() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_first_name_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_id_number() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_id_number_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_phone_number() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_phone_number_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_email() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_email_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_initials() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_initials_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_address() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_address_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_city() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_city_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_postal_code() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_postal_code_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_country() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_country_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_password() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_password_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_username() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_username_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_role() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_role_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_status() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_status_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_date_of_birth() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_date_of_birth_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_majority() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_majority_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_marital_status() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_marital_status_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_nationality() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_nationality_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_language() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_language_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_currency() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_currency_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_time_zone() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_time_zone_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_profile_picture() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_profile_picture_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_cover_picture() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_cover_picture_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_bio() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_bio_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_website() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_website_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_facebook() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_facebook_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_twitter() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_twitter_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_instagram() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_instagram_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_linkedin() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_linkedin_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_github() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_github_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_gitlab() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_gitlab_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_bitbucket() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_bitbucket_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_google() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_google_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_youtube() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_youtube_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_twitch() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_twitch_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_discord() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_discord_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_slack() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_slack_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_telegram() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_telegram_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_whatsapp() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_whatsapp_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_skype() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_skype_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_snapchat() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_snapchat_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_pinterest() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_pinterest_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_tumblr() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_tumblr_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_flickr() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_flickr_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_vimeo() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_vimeo_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_soundcloud() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_soundcloud_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_spotify() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_spotify_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_tiktok() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_tiktok_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_vine() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_vine_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_reddit() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_reddit_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_expiration_date() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_expiration_date_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn session_key_not_present_in_header() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = session_key_not_present_in_header_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn session_key_present_and_not_decryptable_parsable() -> (u16, &'static str, serde_json::Value)
{
  let (code, name, desc) = session_key_present_and_not_decryptable_parsable_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn reference_has_no_linked_cards() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = reference_has_no_linked_cards_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn card_already_linked_to_a_different_reference() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = card_already_linked_to_a_different_reference_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn excluded_by_file_type_exclusions() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = excluded_by_file_type_exclusions_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_card_information() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_card_information_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn cannot_disable_physical_card() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = cannot_disable_physical_card_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn missing_token() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = missing_token_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn user_not_found() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = user_not_found_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn already_exists() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = already_exists_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn database_error() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = database_error_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn hashing_error() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = hashing_error_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_login() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_login_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_user() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_user_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_user_id() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_user_id_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_user_role() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_user_role_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_credentials() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_credentials_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_google_meet() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_google_meet_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_pseudonym() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_pseudonym_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_tag() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_tag_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn invalid_authorization_code() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = invalid_authorization_code_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
}

pub fn request_denied() -> (u16, &'static str, serde_json::Value) {
  let (code, name, desc) = request_denied_tuple();
  (code, name, json!({ "status": code, "name": name, "description": desc }))
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
    let (code, name, response) = operation_not_supported();
    assert_eq!(code, 905);
    assert_eq!(name, "Operation Not Supported");
    assert_eq!(
      response,
      json!({ "status": 905, "name": "Operation Not Supported", "description": "The requested operation is not supported by the system"})
    );
  }
}
