use super::{healthchecker::*, *};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use serde::Serialize;
use std::collections::HashMap;

const STATUS_HEALTHY: &'static str = "HEALTHY";
const STATUS_UNHEALTHY: &'static str = "UNHEALTHY";

#[derive(Serialize)]
struct HealthcheckComponentResponse {
  status: &'static str,
  message: Option<String>,
}

#[derive(Serialize)]
struct HealthcheckResponse {
  status: &'static str,
  components: HashMap<String, HealthcheckComponentResponse>,
}

impl From<Status> for HealthcheckComponentResponse {
  fn from(status: Status) -> Self {
    match status {
      Status::Healthy => HealthcheckComponentResponse {
        status: STATUS_HEALTHY,
        message: None,
      },
      Status::Unhealthy(msg) => HealthcheckComponentResponse {
        status: STATUS_UNHEALTHY,
        message: Some(msg.to_owned()),
      },
    }
  }
}

pub async fn check_health(healthchecker: web::Data<Healthchecker>) -> impl Responder {
  let health = healthchecker.check_health().await;

  let components = health
    .components
    .into_iter()
    .map(|(key, status)| (key, HealthcheckComponentResponse::from(status)))
    .collect();
  let response = HealthcheckResponse {
    status: match health.status {
      SystemStatus::Healthy => STATUS_HEALTHY,
      SystemStatus::Unhealthy => STATUS_UNHEALTHY,
    },
    components,
  };
  let status_code = match health.status {
    SystemStatus::Healthy => StatusCode::OK,
    SystemStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
  };
  HttpResponse::build(status_code).json(response)
}
