use async_trait::async_trait;
use std::collections::HashMap;

/// Details needed to start authentication with an external provider
#[derive(Debug, PartialEq)]
pub struct StartAuthentication {
  pub url: String,
  pub nonce: Option<String>,
}

/// Errors that can occur when performing authentication
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthenticationError {
  #[error("An unknown error occurred")]
  UnknownError,
}

impl StartAuthentication {
  /// Create a new instance of the StartAuthentication struct with only a URL
  pub fn new<S>(url: S) -> Self
  where
    S: Into<String>,
  {
    Self {
      url: url.into(),
      nonce: None,
    }
  }

  /// Update the StartAuthentication struct to have a Nonce value
  pub fn with_nonce<S>(self, nonce: S) -> Self
  where
    S: Into<String>,
  {
    Self {
      url: self.url,
      nonce: Some(nonce.into()),
    }
  }
}

/// Trait that all login providers implement
#[async_trait]
pub trait Provider: Send + Sync {
  /// Start the authentication process, generating details to redirect the user to in order for them to log in
  fn start(&self) -> StartAuthentication;

  /// Complete the authentication process, returning the Player that has just authenticated
  async fn complete(&self, params: HashMap<String, String>) -> Result<String, AuthenticationError>;
}
