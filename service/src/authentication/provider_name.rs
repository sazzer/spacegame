use serde::Serialize;
use std::str::FromStr;

/// Representation of the name of a login provider for a player
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub struct ProviderName(String);

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ProviderNameParseError {
  #[error("Provider Name was blank")]
  Blank,
}

impl FromStr for ProviderName {
  type Err = ProviderNameParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let name = s.trim();
    if name.is_empty() {
      Err(ProviderNameParseError::Blank)
    } else {
      Ok(ProviderName(name.to_owned()))
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
  use rstest::rstest;

  #[rstest(
    input,
    expected,
    case("google", "google"),
    case("  google", "google"),
    case("google  ", "google"),
    case("  google  ", "google"),
    case("google\t", "google"),
    case("\tgoogle", "google"),
    case("\tgoogle\t", "google")
  )]
  fn test_parse_success(input: &str, expected: &str) {
    let result: Result<ProviderName, ProviderNameParseError> = input.parse();

    assert_that!(&result, maybe_ok(eq(ProviderName(expected.to_owned()))));
  }

  #[rstest(
    input,
    expected,
    case("", ProviderNameParseError::Blank),
    case("  ", ProviderNameParseError::Blank),
    case("\t", ProviderNameParseError::Blank)
  )]
  fn test_parse_error(input: &str, expected: ProviderNameParseError) {
    let result: Result<ProviderName, ProviderNameParseError> = input.parse();

    assert_that!(&result, maybe_err(eq(expected)));
  }
}
