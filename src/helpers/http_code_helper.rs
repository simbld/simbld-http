use crate::helpers::json_serializable::JsonSerializable;
/// This file defines the HttpCode struct which represents detailed HTTP code metadata.
/// The as_unified_tuple method returns a UnifiedTuple struct with optional internal fields.
/// If standard_code equals internal_code, the optional fields are None; otherwise they are Some(...).
/// Example: HttpCode::new(202, "Accepted", "Success", 202, "Accepted") returns a UnifiedTuple with None for internal_code and internal_name.
use crate::helpers::unified_tuple_helper::UnifiedTuple;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpCode {
    /// Standard HTTP code.
    pub standard_code: u16,
    /// Standard HTTP name.
    pub standard_name: &'static str,
    /// Description of the HTTP response.
    pub unified_description: &'static str,
    /// Internal HTTP code.
    pub internal_code: u16,
    /// Internal HTTP name.
    pub internal_name: &'static str,
}

impl JsonSerializable for HttpCode {
    fn as_json(&self) -> serde_json::Value {
        // Appelle `serde_json::to_value` pour convertir ce type en JSON
        serde_json::to_value(self).unwrap()
    }
}

impl Serialize for HttpCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HttpCode", 3)?;
        state.serialize_field("standard_code", &self.standard_code)?;
        state.serialize_field("standard_name", &self.standard_name)?;
        state.serialize_field("unified_description", &self.unified_description)?;

        if self.standard_code != self.internal_code {
            state.serialize_field("internal_code", &self.internal_code)?;
            state.serialize_field("internal_name", &self.internal_name)?;
        }

        state.end()
    }
}

impl HttpCode {
    /// Creates a new HttpCode.
    /// Example: HttpCode::new(202, "Accepted", "Request processed", 202, "Accepted")
    pub fn new(
        standard_code: u16,
        standard_name: &'static str,
        unified_description: &'static str,
        internal_code: u16,
        internal_name: &'static str,
    ) -> Self {
        Self { standard_code, standard_name, unified_description, internal_code, internal_name }
    }

    /// Returns a unified tuple representation.
    /// If standard_code equals internal_code, internal fields are set to None.
    pub fn as_unified_tuple(&self) -> UnifiedTuple {
        if self.standard_code == self.internal_code {
            UnifiedTuple {
                standard_code: self.standard_code,
                standard_name: self.standard_name,
                unified_description: self.unified_description,
                internal_code: None,
                internal_name: None,
            }
        } else {
            UnifiedTuple {
                standard_code: self.standard_code,
                standard_name: self.standard_name,
                unified_description: self.unified_description,
                internal_code: Some(self.internal_code),
                internal_name: Some(self.internal_name),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_serialize_http_code_without_internal_diff() {
        // Exemple standard où le code et le nom internes sont identiques aux standards
        let http_code = HttpCode::new(200, "OK", "Request completed", 200, "OK");

        let json_value = serde_json::to_value(http_code).unwrap();
        // Attente d'un résultat JSON simplifié
        assert_eq!(
            json_value,
            json!({
                "standard_code": 200,
                "standard_name": "OK",
                "unified_description": "Request completed"
            })
        );
    }

    #[test]
    fn test_serialize_http_code_with_internal_diff() {
        // Exemple où interne != standard
        let http_code = HttpCode::new(403, "Forbidden", "Access Denied", 9998, "Custom Forbidden");

        let json_value = serde_json::to_value(http_code).unwrap();
        // Attente d'un résultat avec les champs internes ajoutés
        assert_eq!(
            json_value,
            json!({
                "standard_code": 403,
                "standard_name": "Forbidden",
                "unified_description": "Access Denied",
                "internal_code": 9998,
                "internal_name": "Custom Forbidden"
            })
        );
    }
}
