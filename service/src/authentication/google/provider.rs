use crate::authentication::Provider;

/// Authentication Provider for authenticating with Google
pub struct GoogleProvider {}

impl GoogleProvider {
  /// Construct the Google Provider
  pub fn new() -> Self {
    Self {}
  }
}

impl Provider for GoogleProvider {}
