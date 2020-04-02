use super::*;
use actix_web::web;
use std::sync::Arc;

/// Configure the Healthchecks as part of the service
pub fn configure_authentication(
  providers: ProviderRegistry,
) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
  let providers = providers.clone();

  Arc::new(move |cfg| {
    let providers = providers.clone();

    cfg.data(providers);
    cfg.service(super::http::list_providers);
  })
}
