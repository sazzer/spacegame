use super::*;
use actix_web::web;
use std::sync::Arc;

/// Configure the Healthchecks as part of the service
pub fn configure_authentication() -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
  Arc::new(move |cfg| {
    let providers = ProviderRegistry::new();
    cfg.data(providers);
    cfg.service(super::http::list_providers);
  })
}
