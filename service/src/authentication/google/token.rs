use serde::Deserialize;
use std::convert::TryFrom;

/// The shape of the Access Token details retrieved from Google
#[derive(Debug, Deserialize)]
pub struct GoogleToken {
  id_token: Option<String>,
}

/// The shape of the claims inside the ID Token from Google
#[derive(Debug, Deserialize)]
pub struct GoogleClaims {
  sub: String,
  email: Option<String>,
  name: Option<String>,
  picture: Option<String>,
}

/// The errors that can occur when decoding the ID Token
#[derive(Debug, thiserror::Error)]
pub enum ClaimsError {
  #[error("No ID Token was present")]
  MissingIdToken,
  #[error("An unknown error occurred")]
  UnknownError,
}

impl TryFrom<GoogleToken> for GoogleClaims {
  type Error = ClaimsError;

  fn try_from(token: GoogleToken) -> Result<Self, Self::Error> {
    let id_token = token.id_token.ok_or(ClaimsError::MissingIdToken)?;

    let token = jsonwebtoken::dangerous_unsafe_decode::<GoogleClaims>(&id_token);
    log::info!("Decoded claims: {:?}", token);

    Err(ClaimsError::UnknownError)
  }
}
