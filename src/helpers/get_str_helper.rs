use crate::responses::ResponsesTypes;

pub trait GetDescription {
  fn get_str(&self, field: &str) -> Option<&'static str>;
}

impl GetDescription for ResponsesTypes {
  fn get_str(&self, field: &str) -> Option<&'static str> {
    match field {
      "Description" => Some(match self {
        ResponsesTypes::Informational(_) => "Informational response",
        ResponsesTypes::Success(_) => "Successful response",
        ResponsesTypes::Redirection(_) => "Redirection response",
        ResponsesTypes::ClientError(_) => "Client error response",
        ResponsesTypes::ServerError(_) => "Server error response",
        ResponsesTypes::ServiceError(_) => "Service-specific error response",
        ResponsesTypes::CrawlerError(_) => "Crawler-specific error response",
        ResponsesTypes::LocalApiError(_) => "Local API-specific error response",
      }),
      _ => None,
    }
  }
}
