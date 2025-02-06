use crate::responses::ResponsesTypes;

pub trait GetDescription {
    fn get_str(&self, field: &str) -> Option<&'static str>;
}

impl GetDescription for ResponsesTypes {
    fn get_str(&self, field: &str) -> Option<&'static str> {
        match field {
            "Description" => Some(match self {
                ResponsesTypes::Informational(status_code) => status_code.description(),
                ResponsesTypes::Success(status_code) => status_code.description(),
                ResponsesTypes::Redirection(status_code) => status_code.description(),
                ResponsesTypes::ClientError(status_code) => status_code.description(),
                ResponsesTypes::ServerError(status_code) => status_code.description(),
                ResponsesTypes::ServiceError(status_code) => status_code.description(),
                ResponsesTypes::CrawlerError(status_code) => status_code.description(),
                ResponsesTypes::LocalApiError(status_code) => status_code.description(),
            }),
            _ => None,
        }
    }
}
