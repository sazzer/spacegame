use super::*;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of all the login providers that we can use
#[derive(Clone)]
pub struct ProviderRegistry {
  providers: HashMap<ProviderName, Arc<dyn Provider>>,
}

impl ProviderRegistry {
  /// Construct the Provider Registry
  pub fn new(providers: HashMap<ProviderName, Arc<dyn Provider>>) -> Self {
    ProviderRegistry { providers }
  }

  /// Get the list of providers from the registry
  pub fn list_providers(&self) -> Vec<&ProviderName> {
    self.providers.keys().collect()
  }

  /// Find the Provider that has the given name
  pub fn find_provider(&self, provider_name: &ProviderName) -> Option<Arc<dyn Provider>> {
    self
      .providers
      .get(provider_name)
      .map(|provider| provider.clone())
  }
}
