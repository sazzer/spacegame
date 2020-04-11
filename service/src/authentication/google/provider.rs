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
    todo!()
  }
}
