use super::settings::ServiceSettings;
use crate::authentication::{google::GoogleProvider, Provider, ProviderName, ProviderRegistry};
use crate::players::service::PlayerService;
use std::collections::HashMap;
use std::sync::Arc;

/// Create the authentication setup for the service
pub fn create_authentication(
  settings: &ServiceSettings,
  player_service: &PlayerService,
) -> ProviderRegistry {
  let mut providers: HashMap<ProviderName, Arc<dyn Provider>> = HashMap::new();

  if let Some(google_settings) = &settings.google_settings {
    let google_provider = GoogleProvider::new(google_settings.clone(), player_service.clone());
    providers.insert("google".parse().unwrap(), Arc::new(google_provider));
  }

  ProviderRegistry::new(providers)
}
