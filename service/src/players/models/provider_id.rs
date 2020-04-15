use serde::Serialize;
use std::str::FromStr;

/// The ID of a player at an Authentication Provider
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub struct ProviderId(String);

/// Errors that can occur when parsing a provider ID
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ProviderIdParseError {
  #[error("The value must not be blank")]
  Blank,
}

impl FromStr for ProviderId {
  type Err = ProviderIdParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() == 0 {
      Err(ProviderIdParseError::Blank)
    } else {
      Ok(Self(s.to_owned()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
  };

  #[test]
  fn test_parse_id() {
    let input = "1232123";
    let parsed: Result<ProviderId, ProviderIdParseError> = input.parse();

    assert_that!(&parsed, maybe_ok(eq(ProviderId("1232123".to_owned()))));
  }

  #[test]
  fn test_parse_padded_id() {
    let input = "   1232123   ";
    let parsed: Result<ProviderId, ProviderIdParseError> = input.parse();

    assert_that!(
      &parsed,
      maybe_ok(eq(ProviderId("   1232123   ".to_owned())))
    );
  }

  #[test]
  fn test_parse_empty_id() {
    let input = "";
    let parsed: Result<ProviderId, ProviderIdParseError> = input.parse();

    assert_that!(&parsed, maybe_err(eq(ProviderIdParseError::Blank)));
  }

  #[test]
  fn test_parse_blank_id() {
    let input = "   ";
    let parsed: Result<ProviderId, ProviderIdParseError> = input.parse();

    assert_that!(&parsed, maybe_ok(eq(ProviderId("   ".to_owned()))));
  }
}
