use super::*;

/// Registry of all the login providers that we can use
pub struct ProviderRegistry {}

impl ProviderRegistry {
  /// Construct the Provider Registry
  pub fn new() -> Self {
    ProviderRegistry {}
  }

  /// Get the list of providers from the registry
  pub fn list_providers(&self) -> Vec<ProviderName> {
    vec!["google".parse().unwrap(), "facebook".parse().unwrap()]
  }
}
