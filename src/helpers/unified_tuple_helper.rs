//! This file defines the UnifiedTuple struct.
//! It represents a response code in a unified way using optional fields.
//! Example: If standard_code equals internal_code, internal_code and internal_name are None.

#[derive(Debug, PartialEq)]
pub struct UnifiedTuple {
  /// Standard HTTP code.
  pub code: u16,
  /// Standard HTTP name.
  pub name: &'static str,
  /// Description of the response.
  pub description: &'static str,
  /// Internal HTTP code (None if equal to standard code).
  pub internal_code: Option<u16>,
  /// Internal HTTP name (None if equal to standard name).
  pub internal_name: Option<&'static str>,
}
