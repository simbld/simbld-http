//! This file defines the HttpCode struct which represents detailed HTTP code metadata.
//! The as_unified_tuple method returns a UnifiedTuple struct with optional internal fields.
//! If standard_code equals internal_code, the optional fields are None; otherwise they are Some(...).
//! Example: HttpCode::new(202, "Accepted", "Success", 202, "Accepted") returns a UnifiedTuple with None for internal_code and internal_name.

use crate::helpers::unified_tuple_helper::UnifiedTuple;
use serde_json::json;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpCode {
    /// Standard HTTP code.
    pub standard_code: u16,
    /// Standard HTTP name.
    pub standard_name: &'static str,
    /// Description of the HTTP response.
    pub description: &'static str,
    /// Internal HTTP code.
    pub internal_code: u16,
    /// Internal HTTP name.
    pub internal_name: &'static str,
}

impl HttpCode {
    pub(crate) fn get_description(&self) -> &str {
        todo!()
    }
}

impl HttpCode {
    /// Creates a new HttpCode.
    /// Example: HttpCode::new(202, "Accepted", "Request processed", 202, "Accepted")
    pub fn new(
        standard_code: u16,
        standard_name: &'static str,
        description: &'static str,
        internal_code: u16,
        internal_name: &'static str,
    ) -> Self {
        Self { standard_code, standard_name, description, internal_code, internal_name }
    }

    /// Returns a unified tuple representation.
    /// If standard_code equals internal_code, internal fields are set to None.
    pub fn as_unified_tuple(&self) -> UnifiedTuple {
        if self.standard_code == self.internal_code {
            UnifiedTuple {
                code: self.standard_code,
                name: self.standard_name,
                description: self.description,
                internal_code: None,
                internal_name: None,
            }
        } else {
            UnifiedTuple {
                code: self.standard_code,
                name: self.standard_name,
                description: self.description,
                internal_code: Some(self.internal_code),
                internal_name: Some(self.internal_name),
            }
        }
    }

    /// Converts the HttpCode into a JSON representation.
    pub fn as_json(&self) -> serde_json::Value {
        if self.standard_code == self.internal_code {
            json!({
                "code": self.standard_code,
                "name": self.standard_name,
                "description": self.description,
            })
        } else {
            json!({
                "standard_http_code": {
                    "code": self.standard_code,
                    "name": self.standard_name,
                },
                "internal_http_code": {
                    "code": self.internal_code,
                    "name": self.internal_name,
                },
                "description": self.description,
            })
        }
    }
}
