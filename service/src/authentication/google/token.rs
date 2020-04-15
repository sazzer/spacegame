use crate::players::{PlayerLogin, Registration};
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

    let token = jsonwebtoken::dangerous_unsafe_decode::<GoogleClaims>(&id_token).map_err(|e| {
      log::warn!("Failed to deserialize ID Token from Google: {}", e);
      ClaimsError::UnknownError
    })?;
    log::info!("Decoded claims: {:?}", token);

    Ok(token.claims)
  }
}

impl From<GoogleClaims> for Registration {
  fn from(claims: GoogleClaims) -> Self {
    Self {
      name: claims.name.unwrap_or("".to_owned()),
      avatar: claims.picture,
      login: PlayerLogin {
        provider_name: "google".parse().unwrap(),
        provider_id: claims.sub.clone(),
        display_name: claims.email.unwrap_or(claims.sub),
      },
    }
  }
}
