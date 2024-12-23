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
  InvalidLinkedIn = 953,
  #[strum(props(Description = "The GitHub profile name provided is invalid"))]
  InvalidGitHub = 954,
  #[strum(props(Description = "The GitLab profile name provided is invalid"))]
  InvalidGitLab = 955,
  #[strum(props(Description = "The Bitbucket profile name provided is invalid"))]
  InvalidBitbucket = 956,
  #[strum(props(Description = "The Google profile name provided is invalid"))]
  InvalidGoogle = 957,
  #[strum(props(Description = "The YouTube profile name provided is invalid"))]
  InvalidYouTube = 958,
  #[strum(props(Description = "The Twitch profile name provided is invalid"))]
  InvalidTwitch = 959,
  #[strum(props(Description = "The Discord profile name provided is invalid"))]
  InvalidDiscord = 960,
  #[strum(props(Description = "The Slack profile name provided is invalid"))]
  InvalidSlack = 961,
  #[strum(props(Description = "The Telegram profile name provided is invalid"))]
  InvalidTelegram = 962,
  #[strum(props(Description = "The WhatsApp profile name provided is invalid"))]
  InvalidWhatsApp = 963,
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
  InvalidSoundCloud = 970,
  #[strum(props(Description = "The Spotify profile name provided is invalid"))]
  InvalidSpotify = 971,
  #[strum(props(Description = "The TikTok profile name provided is invalid"))]
  InvalidTikTok = 972,
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
  #[strum(props(Description = "User not found"))]
  UserNotFound = 984,
  #[strum(props(Description = "User already exists"))]
  AlreadyExists = 985,
  #[strum(props(Description = "Database error"))]
  DatabaseError = 986,
  #[strum(props(Description = "Password hashing error"))]
  HashingError = 987,
  #[strum(props(Description = "Invalid login"))]
  InvalidLogin = 988,
  #[strum(props(Description = "Invalid user"))]
  InvalidUser = 989,
  #[strum(props(Description = "Invalid user ID"))]
  InvalidUserId = 990,
  #[strum(props(Description = "Invalid user role"))]
  InvalidUserRole = 991,
  #[strum(props(Description = "Invalid credentials"))]
  InvalidCredentials = 992,
  #[strum(props(Description = "User already exists"))]
  UserAlreadyExists = 993,
  #[strum(props(Description = "Invalid pseudonym"))]
  InvalidPseudonym = 994,
  #[strum(props(Description = "Invalid tag"))]
  InvalidTag = 995,
  #[strum(props(Description = "Invalid authorization code"))]
  InvalidAuthorizationCode = 996,
  #[strum(props(
    Description = "Unofficial HTTP status code LinkedIn that is returned by the server as a generic, or catch-all error code. The reason for the HTTP response varies based on the service or host"
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
pub fn approved_no_action_required_tuple() -> (u16, &'static str) {
  (900, "The operation was approved and no further action is needed")
}

pub fn approved_tuple() -> (u16, &'static str) {
  (901, "The operation was successfully approved")
}

pub fn duplicated_transaction_id_tuple() -> (u16, &'static str) {
  (902, "The transaction ID is a duplicate; the transaction has already been processed")
}

pub fn validation_errors_provided_tuple() -> (u16, &'static str) {
  (903, "Validation errors occurred. Please verify the provided values and try again")
}

pub fn operation_not_allowed_tuple() -> (u16, &'static str) {
  (904, "The requested operation is not permitted")
}

pub fn operation_not_supported_tuple() -> (u16, &'static str) {
  (905, "The requested operation is not supported by the system")
}

pub fn transaction_timeout_tuple() -> (u16, &'static str) {
  (906, "The transaction could not be completed due to a timeout (e.g., an authorization expired before capture)")
}

pub fn authentification_failed_tuple() -> (u16, &'static str) {
  (907, "Authentication failed due to incorrect or missing credentials")
}

pub fn do_not_honor_tuple() -> (u16, &'static str) {
  (908, "General decline with no specific reason provided, or insufficient funds")
}

pub fn insufficient_funds_tuple() -> (u16, &'static str) {
  (909, "The account does not have enough funds to complete the transaction")
}

pub fn incorrect_pin_tuple() -> (u16, &'static str) {
  (910, "The provided PIN is incorrect")
}

pub fn invalid_transaction_tuple() -> (u16, &'static str) {
  (911, "The transaction request is invalid or unsupported")
}

pub fn invalid_amount_tuple() -> (u16, &'static str) {
  (912, "The specified amount is invalid")
}

pub fn invalid_card_number_tuple() -> (u16, &'static str) {
  (913, "The card number (PAN) is invalid or the card type is not accepted")
}

pub fn invalid_cvv_tuple() -> (u16, &'static str) {
  (914, "The provided CVV code is invalid")
}

pub fn invalid_card_holder_name_tuple() -> (u16, &'static str) {
  (915, "The expiration date (MMYY) is invalid or in the past")
}

pub fn invalid_card_holder_last_name_tuple() -> (u16, &'static str) {
  (916, "The cardholder's last name is invalid")
}

pub fn invalid_card_holder_first_name_tuple() -> (u16, &'static str) {
  (917, "The cardholder's first name is invalid")
}

pub fn invalid_card_holder_id_number_tuple() -> (u16, &'static str) {
  (918, "The cardholder's ID number is invalid")
}

pub fn invalid_card_holder_phone_number_tuple() -> (u16, &'static str) {
  (919, "The cardholder's phone number is invalid")
}

pub fn card_already_active_tuple() -> (u16, &'static str) {
  (920, "The card is already active and cannot be reactivated")
}

pub fn card_not_active_tuple() -> (u16, &'static str) {
  (921, "The card is not active or cannot be found")
}

pub fn expired_card_tuple() -> (u16, &'static str) {
  (922, "The card has expired and cannot be used")
}

pub fn lost_card_tuple() -> (u16, &'static str) {
  (923, "The card has been reported lost and cannot be used")
}

pub fn stolen_card_tuple() -> (u16, &'static str) {
  (924, "The card has been reported stolen and cannot be used")
}

pub fn invalid_last_name_tuple() -> (u16, &'static str) {
  (925, "The last name provided is invalid")
}

pub fn invalid_first_name_tuple() -> (u16, &'static str) {
  (926, "The first name provided is invalid")
}

pub fn invalid_id_number_tuple() -> (u16, &'static str) {
  (927, "The ID number provided is invalid")
}

pub fn invalid_phone_number_tuple() -> (u16, &'static str) {
  (928, "The phone number provided is invalid")
}

pub fn invalid_email_tuple() -> (u16, &'static str) {
  (929, "The email address provided is invalid")
}

pub fn invalid_initials_tuple() -> (u16, &'static str) {
  (930, "The initials provided are invalid")
}

pub fn invalid_address_tuple() -> (u16, &'static str) {
  (931, "The address provided is invalid")
}

pub fn invalid_city_tuple() -> (u16, &'static str) {
  (932, "The city provided is invalid")
}

pub fn invalid_postal_code_tuple() -> (u16, &'static str) {
  (933, "The postal code provided is invalid")
}

pub fn invalid_country_tuple() -> (u16, &'static str) {
  (934, "The country provided is invalid")
}

pub fn invalid_password_tuple() -> (u16, &'static str) {
  (935, "The password provided is invalid")
}

pub fn invalid_username_tuple() -> (u16, &'static str) {
  (936, "The username provided is invalid")
}

pub fn invalid_role_tuple() -> (u16, &'static str) {
  (937, "The role specified is invalid")
}

pub fn invalid_status_tuple() -> (u16, &'static str) {
  (938, "The status specified is invalid")
}

pub fn invalid_date_of_birth_tuple() -> (u16, &'static str) {
  (939, "The date of birth provided is invalid")
}

pub fn invalid_majority_tuple() -> (u16, &'static str) {
  (940, "The majority information provided is invalid")
}

pub fn invalid_marital_status_tuple() -> (u16, &'static str) {
  (941, "The marital status provided is invalid")
}

pub fn invalid_nationality_tuple() -> (u16, &'static str) {
  (942, "The nationality provided is invalid")
}

pub fn invalid_language_tuple() -> (u16, &'static str) {
  (943, "The language provided is invalid")
}

pub fn invalid_currency_tuple() -> (u16, &'static str) {
  (944, "The currency provided is invalid")
}

pub fn invalid_time_zone_tuple() -> (u16, &'static str) {
  (945, "The time zone specified is invalid")
}

pub fn invalid_profile_picture_tuple() -> (u16, &'static str) {
  (946, "The profile picture is invalid or unsupported")
}

pub fn invalid_cover_picture_tuple() -> (u16, &'static str) {
  (947, "The cover picture is invalid or unsupported")
}

pub fn invalid_bio_tuple() -> (u16, &'static str) {
  (948, "The bio provided is invalid")
}

pub fn invalid_website_tuple() -> (u16, &'static str) {
  (949, "The website URL provided is invalid")
}

pub fn invalid_facebook_tuple() -> (u16, &'static str) {
  (950, "The Facebook profile name provided is invalid")
}

pub fn invalid_twitter_tuple() -> (u16, &'static str) {
  (951, "The Twitter profile name provided is invalid")
}

pub fn invalid_instagram_tuple() -> (u16, &'static str) {
  (952, "The Instagram profile name provided is invalid")
}

pub fn invalid_linked_in_tuple() -> (u16, &'static str) {
  (953, "The LinkedIn profile name provided is invalid")
}

pub fn invalid_git_hub_tuple() -> (u16, &'static str) {
  (954, "The GitHub profile name provided is invalid")
}

pub fn invalid_git_lab_tuple() -> (u16, &'static str) {
  (955, "The GitLab profile name provided is invalid")
}

pub fn invalid_bit_bucket_tuple() -> (u16, &'static str) {
  (956, "The Bitbucket profile name provided is invalid")
}

pub fn invalid_google_tuple() -> (u16, &'static str) {
  (957, "The Google profile name provided is invalid")
}

pub fn invalid_you_tube_tuple() -> (u16, &'static str) {
  (958, "The YouTube profile name provided is invalid")
}

pub fn invalid_twitch_tuple() -> (u16, &'static str) {
  (959, "The Twitch profile name provided is invalid")
}

pub fn invalid_discord_tuple() -> (u16, &'static str) {
  (960, "The Discord profile name provided is invalid")
}

pub fn invalid_slack_tuple() -> (u16, &'static str) {
  (961, "The Slack profile name provided is invalid")
}

pub fn invalid_telegram_tuple() -> (u16, &'static str) {
  (962, "The Telegram profile name provided is invalid")
}

pub fn invalid_whats_app_tuple() -> (u16, &'static str) {
  (963, "The WhatsApp profile name provided is invalid")
}

pub fn invalid_skype_tuple() -> (u16, &'static str) {
  (964, "The Skype profile name or ID provided is invalid")
}

pub fn invalid_snapchat_tuple() -> (u16, &'static str) {
  (965, "The Snapchat profile name provided is invalid")
}

pub fn invalid_pinterest_tuple() -> (u16, &'static str) {
  (966, "The Pinterest profile name provided is invalid")
}

pub fn invalid_tumblr_tuple() -> (u16, &'static str) {
  (967, "The Tumblr profile name provided is invalid")
}

pub fn invalid_flickr_tuple() -> (u16, &'static str) {
  (968, "The Flickr profile name provided is invalid")
}

pub fn invalid_vimeo_tuple() -> (u16, &'static str) {
  (969, "The Vimeo profile name provided is invalid")
}

pub fn invalid_sound_cloud_tuple() -> (u16, &'static str) {
  (970, "The SoundCloud profile name provided is invalid")
}

pub fn invalid_spotify_tuple() -> (u16, &'static str) {
  (971, "The Spotify profile name provided is invalid")
}

pub fn invalid_tik_tok_tuple() -> (u16, &'static str) {
  (972, "The TikTok profile name provided is invalid")
}

pub fn invalid_vine_tuple() -> (u16, &'static str) {
  (973, "The Vine profile name provided is invalid")
}

pub fn invalid_reddit_tuple() -> (u16, &'static str) {
  (974, "The Reddit profile name provided is invalid")
}

pub fn invalid_expiration_date_tuple() -> (u16, &'static str) {
  (975, "The expiration date (MMYY) is invalid or in the past")
}

pub fn session_key_not_present_in_header_tuple() -> (u16, &'static str) {
  (976, "The session key is missing from the request header")
}

pub fn session_key_present_and_not_decryptable_parsable_tuple() -> (u16, &'static str) {
  (977, "The session key provided is invalid, corrupted, or unparsable")
}

pub fn reference_has_no_linked_cards_tuple() -> (u16, &'static str) {
  (978, "The reference provided does not have any linked cards")
}

pub fn card_already_linked_to_a_different_reference_tuple() -> (u16, &'static str) {
  (979, "The card is already linked to a different reference and cannot be re-linked")
}

pub fn excluded_by_file_type_exclusions_tuple() -> (u16, &'static str) {
  (980, "The uploaded file type is not allowed")
}

pub fn invalid_card_information_tuple() -> (u16, &'static str) {
  (981, "The card information provided is invalid")
}

pub fn cannot_disable_physical_card_tuple() -> (u16, &'static str) {
  (982, "The operation to disable a physical card is not allowed")
}

pub fn missing_token_tuple() -> (u16, &'static str) {
  (983, "The token is missing from the request")
}

pub fn user_not_found_tuple() -> (u16, &'static str) {
  (984, "User not found")
}

pub fn already_exists_tuple() -> (u16, &'static str) {
  (985, "User already exists")
}

pub fn database_error_tuple() -> (u16, &'static str) {
  (986, "Database error")
}

pub fn hashing_error_tuple() -> (u16, &'static str) {
  (987, "Password hashing error")
}

pub fn invalid_login_tuple() -> (u16, &'static str) {
  (988, "Invalid login")
}

pub fn invalid_user_tuple() -> (u16, &'static str) {
  (989, "Invalid user")
}

pub fn invalid_user_id_tuple() -> (u16, &'static str) {
  (990, "Invalid user ID")
}

pub fn invalid_user_role_tuple() -> (u16, &'static str) {
  (991, "Invalid user role")
}

pub fn invalid_credentials_tuple() -> (u16, &'static str) {
  (992, "Invalid credentials")
}

pub fn google_meet_tuple() -> (u16, &'static str) {
  (993, "Invalid google meet profile name provided is invalid")
}

pub fn invalid_pseudonym_tuple() -> (u16, &'static str) {
  (994, "Invalid pseudonym")
}

pub fn invalid_tag_tuple() -> (u16, &'static str) {
  (995, "Invalid tag")
}

pub fn invalid_authorization_code_tuple() -> (u16, &'static str) {
  (996, "Invalid authorization code")
}

pub fn invalid_messenger_tuple() -> (u16, &'static str) {
  (997, "Invalid messenger profile name provided is invalid")
}

pub fn invalid_zoom_tuple() -> (u16, &'static str) {
  (998, "Invalid zoom profile name provided is invalid")
}

pub fn invalid_microsoft_teams_tuple() -> (u16, &'static str) {
  (999, "Invalid microsoft teams profile name provided is invalid")
}

pub fn request_denied_tuple() -> (u16, &'static str) {
  (999, "Unofficial HTTP status code LinkedIn that is returned by the server as a generic, or catch-all error code. The reason for the HTTP response varies based on the service or host")
}


/// The functions returns a tuple containing a status code and a JSON value with status and description fields.
pub fn approved_no_action_required() -> (u16, serde_json::Value) {
  let (code, description) = approved_no_action_required_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn approved() -> (u16, serde_json::Value) {
  let (code, description) = approved_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn duplicated_transaction_id() -> (u16, serde_json::Value) {
  let (code, description) = duplicated_transaction_id_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn validation_errors_provided() -> (u16, serde_json::Value) {
  let (code, description) = validation_errors_provided_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn operation_not_allowed() -> (u16, serde_json::Value) {
  let (code, description) = operation_not_allowed_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn operation_not_supported() -> (u16, serde_json::Value) {
  let (code, description) = operation_not_supported_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn transaction_timeout() -> (u16, serde_json::Value) {
  let (code, description) = transaction_timeout_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn authentification_failed() -> (u16, serde_json::Value) {
  let (code, description) = authentification_failed_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn do_not_honor() -> (u16, serde_json::Value) {
  let (code, description) = do_not_honor_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn insufficient_funds() -> (u16, serde_json::Value) {
  let (code, description) = insufficient_funds_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn incorrect_pin() -> (u16, serde_json::Value) {
  let (code, description) = incorrect_pin_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_transaction() -> (u16, serde_json::Value) {
  let (code, description) = invalid_transaction_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_amount() -> (u16, serde_json::Value) {
  let (code, description) = invalid_amount_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_card_number() -> (u16, serde_json::Value) {
  let (code, description) = invalid_card_number_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_cvv() -> (u16, serde_json::Value) {
  let (code, description) = invalid_cvv_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_card_holder_name() -> (u16, serde_json::Value) {
  let (code, description) = invalid_card_holder_name_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_card_holder_last_name() -> (u16, serde_json::Value) {
  let (code, description) = invalid_card_holder_last_name_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_card_holder_first_name() -> (u16, serde_json::Value) {
  let (code, description) = invalid_card_holder_first_name_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_card_holder_id_number() -> (u16, serde_json::Value) {
  let (code, description) = invalid_card_holder_id_number_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_card_holder_phone_number() -> (u16, serde_json::Value) {
  let (code, description) = invalid_card_holder_phone_number_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn card_already_active() -> (u16, serde_json::Value) {
  let (code, description) = card_already_active_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn card_not_active() -> (u16, serde_json::Value) {
  let (code, description) = card_not_active_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn expired_card() -> (u16, serde_json::Value) {
  let (code, description) = expired_card_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn lost_card() -> (u16, serde_json::Value) {
  let (code, description) = lost_card_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn stolen_card() -> (u16, serde_json::Value) {
  let (code, description) = stolen_card_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_last_name() -> (u16, serde_json::Value) {
  let (code, description) = invalid_last_name_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_first_name() -> (u16, serde_json::Value) {
  let (code, description) = invalid_first_name_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_id_number() -> (u16, serde_json::Value) {
  let (code, description) = invalid_id_number_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_phone_number() -> (u16, serde_json::Value) {
  let (code, description) = invalid_phone_number_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_email() -> (u16, serde_json::Value) {
  let (code, description) = invalid_email_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_initials() -> (u16, serde_json::Value) {
  let (code, description) = invalid_initials_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_address() -> (u16, serde_json::Value) {
  let (code, description) = invalid_address_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_city() -> (u16, serde_json::Value) {
  let (code, description) = invalid_city_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_postal_code() -> (u16, serde_json::Value) {
  let (code, description) = invalid_postal_code_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_country() -> (u16, serde_json::Value) {
  let (code, description) = invalid_country_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_password() -> (u16, serde_json::Value) {
  let (code, description) = invalid_password_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_username() -> (u16, serde_json::Value) {
  let (code, description) = invalid_username_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_role() -> (u16, serde_json::Value) {
  let (code, description) = invalid_role_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_status() -> (u16, serde_json::Value) {
  let (code, description) = invalid_status_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_date_of_birth() -> (u16, serde_json::Value) {
  let (code, description) = invalid_date_of_birth_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_majority() -> (u16, serde_json::Value) {
  let (code, description) = invalid_majority_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_marital_status() -> (u16, serde_json::Value) {
  let (code, description) = invalid_marital_status_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_nationality() -> (u16, serde_json::Value) {
  let (code, description) = invalid_nationality_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_language() -> (u16, serde_json::Value) {
  let (code, description) = invalid_language_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_currency() -> (u16, serde_json::Value) {
  let (code, description) = invalid_currency_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_time_zone() -> (u16, serde_json::Value) {
  let (code, description) = invalid_time_zone_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_profile_picture() -> (u16, serde_json::Value) {
  let (code, description) = invalid_profile_picture_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_cover_picture() -> (u16, serde_json::Value) {
  let (code, description) = invalid_cover_picture_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_bio() -> (u16, serde_json::Value) {
  let (code, description) = invalid_bio_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_website() -> (u16, serde_json::Value) {
  let (code, description) = invalid_website_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_facebook() -> (u16, serde_json::Value) {
  let (code, description) = invalid_facebook_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_twitter() -> (u16, serde_json::Value) {
  let (code, description) = invalid_twitter_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_instagram() -> (u16, serde_json::Value) {
  let (code, description) = invalid_instagram_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_linked_in() -> (u16, serde_json::Value) {
  let (code, description) = invalid_linked_in_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_git_hub() -> (u16, serde_json::Value) {
  let (code, description) = invalid_git_hub_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_git_lab() -> (u16, serde_json::Value) {
  let (code, description) = invalid_git_lab_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_bit_bucket() -> (u16, serde_json::Value) {
  let (code, description) = invalid_bit_bucket_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_google() -> (u16, serde_json::Value) {
  let (code, description) = invalid_google_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_you_tube() -> (u16, serde_json::Value) {
  let (code, description) = invalid_you_tube_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_twitch() -> (u16, serde_json::Value) {
  let (code, description) = invalid_twitch_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_discord() -> (u16, serde_json::Value) {
  let (code, description) = invalid_discord_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_slack() -> (u16, serde_json::Value) {
  let (code, description) = invalid_slack_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_telegram() -> (u16, serde_json::Value) {
  let (code, description) = invalid_telegram_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_whats_app() -> (u16, serde_json::Value) {
  let (code, description) = invalid_whats_app_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_skype() -> (u16, serde_json::Value) {
  let (code, description) = invalid_skype_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_snapchat() -> (u16, serde_json::Value) {
  let (code, description) = invalid_snapchat_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_pinterest() -> (u16, serde_json::Value) {
  let (code, description) = invalid_pinterest_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_tumblr() -> (u16, serde_json::Value) {
  let (code, description) = invalid_tumblr_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_flickr() -> (u16, serde_json::Value) {
  let (code, description) = invalid_flickr_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_vimeo() -> (u16, serde_json::Value) {
  let (code, description) = invalid_vimeo_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_sound_cloud() -> (u16, serde_json::Value) {
  let (code, description) = invalid_sound_cloud_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_spotify() -> (u16, serde_json::Value) {
  let (code, description) = invalid_spotify_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_tik_tok() -> (u16, serde_json::Value) {
  let (code, description) = invalid_tik_tok_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_vine() -> (u16, serde_json::Value) {
  let (code, description) = invalid_vine_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_reddit() -> (u16, serde_json::Value) {
  let (code, description) = invalid_reddit_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_expiration_date() -> (u16, serde_json::Value) {
  let (code, description) = invalid_expiration_date_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn session_key_not_present_in_header() -> (u16, serde_json::Value) {
  let (code, description) = session_key_not_present_in_header_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn session_key_present_and_not_decryptable_parsable() -> (u16, serde_json::Value) {
  let (code, description) = session_key_present_and_not_decryptable_parsable_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn reference_has_no_linked_cards() -> (u16, serde_json::Value) {
  let (code, description) = reference_has_no_linked_cards_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn card_already_linked_to_a_different_reference() -> (u16, serde_json::Value) {
  let (code, description) = card_already_linked_to_a_different_reference_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn excluded_by_file_type_exclusions() -> (u16, serde_json::Value) {
  let (code, description) = excluded_by_file_type_exclusions_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_card_information() -> (u16, serde_json::Value) {
  let (code, description) = invalid_card_information_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn cannot_disable_physical_card() -> (u16, serde_json::Value) {
  let (code, description) = cannot_disable_physical_card_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn missing_token() -> (u16, serde_json::Value) {
  let (code, description) = missing_token_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn user_not_found() -> (u16, serde_json::Value) {
  let (code, description) = user_not_found_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn already_exists() -> (u16, serde_json::Value) {
  let (code, description) = already_exists_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn database_error() -> (u16, serde_json::Value) {
  let (code, description) = database_error_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn hashing_error() -> (u16, serde_json::Value) {
  let (code, description) = hashing_error_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_login() -> (u16, serde_json::Value) {
  let (code, description) = invalid_login_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_user() -> (u16, serde_json::Value) {
  let (code, description) = invalid_user_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_user_id() -> (u16, serde_json::Value) {
  let (code, description) = invalid_user_id_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_user_role() -> (u16, serde_json::Value) {
  let (code, description) = invalid_user_role_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_credentials() -> (u16, serde_json::Value) {
  let (code, description) = invalid_credentials_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn google_meet() -> (u16, serde_json::Value) {
  let (code, description) = google_meet_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_pseudonym() -> (u16, serde_json::Value) {
  let (code, description) = invalid_pseudonym_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_tag() -> (u16, serde_json::Value) {
  let (code, description) = invalid_tag_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_authorization_code() -> (u16, serde_json::Value) {
  let (code, description) = invalid_authorization_code_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_messenger() -> (u16, serde_json::Value) {
  let (code, description) = invalid_messenger_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_zoom() -> (u16, serde_json::Value) {
  let (code, description) = invalid_zoom_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn invalid_microsoft_teams() -> (u16, serde_json::Value) {
  let (code, description) = invalid_microsoft_teams_tuple();
  (code, json!({ "status": code, "description": description }))
}

pub fn request_denied() -> (u16, serde_json::Value) {
  let (code, description) = request_denied_tuple();
  (code, json!({ "status": code, "description": description }))
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
      (902, "The transaction ID is a duplicate; the transaction has already been processed")
    );
  }

  #[test]
  fn test_from_u16_validation_errors_provided() {
    let response = ResponsesLocalApiCodes::from_u16(903);
    assert_eq!(response, Some(ResponsesLocalApiCodes::ValidationErrorsProvided));
  }

  #[test]
  fn test_operation_not_allowed() {
    let (code, response) = operation_not_allowed_tuple();
    assert_eq!(code, 904);
    assert_eq!(response, "The requested operation is not permitted");
  }

  #[test]
  fn test_operation_not_supported() {
    let (code, response) = operation_not_supported();
    assert_eq!(code, 905);
    assert_eq!(
      response,
      json!({ "status": 905, "description": "The requested operation is not supported by the system"})
    );
  }
}
