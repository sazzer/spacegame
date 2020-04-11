use crate::authentication::{service::ProviderRegistry, Provider, ProviderName};
use std::collections::HashMap;
use std::sync::Arc;

/// Builder for the Provider Registry
#[derive(Default)]
pub struct ProviderRegistryBuilder {
  providers: HashMap<ProviderName, Arc<dyn Provider>>,
}

impl ProviderRegistryBuilder {
  /// Add a new provider to the registry we are building
  pub fn with_provider(mut self, name: ProviderName, provider: Arc<dyn Provider>) -> Self {
    self.providers.insert(name, provider);
    self
  }

  /// Build the registry from the providers we've added so far
  pub fn build(self) -> ProviderRegistry {
    ProviderRegistry::new(self.providers)
  }
}
