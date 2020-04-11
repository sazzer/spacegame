pub use super::http::configure::configure_authentication;
use super::ProviderRegistry;
use super::ProviderRegistryBuilder;
use std::sync::Arc;

/// Configure the provider registry for authentiction
pub fn configure_provider_registry() -> ProviderRegistry {
  let google = super::google::GoogleProvider::new();

  ProviderRegistryBuilder::default()
    .with_provider("google".parse().unwrap(), Arc::new(google))
    .build()
}
