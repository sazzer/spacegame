use super::healthchecker::Healthchecker;
use actix_web::web;
use std::sync::Arc;

/// Configure the Healthchecks as part of the service
pub fn configure_healthchecks(
  healthchecker: Healthchecker,
) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
  let healthchecker = healthchecker.clone();
  Arc::new(move |cfg| {
    cfg.data(healthchecker.clone());

    cfg.route(
      "/health",
      web::get().to(super::http::check_health::check_health),
    );
  })
}
