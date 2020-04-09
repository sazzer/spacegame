use super::healthchecker::*;
use actix_http::http::StatusCode;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use std::collections::HashMap;

/// The status of a component or of the entire system
#[derive(Serialize, Debug, PartialEq)]
enum Status {
  /// The status is Healthy
  Healthy,

  /// The status is unhealthy
  Unhealthy,
}

/// API Model to represent the health of a component
#[derive(Serialize, Debug, PartialEq)]
struct ComponentHealthModel {
  status: Status,
  #[serde(skip_serializing_if = "Option::is_none")]
  message: Option<String>,
}

/// API Model to represent the health of the entire system
#[derive(Serialize, Debug, PartialEq)]
struct SystemHealthModel {
  status: Status,
  components: HashMap<String, ComponentHealthModel>,
}

impl From<bool> for Status {
  /// Convert a boolean - the flag of whether a component or the entire system is healthy - into the health status
  fn from(healthy: bool) -> Status {
    if healthy {
      Status::Healthy
    } else {
      Status::Unhealthy
    }
  }
}

impl From<ComponentHealth> for ComponentHealthModel {
  /// Convert the health of a component into the API model
  fn from(health: ComponentHealth) -> ComponentHealthModel {
    ComponentHealthModel {
      status: health.is_healthy().into(),
      message: match health {
        ComponentHealth::Healthy => None,
        ComponentHealth::Unhealthy(message) => Some(message),
      },
    }
  }
}
impl From<SystemHealth> for SystemHealthModel {
  /// Convert the health of the system into the API model
  fn from(health: SystemHealth) -> SystemHealthModel {
    SystemHealthModel {
      status: health.is_healthy().into(),
      components: health
        .components
        .into_iter()
        .map(|(key, component)| (key, component.into()))
        .collect(),
    }
  }
}

#[get("/health")]
pub async fn check_health(healthchecker: web::Data<Healthchecker>) -> impl Responder {
  let health = healthchecker.check_health();

  let status_code = if health.is_healthy() {
    StatusCode::OK
  } else {
    StatusCode::SERVICE_UNAVAILABLE
  };

  let response: SystemHealthModel = health.into();

  HttpResponse::build(status_code).json(response)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::boxed::Box;
  use std::error::Error;

  #[derive(thiserror::Error, Debug)]
  enum TestError {
    #[error("A Test Error")]
    TestEntry,
  }

  #[test]
  fn test_convert_true_to_status() {
    let result: Status = true.into();
    assert_eq!(Status::Healthy, result);
  }

  #[test]
  fn test_convert_false_to_status() {
    let result: Status = false.into();
    assert_eq!(Status::Unhealthy, result);
  }

  #[test]
  fn test_convert_healthy_component_to_model() {
    let input: Result<(), Box<dyn Error>> = Ok(());
    let component: ComponentHealth = input.into();
    let result: ComponentHealthModel = component.into();
    assert_eq!(
      serde_json::to_value(result).unwrap(),
      serde_json::json!(
        {
          "status": "Healthy"
        }
      )
    );
  }

  #[test]
  fn test_convert_unhealthy_component_to_model() {
    let input: Result<(), Box<dyn Error>> = Err(Box::new(TestError::TestEntry));
    let component: ComponentHealth = input.into();
    let result: ComponentHealthModel = component.into();
    assert_eq!(
      serde_json::to_value(result).unwrap(),
      serde_json::json!(
        {
          "status": "Unhealthy",
          "message": "A Test Error"
        }
      )
    );
  }

  #[test]
  fn test_convert_empty_system_to_model() {
    let components = HashMap::new();
    let system = SystemHealth { components };
    let result: SystemHealthModel = system.into();

    assert_eq!(
      serde_json::to_value(result).unwrap(),
      serde_json::json!(
        {
          "status": "Healthy",
          "components": {}
        }
      )
    );
  }

  #[test]
  fn test_convert_healthy_system_to_model() {
    let mut components = HashMap::new();
    components.insert("passing".to_owned(), ComponentHealth::Healthy);
    let system = SystemHealth { components };
    let result: SystemHealthModel = system.into();

    assert_eq!(
      serde_json::to_value(result).unwrap(),
      serde_json::json!(
        {
          "status": "Healthy",
          "components": {
            "passing": {
              "status": "Healthy"
            }
          }
        }
      )
    );
  }

  #[test]
  fn test_convert_unhealthy_system_to_model() {
    let mut components = HashMap::new();
    components.insert(
      "failing".to_owned(),
      ComponentHealth::Unhealthy("Oops".to_owned()),
    );
    let system = SystemHealth { components };
    let result: SystemHealthModel = system.into();

    assert_eq!(
      serde_json::to_value(result).unwrap(),
      serde_json::json!(
        {
          "status": "Unhealthy",
          "components": {
            "failing": {
              "status": "Unhealthy",
              "message": "Oops"
            }
          }
        }
      )
    );
  }

  #[test]
  fn test_convert_mixed_system_to_model() {
    let mut components = HashMap::new();
    components.insert("passing".to_owned(), ComponentHealth::Healthy);
    components.insert(
      "failing".to_owned(),
      ComponentHealth::Unhealthy("Oops".to_owned()),
    );
    let system = SystemHealth { components };
    let result: SystemHealthModel = system.into();

    assert_eq!(
      serde_json::to_value(result).unwrap(),
      serde_json::json!(
        {
          "status": "Unhealthy",
          "components": {
            "passing": {
              "status": "Healthy"
            },
            "failing": {
              "status": "Unhealthy",
              "message": "Oops"
            }
          }
        }
      )
    );
  }
}
