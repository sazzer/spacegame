pub use super::http::configure::configure_authentication;
use super::ProviderRegistry;
use super::ProviderRegistryBuilder;
use std::sync::Arc;

/// Configure the provider registry for authentiction
pub fn configure_provider_registry(
  google_settings: super::google::GoogleSettings,
) -> ProviderRegistry {
  let mut builder = ProviderRegistryBuilder::default();

  log::info!("Configuring Google Provider: {:?}", google_settings);
  if google_settings.is_configured() {
    let google = super::google::GoogleProvider::new(google_settings);
    builder = builder.with_provider("google".parse().unwrap(), Arc::new(google));
  }

  builder.build()
}
