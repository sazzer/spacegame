use super::{healthchecker::Healthchecker, Component};
use actix_web::web;
use std::collections::HashMap;
use std::sync::Arc;

/// Configure the Healthchecks as part of the service
pub fn configure_healthchecks(
  checks: HashMap<String, Arc<dyn Component>>,
) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
  Arc::new(move |cfg| {
    let healthchecker = Healthchecker::new(checks.clone());
    cfg.data(healthchecker);
    cfg.service(super::http::check_health);
  })
}
