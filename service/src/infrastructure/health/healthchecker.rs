use super::check::*;
use std::collections::HashMap;
use std::sync::Arc;

// The overall status of the system health
#[derive(Debug, PartialEq)]
pub enum SystemStatus {
  Healthy,
  Unhealthy,
}

// The overall system health
#[derive(Debug, PartialEq)]
pub struct SystemHealth {
  pub status: SystemStatus,
  pub components: HashMap<String, Status>,
}

// Mechanism to check the health of the system
#[derive(Clone)]
pub struct Healthchecker {
  checks: HashMap<String, Arc<dyn Component>>,
}

impl Healthchecker {
  // Construct a new healthchecker from a set of components
  pub fn new(checks: HashMap<String, Arc<dyn Component>>) -> Self {
    Healthchecker { checks }
  }

  // Actually check the health of the system
  pub async fn check_health(&self) -> SystemHealth {
    let mut components: HashMap<String, Status> = HashMap::new();

    for (k, v) in self.checks.iter() {
      let result = v.check_health().await;
      components.insert(k.to_owned(), result);
    }

    let healthy = components
      .iter()
      .map(|(_, value)| value)
      .all(|value| value.is_ok());

    SystemHealth {
      status: if healthy {
        SystemStatus::Healthy
      } else {
        SystemStatus::Unhealthy
      },
      components,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use async_trait::async_trait;
  use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
  };

  #[async_trait]
  impl Component for Status {
    async fn check_health(&self) -> Status {
      self.clone()
    }
  }

  #[actix_rt::test]
  async fn test_no_checks() {
    let sut = Healthchecker::new(HashMap::new());
    let result = sut.check_health().await;

    assert_that!(&result.status, eq(SystemStatus::Healthy));
    assert_that!(&result.components.len(), eq(0));
  }

  #[actix_rt::test]
  async fn test_passing_check() {
    let mut checks = HashMap::new();
    checks.insert(
      "passing".to_owned(),
      Arc::new(Ok(()).to_owned()) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(checks);
    let result = sut.check_health().await;

    assert_that!(&result.status, eq(SystemStatus::Healthy));
    assert_that!(&result.components.len(), eq(1));
    assert_that!(&result.components.get("passing"), maybe_some(eq(&Ok(()))));
  }

  #[actix_rt::test]
  async fn test_failing_check() {
    let mut checks = HashMap::new();
    checks.insert(
      "failing".to_owned(),
      Arc::new(Err("Failing".to_owned())) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(checks);
    let result = sut.check_health().await;

    assert_that!(&result.status, eq(SystemStatus::Unhealthy));
    assert_that!(&result.components.len(), eq(1));
    let failing_message = Err("Failing".to_owned());
    assert_that!(
      &result.components.get("failing"),
      maybe_some(eq(&failing_message))
    );
  }

  #[actix_rt::test]
  async fn test_mixed_check() {
    let mut checks = HashMap::new();
    checks.insert("passing".to_owned(), Arc::new(Ok(())) as Arc<dyn Component>);
    checks.insert(
      "failing".to_owned(),
      Arc::new(Err("Failing".to_owned())) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(checks);
    let result = sut.check_health().await;

    assert_that!(&result.status, eq(SystemStatus::Unhealthy));
    assert_that!(&result.components.len(), eq(2));
    assert_that!(&result.components.get("passing"), maybe_some(eq(&Ok(()))));
    let failing_message = Err("Failing".to_owned());
    assert_that!(
      &result.components.get("failing"),
      maybe_some(eq(&failing_message))
    );
  }
}
