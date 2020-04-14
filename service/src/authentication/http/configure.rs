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

    cfg.route(
      "/authentication",
      web::get().to(super::list_providers::list_providers),
    );
    cfg.route(
      "/authentication/{provider}",
      web::get().to(super::start::start_authentication),
    );
    cfg.route(
      "/authentication/{provider}/complete",
      web::get().to(super::complete::complete_authentication),
    );
  })
}
