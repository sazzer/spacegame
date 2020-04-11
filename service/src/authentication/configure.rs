pub use super::http::configure::configure_authentication;
use super::ProviderRegistry;
use super::ProviderRegistryBuilder;

/// Configure the provider registry for authentiction
pub fn configure_provider_registry() -> ProviderRegistry {
  ProviderRegistryBuilder::default().build()
}
