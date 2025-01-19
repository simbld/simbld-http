use crate::helpers::generate_responses_functions::generate_responses_functions;
use serde_json::json;
/// Enum representing HTTP response status codes and descriptions.
/// Each variant corresponds to a specific HTTP status code.
///
/// * Example:
/// ```rust
///
/// use simbld_http::responses::local::ResponsesLocalCodes::ApprovedNoActionRequired;
///
/// let response = ApprovedNoActionRequired;
/// let json = response.as_json();
/// println!("{:?}", json);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum ResponsesLocalApiCodes {
  ApprovedNoActionRequired,
  Approved,
  DuplicatedTransactionId,
  ValidationErrorsProvided,
  OperationNotAllowed,
  OperationNotSupported,
  TransactionTimeout,
  AuthentificationFailed,
  DoNotHonor,
  InsufficientFunds,
  IncorrectPIN,
  InvalidTransaction,
  InvalidAmount,
  InvalidCardNumber,
  InvalidCVV,
  InvalidCardHolderName,
  InvalidCardHolderLastName,
  InvalidCardHolderFirstName,
  InvalidCardHolderIdNumber,
  InvalidCardHolderPhoneNumber,
  CardAlreadyActive,
  CardNotActive,
  ExpiredCard,
  LostCard,
  InvalidLastName,
  InvalidFirstName,
  InvalidIdNumber,
  InvalidPhoneNumber,
  InvalidEmail,
  InvalidInitials,
  InvalidAddress,
  InvalidCity,
  InvalidPostalCode,
  InvalidCountry,
  InvalidPassword,
  InvalidUsername,
  InvalidRole,
  InvalidStatus,
  InvalidDateOfBirth,
  InvalidMajority,
  InvalidMaritalStatus,
  InvalidNationality,
  InvalidLanguage,
  InvalidCurrency,
  InvalidTimeZone,
  InvalidProfilePicture,
  InvalidCoverPicture,
  InvalidBio,
  InvalidWebsite,
  InvalidFacebook,
  InvalidTwitter,
  InvalidInstagram,
  InvalidLinkedin,
  InvalidGithub,
  InvalidGitlab,
  InvalidBitbucket,
  InvalidGoogle,
  InvalidYoutube,
  InvalidTwitch,
  InvalidDiscord,
  InvalidSlack,
  InvalidTelegram,
  InvalidWhatsapp,
  InvalidSkype,
  InvalidSnapchat,
  InvalidPinterest,
  InvalidTumblr,
  InvalidFlickr,
  InvalidVimeo,
  InvalidSoundCloud,
  InvalidSpotify,
  InvalidAppleMusic,
  InvalidTidal,
  InvalidDeezer,
  InvalidAmazonMusic,
  InvalidBandcamp,
  InvalidMixcloud,
  InvalidSnap,
  InvalidReddit,
  InvalidTikTok,
}

generate_responses_functions! {
  ApprovedNoActionRequired => (200, "OK", "Operation approved, no action needed.", 900, "Approved No Action Required", 0, "", "", ""),
  Approved => (200, "OK", "Operation successfully approved.", 901, "Approved", 0, "", "", ""),
  DuplicatedTransactionId => (400, "Bad Request", "Duplicate transaction ID; already processed.", 902, "Duplicated Transaction ID", 0, "", "", ""),
  ValidationErrorsProvided => (400, "Bad Request", "Validation errors occurred; check provided values.", 903, "Validation Errors Provided", 0, "", "", ""),
  OperationNotAllowed => (403, "Forbidden", "The requested operation is not permitted.", 904, "Operation Not Allowed", 0, "", "", ""),
  OperationNotSupported => (400, "Bad Request", "The requested operation is not supported.", 905, "Operation Not Supported", 0, "", "", ""),
  TransactionTimeout => (408, "Request Timeout", "Transaction could not complete before it expired.", 906, "Transaction Timeout", 0, "", "", ""),
  AuthentificationFailed => (401, "Unauthorized", "Authentication failed (incorrect credentials).", 907, "Authentification Failed", 0, "", "", ""),
  DoNotHonor => (402, "Payment Required", "General decline, insufficient funds or no reason provided.", 908, "Do Not Honor", 0, "", "", ""),
  InsufficientFunds => (402, "Payment Required", "Account has insufficient funds.", 909, "Insufficient Funds", 0, "", "", ""),
  IncorrectPIN => (400, "Bad Request", "PIN code is incorrect.", 910, "Incorrect PIN", 0, "", "", ""),
  InvalidTransaction => (400, "Bad Request", "Transaction request is invalid/unsupported.", 911, "Invalid Transaction", 0, "", "", ""),
  InvalidAmount => (400, "Bad Request", "Specified amount is invalid.", 912, "Invalid Amount", 0, "", "", ""),
  InvalidCardNumber => (400, "Bad Request", "Card number (PAN) is invalid or not accepted.", 913, "Invalid Card Number", 0, "", "", ""),
  InvalidCVV => (400, "Bad Request", "CVV code is invalid.", 914, "Invalid CVV", 0, "", "", ""),
  InvalidCardHolderName => (400, "Bad Request", "Card holder name is invalid.", 915, "Invalid Card Holder Name", 0, "", "", ""),
  InvalidCardHolderLastName => (400, "Bad Request", "Card holder's last name is invalid.", 916, "Invalid Card Holder Last Name", 0, "", "", ""),
  InvalidCardHolderFirstName => (400, "Bad Request", "Card holder's first name is invalid.", 917, "Invalid Card Holder First Name", 0, "", "", ""),
  InvalidCardHolderIdNumber => (400, "Bad Request", "Card holder's ID number is invalid.", 918, "Invalid Card Holder ID Number", 0, "", "", ""),
  InvalidCardHolderPhoneNumber => (400, "Bad Request", "Card holder's phone number is invalid.", 919, "Invalid Card Holder Phone Number", 0, "", "", ""),
  CardAlreadyActive => (400, "Bad Request", "Card is already active, cannot re-activate.", 920, "Card Already Active", 0, "", "", ""),
  CardNotActive => (400, "Bad Request", "Card is not active or not found.", 921, "Card Not Active", 0, "", "", ""),
  ExpiredCard => (400, "Bad Request", "Card has expired.", 922, "Expired Card", 0, "", "", ""),
  LostCard => (400, "Bad Request", "Card was reported lost.", 923, "Lost Card", 0, "", "", ""),
  InvalidLastName => (400, "Bad Request", "Last name provided is invalid.", 925, "Invalid Last Name", 0, "", "", ""),
  InvalidFirstName => (400, "Bad Request", "First name provided is invalid.", 926, "Invalid First Name", 0, "", "", ""),
  InvalidIdNumber => (400, "Bad Request", "ID number provided is invalid.", 927, "Invalid ID Number", 0, "", "", ""),
  InvalidPhoneNumber => (400, "Bad Request", "Phone number provided is invalid.", 928, "Invalid Phone Number", 0, "", "", ""),
  InvalidEmail => (400, "Bad Request", "Email address provided is invalid.", 929, "Invalid Email", 0, "", "", ""),
  InvalidInitials => (400, "Bad Request", "Initials provided are invalid.", 930, "Invalid Initials", 0, "", "", ""),
  InvalidAddress => (400, "Bad Request", "Address provided is invalid.", 931, "Invalid Address", 0, "", "", ""),
  InvalidCity => (400, "Bad Request", "City provided is invalid.", 932, "Invalid City", 0, "", "", ""),
  InvalidPostalCode => (400, "Bad Request", "Postal code provided is invalid.", 933, "Invalid Postal Code", 0, "", "", ""),
  InvalidCountry => (400, "Bad Request", "Country provided is invalid.", 934, "Invalid Country", 0, "", "", ""),
  InvalidPassword => (400, "Bad Request", "Password provided is invalid.", 935, "Invalid Password", 0, "", "", ""),
  InvalidUsername => (400, "Bad Request", "Username provided is invalid.", 936, "Invalid Username", 0, "", "", ""),
  InvalidRole => (400, "Bad Request", "Role specified is invalid.", 937, "Invalid Role", 0, "", "", ""),
  InvalidStatus => (400, "Bad Request", "Status specified is invalid.", 938, "Invalid Status", 0, "", "", ""),
  InvalidDateOfBirth => (400, "Bad Request", "Date of birth provided is invalid.", 939, "Invalid Date of Birth", 0, "", "", ""),
  InvalidMajority => (400, "Bad Request", "Majority information is invalid.", 940, "Invalid Majority", 0, "", "", ""),
  InvalidMaritalStatus => (400, "Bad Request", "Marital status is invalid.", 941, "Invalid Marital Status", 0, "", "", ""),
  InvalidNationality => (400, "Bad Request", "Nationality provided is invalid.", 942, "Invalid Nationality", 0, "", "", ""),
  InvalidLanguage => (400, "Bad Request", "Language provided is invalid.", 943, "Invalid Language", 0, "", "", ""),
  InvalidCurrency => (400, "Bad Request", "Currency provided is invalid.", 944, "Invalid Currency", 0, "", "", ""),
  InvalidTimeZone => (400, "Bad Request", "Time zone specified is invalid.", 945, "Invalid Time Zone", 0, "", "", ""),
  InvalidProfilePicture => (400, "Bad Request", "Profile picture is invalid or unsupported.", 946, "Invalid Profile Picture", 0, "", "", ""),
  InvalidCoverPicture => (400, "Bad Request", "Cover picture is invalid or unsupported.", 947, "Invalid Cover Picture", 0, "", "", ""),
  InvalidBio => (400, "Bad Request", "Bio provided is invalid.", 948, "Invalid Bio", 0, "", "", ""),
  InvalidWebsite => (400, "Bad Request", "Website URL provided is invalid.", 949, "Invalid Website", 0, "", "", ""),
  InvalidFacebook => (400, "Bad Request", "Facebook profile name provided is invalid.", 950, "Invalid Facebook", 0, "", "", ""),
  InvalidTwitter => (400, "Bad Request", "Twitter profile name provided is invalid.", 951, "Invalid Twitter", 0, "", "", ""),
  InvalidInstagram => (400, "Bad Request", "Instagram profile name provided is invalid.", 952, "Invalid Instagram", 0, "", "", ""),
  InvalidLinkedin => (400, "Bad Request", "LinkedIn profile name provided is invalid.", 953, "Invalid LinkedIn", 0, "", "", ""),
  InvalidGithub => (400, "Bad Request", "GitHub profile name provided is invalid.", 954, "Invalid GitHub", 0, "", "", ""),
  InvalidGitlab => (400, "Bad Request", "GitLab profile name provided is invalid.", 955, "Invalid GitLab", 0, "", "", ""),
  InvalidBitbucket => (400, "Bad Request", "Bitbucket profile name provided is invalid.", 956, "Invalid Bitbucket", 0, "", "", ""),
  InvalidGoogle => (400, "Bad Request", "Google profile name provided is invalid.", 957, "Invalid Google", 0, "", "", ""),
  InvalidYoutube => (400, "Bad Request", "YouTube profile name provided is invalid.", 958, "Invalid YouTube", 0, "", "", ""),
  InvalidTwitch => (400, "Bad Request", "Twitch profile name provided is invalid.", 959, "Invalid Twitch", 0, "", "", ""),
  InvalidDiscord => (400, "Bad Request", "Discord profile name provided is invalid.", 960, "Invalid Discord", 0, "", "", ""),
  InvalidSlack => (400, "Bad Request", "Slack profile name provided is invalid.", 961, "Invalid Slack", 0, "", "", ""),
  InvalidTelegram => (400, "Bad Request", "Telegram profile name provided is invalid.", 962, "Invalid Telegram", 0, "", "", ""),
  InvalidWhatsapp => (400, "Bad Request", "WhatsApp info provided is invalid.", 963, "Invalid WhatsApp", 0, "", "", ""),
  InvalidSkype => (400, "Bad Request", "Skype name/ID provided is invalid.", 964, "Invalid Skype", 0, "", "", ""),
  InvalidSnapchat => (400, "Bad Request", "Snapchat name provided is invalid.", 965, "Invalid Snapchat", 0, "", "", ""),
  InvalidPinterest => (400, "Bad Request", "Pinterest name provided is invalid.", 966, "Invalid Pinterest", 0, "", "", ""),
  InvalidTumblr => (400, "Bad Request", "Tumblr name provided is invalid.", 967, "Invalid Tumblr", 0, "", "", ""),
  InvalidFlickr => (400, "Bad Request", "Flickr name provided is invalid.", 968, "Invalid Flickr", 0, "", "", ""),
  InvalidVimeo => (400, "Bad Request", "Vimeo name provided is invalid.", 969, "Invalid Vimeo", 0, "", "", ""),
  InvalidSoundCloud => (400, "Bad Request", "SoundCloud name provided is invalid.", 970, "Invalid SoundCloud", 0, "", "", ""),
  InvalidSpotify => (400, "Bad Request", "Spotify name provided is invalid.", 971, "Invalid Spotify", 0, "", "", ""),
  InvalidAppleMusic => (400, "Bad Request", "Apple Music name provided is invalid.", 972, "Invalid Apple Music", 0, "", "", ""),
  InvalidTidal => (400, "Bad Request", "Tidal name provided is invalid.", 973, "Invalid Tidal", 0, "", "", ""),
  InvalidDeezer => (400, "Bad Request", "Deezer name provided is invalid.", 974, "Invalid Deezer", 0, "", "", ""),
  InvalidAmazonMusic => (400, "Bad Request", "Amazon Music name provided is invalid.", 975, "Invalid Amazon Music", 0, "", "", ""),
  InvalidBandcamp => (400, "Bad Request", "Bandcamp name provided is invalid.", 976, "Invalid Bandcamp", 0, "", "", ""),
  InvalidMixcloud => (400, "Bad Request", "Mixcloud name provided is invalid.", 977, "Invalid Mixcloud", 0, "", "", ""),
  InvalidSnap => (400, "Bad Request", "Snap profile name provided is invalid.", 978, "Invalid Snap", 0, "", "", ""),
  InvalidReddit => (400, "Bad Request", "Reddit profile name provided is invalid.", 979, "Invalid Reddit", 0, "", "", ""),
  InvalidTikTok => (400, "Bad Request", "TikTok profile name provided is invalid.", 980, "Invalid TikTok", 0, "", "", ""),

}

/// This file defines the `ResponsesLocalApiCodes` enum and provides five main functionalities:
/// 1. `to_u16()` - returns the standard HTTP code as a `u16`.
/// 2. `from_u16(u16) -> Option<Self>` - attempts to build a variant from a given code.
/// 3. `as_tuple()` - returns a `UnifiedTuple` with standard/internal codes, names, and a description.
/// 4. `as_json()` - converts the variant to a JSON object.
/// 5. `Into<(u16, &'static str)>` - yields `(std_code, std_name)`.
#[cfg(test)]
mod tests {
  use super::*;
  use crate::responses::ResponsesLocalApiCodes;

  #[test]
  fn test_local_api_codes_to_u16() {
    let response = ResponsesLocalApiCodes::InvalidCardNumber;
    let code = response.to_u16();
    assert_eq!(code, 913);
  }

  #[test]
  fn test_local_api_codes_from_u16() {
    let status = ResponsesLocalApiCodes::from_u16(914);
    assert_eq!(status, Some(ResponsesLocalApiCodes::InvalidCVV));
  }

  #[test]
  fn test_local_api_codes_as_tuple() {
    let code = ResponsesLocalApiCodes::InvalidEmail;
    let tuple = code.as_tuple();
    assert_eq!(
      tuple,
      UnifiedTuple::NineFields(
        400,
        "Bad Request",
        "Email address provided is invalid.",
        929,
        "Invalid Email",
        101,
        "meta1",
        "meta2",
        "meta3"
      )
    );
  }

  #[test]
  fn test_local_api_codes_as_json() {
    let code = ResponsesLocalApiCodes::InvalidPhoneNumber;
    let json_result = code.as_json();
    let expected_json = json!({
        "standard http code": {
            "code": 400,
            "name": "Bad Request"
        },
        "internal http code": {
            "code": 928,
            "name": "Invalid Phone Number"
        },
        "description": "Phone number provided is invalid.",
        "metadata": {
            "meta1": 102,
            "meta2": "meta1",
            "meta3": "meta2",
            "meta4": "meta3"
        }
    });
    assert_eq!(json_result, expected_json);
  }

  #[test]
  fn test_local_api_codes_into_tuple() {
    let code = ResponsesLocalApiCodes::InvalidPassword;
    let (std_code, std_name): (u16, &'static str) = code.into();
    assert_eq!(std_code, 400);
    assert_eq!(std_name, "Bad Request");
  }
}
