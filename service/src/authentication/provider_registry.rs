use super::*;
use std::boxed::Box;
use std::collections::HashMap;

/// Registry of all the login providers that we can use
pub struct ProviderRegistry {
  providers: HashMap<ProviderName, Box<dyn Provider>>,
}

impl ProviderRegistry {
  /// Construct the Provider Registry
  pub fn new(providers: HashMap<ProviderName, Box<dyn Provider>>) -> Self {
    ProviderRegistry { providers }
  }

  /// Get the list of providers from the registry
  pub fn list_providers(&self) -> Vec<&ProviderName> {
    self.providers.keys().collect()
  }
}
