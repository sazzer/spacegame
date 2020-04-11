use crate::authentication::{service::ProviderRegistry, Provider, ProviderName};
use std::boxed::Box;
use std::collections::HashMap;

/// Builder for the Provider Registry
#[derive(Default)]
pub struct ProviderRegistryBuilder {
  providers: HashMap<ProviderName, Box<dyn Provider>>,
}

impl ProviderRegistryBuilder {
  /// Add a new provider to the registry we are building
  pub fn with_provider(mut self, name: ProviderName, provider: Box<dyn Provider>) -> Self {
    self.providers.insert(name, provider);
    self
  }

  /// Build the registry from the providers we've added so far
  pub fn build(self) -> ProviderRegistry {
    ProviderRegistry::new(self.providers)
  }
}
