/// This macro generates an enum `ResponsesClientCodes` with associated methods for handling HTTP response codes.
/// This will generate the following:
///
/// - An enum `ResponsesClientCodes` with variants `Ok`, `BadRequest`, and `NotFound`.
/// - A method `to_u16` that converts an enum variant to its corresponding HTTP status code.
/// - A method `as_tuple` that returns a `ResponseTuple` containing the status code, name, and description.
/// - A method `from_u16` that converts an HTTP status code to its corresponding enum variant, if it exists.
/// - An implementation of `Into<(u16, std::borrow::Cow<'static, str>)>` for converting the enum variant into a tuple containing the status code and a description.
///
/// # Example
///
/// ```rust
/// use responses::ResponsesClientCodes;
///
///
/// generate_responses! {
///     Ok => (200, "OK"),
///     BadRequest => (400, "Bad Request"),
///     NotFound => (404, "Not Found")
/// }
///
/// let response = ResponsesClientCodes::Ok;
/// assert_eq!(response.to_u16(), 200);
/// assert_eq!(response.as_tuple().desc, "OK");
/// assert_eq!(ResponsesClientCodes::from_u16(404), Some(ResponsesClientCodes::NotFound));
/// let tuple: (u16, std::borrow::Cow<'static, str>) = response.into();
/// assert_eq!(tuple, (200, std::borrow::Cow::Borrowed("Ok")));
/// ```
#[macro_export]
macro_rules! generate_responses {
    ($($name:ident => ($code:expr, $desc:expr)),*) => {
        #[derive(Debug, PartialEq)]
        pub enum ResponsesClientCodes {
            $($name),*
        }

        impl ResponsesClientCodes {
            pub fn to_u16(&self) -> u16 {
                match self {
                    $(ResponsesClientCodes::$name => $code),*
                }
            }

            pub fn as_tuple(&self) -> ResponseTuple {
                match self {
                    $(ResponsesClientCodes::$name => ResponseTuple {
                        std_code: $std_code,
                        std_name: stringify!($int_name),
                        int_code: Some($int_code),
                        int_name: Some(stringify!($int_name)),
                        desc: $desc,
                    }),*
                }
            }

            pub fn from_u16(code: u16) -> Option<Self> {
                match code {
                    $($code => Some(ResponsesClientCodes::$name)),*,
                    _ => None,
                }
            }
        }

        impl Into<(u16, std::borrow::Cow<'static, str>)> for ResponsesClientCodes {
            fn into(self) -> (u16, std::borrow::Cow<'static, str>) {
                let code = self.to_u16();
                let description = match self {
                    $(ResponsesClientCodes::$name => std::borrow::Cow::Borrowed(stringify!($name))),*
                };
                (code, description)
            }
        }
    };
}

#[macro_export]
macro_rules! generate_responses_with_metadata {
    ($enum_name:ident, $($name:ident => ($std_code:expr, $std_name:expr, $desc:expr, $int_code:expr, $int_name:expr)),*) => {
        #[derive(Debug, PartialEq)]
        pub enum $enum_name {
            $($name),*
        }

        impl $enum_name {
            pub fn to_u16(&self) -> u16 {
                match self {
                    $($enum_name::$name => $std_code),*
                }
            }

            pub fn as_tuple(&self) -> ResponseTuple {
                match self {
                    $($enum_name::$name => ResponseTuple {
                        std_code: $std_code,
                        std_name: $std_name,
                        int_code: Some($int_code),
                        int_name: Some($int_name),
                        desc: $desc,
                    }),*
                }
            }

            pub fn from_u16(code: u16) -> Option<Self> {
                match code {
                    $($std_code => Some($enum_name::$name)),*,
                    _ => None,
                }
            }
        }

        impl Into<(u16, std::borrow::Cow<'static, str>, u16, std::borrow::Cow<'static, str>)> for $enum_name {
            fn into(self) -> (u16, std::borrow::Cow<'static, str>, u16, std::borrow::Cow<'static, str>) {
                let std_code = self.to_u16();
                let description = match self {
                    $($enum_name::$name => std::borrow::Cow::Borrowed($desc)),*
                };
                let int_code = match self {
                    $($enum_name::$name => $int_code),*
                };
                let int_name = match self {
                    $($enum_name::$name => std::borrow::Cow::Borrowed($int_name)),*
                };
                (std_code, description, int_code, int_name)
            }
        }
    };
}
