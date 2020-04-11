use crate::authentication::ProviderRegistry;
use actix_web::web;
use std::sync::Arc;

/// Configure the Authentication handlers as part of the service
pub fn configure_authentication(
  provider_registry: ProviderRegistry,
) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
  let provider_registry = provider_registry.clone();
  Arc::new(move |cfg| {
    cfg.data(provider_registry.clone());
    cfg.service(super::list_providers::list_providers);
    cfg.service(super::start::start_authentication);
  })
}
