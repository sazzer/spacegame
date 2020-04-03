#[cfg(test)]
use mockall::automock;

/// Details needed to start authentication with an external provider
#[derive(Debug, PartialEq)]
pub struct StartAuthentication {
  pub url: String,
  pub nonce: Option<String>,
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
#[cfg_attr(test, automock)]
pub trait Provider: Send + Sync {
  /// Start the authentication process, generating details to redirect the user to in order for them to log in
  fn start(&self) -> StartAuthentication;
}
