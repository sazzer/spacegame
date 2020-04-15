use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

/// The ID of a player
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
pub struct PlayerId(Uuid);

/// Errors that can occur when parsing a Player ID
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum PlayerIdParseError {
  #[error("The value must not be blank: {0}")]
  Malformed(#[from] uuid::Error),
}

impl FromStr for PlayerId {
  type Err = PlayerIdParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parsed_uuid: Uuid = s.parse()?;
    Ok(Self(parsed_uuid))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use assert_matches::assert_matches;
  use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
  };

  #[test]
  fn test_parse_id() {
    let expected = Uuid::new_v4();

    let input = format!("{}", expected);
    let parsed: Result<PlayerId, PlayerIdParseError> = input.parse();

    assert_that!(&parsed, maybe_ok(eq(PlayerId(expected))));
  }

  #[test]
  fn test_parse_malformed_id() {
    let parsed: Result<PlayerId, PlayerIdParseError> = "1232123".parse();

    assert_matches!(parsed.unwrap_err(), PlayerIdParseError::Malformed(_));
  }

  #[test]
  fn test_parse_blank_id() {
    let parsed: Result<PlayerId, PlayerIdParseError> = "".parse();

    assert_matches!(parsed.unwrap_err(), PlayerIdParseError::Malformed(_));
  }
}
