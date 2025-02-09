use crate::generate_responses_functions;

generate_responses_functions! {
"",
  ResponsesLocalApiCodes,
  ApprovedNoActionRequired => (200, "OK", "Operation approved, no action needed.", 900, "Approved No Action Required"),
  Approved => (200, "OK", "Operation successfully approved.", 901, "Approved"),
  DuplicatedTransactionId => (400, "Bad Request", "Duplicate transaction ID; already processed.", 902, "Duplicated Transaction ID"),
  ValidationErrorsProvided => (400, "Bad Request", "Validation errors occurred; check provided values.", 903, "Validation Errors Provided"),
  OperationNotAllowed => (403, "Forbidden", "The requested operation is not permitted.", 904, "Operation Not Allowed"),
  OperationNotSupported => (400, "Bad Request", "The requested operation is not supported.", 905, "Operation Not Supported"),
  TransactionTimeout => (408, "Request Timeout", "Transaction could not complete before it expired.", 906, "Transaction Timeout"),
  AuthentificationFailed => (401, "Unauthorized", "Authentication failed (incorrect credentials).", 907, "Authentification Failed"),
  DoNotHonor => (402, "Payment Required", "General decline, insufficient funds or no reason provided.", 908, "Do Not Honor"),
  InsufficientFunds => (402, "Payment Required", "Account has insufficient funds.", 909, "Insufficient Funds"),
  IncorrectPIN => (400, "Bad Request", "PIN code is incorrect.", 910, "Incorrect PIN"),
  InvalidTransaction => (400, "Bad Request", "Transaction request is invalid/unsupported.", 911, "Invalid Transaction"),
  InvalidAmount => (400, "Bad Request", "Specified amount is invalid.", 912, "Invalid Amount"),
  InvalidCardNumber => (400, "Bad Request", "Card number (PAN) is invalid or not accepted.", 913, "Invalid Card Number"),
  InvalidCVV => (400, "Bad Request", "CVV code is invalid.", 914, "Invalid CVV"),
  InvalidCardHolderName => (400, "Bad Request", "Card holder name is invalid.", 915, "Invalid Card Holder Name"),
  InvalidCardHolderLastName => (400, "Bad Request", "Card holder's last name is invalid.", 916, "Invalid Card Holder Last Name"),
  InvalidCardHolderFirstName => (400, "Bad Request", "Card holder's first name is invalid.", 917, "Invalid Card Holder First Name"),
  InvalidCardHolderIdNumber => (400, "Bad Request", "Card holder's ID number is invalid.", 918, "Invalid Card Holder ID Number"),
  InvalidCardHolderPhoneNumber => (400, "Bad Request", "Card holder's phone number is invalid.", 919, "Invalid Card Holder Phone Number"),
  CardAlreadyActive => (400, "Bad Request", "Card is already active, cannot re-activate.", 920, "Card Already Active"),
  CardNotActive => (400, "Bad Request", "Card is not active or not found.", 921, "Card Not Active"),
  ExpiredCard => (400, "Bad Request", "Card has expired.", 922, "Expired Card"),
  LostCard => (400, "Bad Request", "Card was reported lost.", 923, "Lost Card"),
  InvalidLastName => (400, "Bad Request", "Last name provided is invalid.", 925, "Invalid Last Name"),
  InvalidFirstName => (400, "Bad Request", "First name provided is invalid.", 926, "Invalid First Name"),
  InvalidIdNumber => (400, "Bad Request", "ID number provided is invalid.", 927, "Invalid ID Number"),
  InvalidPhoneNumber => (400, "Bad Request", "Phone number provided is invalid.", 928, "Invalid Phone Number"),
  InvalidEmail => (400, "Bad Request", "Email address provided is invalid.", 929, "Invalid Email"),
  InvalidInitials => (400, "Bad Request", "Initials provided are invalid.", 930, "Invalid Initials"),
  InvalidAddress => (400, "Bad Request", "Address provided is invalid.", 931, "Invalid Address"),
  InvalidCity => (400, "Bad Request", "City provided is invalid.", 932, "Invalid City"),
  InvalidPostalCode => (400, "Bad Request", "Postal code provided is invalid.", 933, "Invalid Postal Code"),
  InvalidCountry => (400, "Bad Request", "Country provided is invalid.", 934, "Invalid Country"),
  InvalidPassword => (400, "Bad Request", "Password provided is invalid.", 935, "Invalid Password"),
  InvalidUsername => (400, "Bad Request", "Username provided is invalid.", 936, "Invalid Username"),
  InvalidRole => (400, "Bad Request", "Role specified is invalid.", 937, "Invalid Role"),
  InvalidStatus => (400, "Bad Request", "Status specified is invalid.", 938, "Invalid Status"),
  InvalidDateOfBirth => (400, "Bad Request", "Date of birth provided is invalid.", 939, "Invalid Date of Birth"),
  InvalidMajority => (400, "Bad Request", "Majority information is invalid.", 940, "Invalid Majority"),
  InvalidMaritalStatus => (400, "Bad Request", "Marital status is invalid.", 941, "Invalid Marital Status"),
  InvalidNationality => (400, "Bad Request", "Nationality provided is invalid.", 942, "Invalid Nationality"),
  InvalidLanguage => (400, "Bad Request", "Language provided is invalid.", 943, "Invalid Language"),
  InvalidCurrency => (400, "Bad Request", "Currency provided is invalid.", 944, "Invalid Currency"),
  InvalidTimeZone => (400, "Bad Request", "Time zone specified is invalid.", 945, "Invalid Time Zone"),
  InvalidProfilePicture => (400, "Bad Request", "Profile picture is invalid or unsupported.", 946, "Invalid Profile Picture"),
  InvalidCoverPicture => (400, "Bad Request", "Cover picture is invalid or unsupported.", 947, "Invalid Cover Picture"),
  InvalidBio => (400, "Bad Request", "Bio provided is invalid.", 948, "Invalid Bio"),
  InvalidWebsite => (400, "Bad Request", "Website URL provided is invalid.", 949, "Invalid Website"),
  InvalidFacebook => (400, "Bad Request", "Facebook profile name provided is invalid.", 950, "Invalid Facebook"),
  InvalidTwitter => (400, "Bad Request", "Twitter profile name provided is invalid.", 951, "Invalid Twitter"),
  InvalidInstagram => (400, "Bad Request", "Instagram profile name provided is invalid.", 952, "Invalid Instagram"),
  InvalidLinkedin => (400, "Bad Request", "LinkedIn profile name provided is invalid.", 953, "Invalid LinkedIn"),
  InvalidGithub => (400, "Bad Request", "GitHub profile name provided is invalid.", 954, "Invalid GitHub"),
  InvalidGitlab => (400, "Bad Request", "GitLab profile name provided is invalid.", 955, "Invalid GitLab"),
  InvalidBitbucket => (400, "Bad Request", "Bitbucket profile name provided is invalid.", 956, "Invalid Bitbucket"),
  InvalidGoogle => (400, "Bad Request", "Google profile name provided is invalid.", 957, "Invalid Google"),
  InvalidYoutube => (400, "Bad Request", "YouTube profile name provided is invalid.", 958, "Invalid YouTube"),
  InvalidTwitch => (400, "Bad Request", "Twitch profile name provided is invalid.", 959, "Invalid Twitch"),
  InvalidDiscord => (400, "Bad Request", "Discord profile name provided is invalid.", 960, "Invalid Discord"),
  InvalidSlack => (400, "Bad Request", "Slack profile name provided is invalid.", 961, "Invalid Slack"),
  InvalidTelegram => (400, "Bad Request", "Telegram profile name provided is invalid.", 962, "Invalid Telegram"),
  InvalidWhatsapp => (400, "Bad Request", "WhatsApp info provided is invalid.", 963, "Invalid WhatsApp"),
  InvalidSkype => (400, "Bad Request", "Skype name/ID provided is invalid.", 964, "Invalid Skype"),
  InvalidSnapchat => (400, "Bad Request", "Snapchat name provided is invalid.", 965, "Invalid Snapchat"),
  InvalidPinterest => (400, "Bad Request", "Pinterest name provided is invalid.", 966, "Invalid Pinterest"),
  InvalidTumblr => (400, "Bad Request", "Tumblr name provided is invalid.", 967, "Invalid Tumblr"),
  InvalidFlickr => (400, "Bad Request", "Flickr name provided is invalid.", 968, "Invalid Flickr"),
  InvalidVimeo => (400, "Bad Request", "Vimeo name provided is invalid.", 969, "Invalid Vimeo"),
  InvalidSoundCloud => (400, "Bad Request", "SoundCloud name provided is invalid.", 970, "Invalid SoundCloud"),
  InvalidSpotify => (400, "Bad Request", "Spotify name provided is invalid.", 971, "Invalid Spotify"),
  InvalidAppleMusic => (400, "Bad Request", "Apple Music name provided is invalid.", 972, "Invalid Apple Music"),
  InvalidTidal => (400, "Bad Request", "Tidal name provided is invalid.", 973, "Invalid Tidal"),
  InvalidDeezer => (400, "Bad Request", "Deezer name provided is invalid.", 974, "Invalid Deezer"),
  InvalidAmazonMusic => (400, "Bad Request", "Amazon Music name provided is invalid.", 975, "Invalid Amazon Music"),
  InvalidBandcamp => (400, "Bad Request", "Bandcamp name provided is invalid.", 976, "Invalid Bandcamp"),
  InvalidMixcloud => (400, "Bad Request", "Mixcloud name provided is invalid.", 977, "Invalid Mixcloud"),
  InvalidSnap => (400, "Bad Request", "Snap profile name provided is invalid.", 978, "Invalid Snap"),
  InvalidReddit => (400, "Bad Request", "Reddit profile name provided is invalid.", 979, "Invalid Reddit"),
  InvalidTikTok => (400, "Bad Request", "TikTok profile name provided is invalid.", 980, "Invalid TikTok"),

}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::helpers::unified_tuple_helper::UnifiedTuple;
  use crate::responses::ResponsesLocalApiCodes;
  use serde_json::json;
  
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
            UnifiedTuple(
                400,
                "Bad Request",
                "Email address provided is invalid.",
                929,
                "Invalid Email"
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
            "description": "Phone number provided is invalid."
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
