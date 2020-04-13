use super::model::*;
use crate::infrastructure::health::healthchecker::*;
use actix_web::web;

/// Actix handler to check the health of the system
pub async fn check_health(healthchecker: web::Data<Healthchecker>) -> SystemHealthModel {
  healthchecker.check_health().await.into()
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::infrastructure::health::{Component, TestComponent};
  use std::collections::HashMap;
  use std::sync::Arc;

  #[actix_rt::test]
  async fn test_check_health() {
    let mut components = HashMap::new();
    components.insert(
      "passing".to_owned(),
      Arc::new(TestComponent::Healthy) as Arc<dyn Component>,
    );
    components.insert(
      "failing".to_owned(),
      Arc::new(TestComponent::Unhealthy) as Arc<dyn Component>,
    );
    let healthchecker = Healthchecker::new(components);

    let result = check_health(web::Data::new(healthchecker)).await;

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
              "message": "An Error"
            }
          }
        }
      )
    );
  }
}
