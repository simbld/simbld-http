use std::fmt;

#[derive(Debug, Copy, Clone, strum_macros::EnumIter)]
pub enum MockResponses {
  Ok,
  BadRequest,
  Unauthorized,
  NotFound,
  InternalServerError,
}

impl Into<u16> for MockResponses {
  fn into(self) -> u16 {
    match self {
      MockResponses::Ok => 200,
      MockResponses::BadRequest => 400,
      MockResponses::Unauthorized => 401,
      MockResponses::NotFound => 404,
      MockResponses::InternalServerError => 500,
    }
  }
}

impl fmt::Display for MockResponses {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let response_name = match self {
      MockResponses::Ok => "Ok",
      MockResponses::BadRequest => "BadRequest",
      MockResponses::Unauthorized => "Unauthorized",
      MockResponses::NotFound => "NotFound",
      MockResponses::InternalServerError => "InternalServerError",
    };
    write!(f, "{}", response_name)
  }
}
