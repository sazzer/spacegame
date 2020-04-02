use super::settings::ServiceSettings;
use crate::authentication::{google::GoogleProvider, Provider, ProviderName, ProviderRegistry};
use std::collections::HashMap;
use std::sync::Arc;

/// Create the authentication setup for the service
pub fn create_authentication(settings: &ServiceSettings) -> ProviderRegistry {
  let mut providers: HashMap<ProviderName, Arc<dyn Provider + Send + Sync>> = HashMap::new();

  if let Some(google_settings) = &settings.google_settings {
    let google_provider = GoogleProvider::new(google_settings.clone());
    providers.insert("google".parse().unwrap(), Arc::new(google_provider));
  }

  ProviderRegistry::new(providers)
}
