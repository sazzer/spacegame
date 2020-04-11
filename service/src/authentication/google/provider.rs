use crate::authentication::{Provider, StartAuthentication};

/// Authentication Provider for authenticating with Google
pub struct GoogleProvider {}

impl GoogleProvider {
  /// Construct the Google Provider
  pub fn new() -> Self {
    Self {}
  }
}

impl Provider for GoogleProvider {
  /// Start authentication with the provider
  fn start(&self) -> StartAuthentication {
    StartAuthentication {
      redirect_url: "http://www.google.com".to_owned(),
      nonce: Some("my_nonce".to_owned()),
    }
  }
}
