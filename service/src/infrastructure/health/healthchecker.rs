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
pub struct Healthchecker {
  checks: HashMap<String, Arc<dyn Component>>,
}

impl Healthchecker {
  // Construct a new healthchecker from a set of components
  pub fn new(checks: HashMap<String, Arc<dyn Component>>) -> Self {
    Healthchecker { checks }
  }

  // Actually check the health of the system
  pub fn check_health(&self) -> SystemHealth {
    let components: HashMap<String, Status> = self
      .checks
      .iter()
      .map(|(key, value)| (key.to_owned(), value.check_health()))
      .collect();

    let healthy = components
      .iter()
      .map(|(_, value)| value)
      .all(|value| value == &Status::Healthy);

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
  use galvanic_assert::{
    assert_that,
    matchers::{variant::*, *},
  };

  impl Component for String {
    fn check_health(&self) -> Status {
      if "" == self {
        Status::Healthy
      } else {
        Status::Unhealthy(self.to_owned())
      }
    }
  }

  #[test]
  fn test_no_checks() {
    let sut = Healthchecker::new(HashMap::new());
    let result = sut.check_health();

    assert_that!(&result.status, eq(SystemStatus::Healthy));
    assert_that!(&result.components.len(), eq(0));
  }

  #[test]
  fn test_passing_check() {
    let mut checks = HashMap::new();
    checks.insert(
      "passing".to_owned(),
      Arc::new("".to_owned()) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(checks);
    let result = sut.check_health();

    assert_that!(&result.status, eq(SystemStatus::Healthy));
    assert_that!(&result.components.len(), eq(1));
    assert_that!(
      &result.components.get("passing"),
      maybe_some(eq(&Status::Healthy))
    );
  }

  #[test]
  fn test_failing_check() {
    let mut checks = HashMap::new();
    checks.insert(
      "failing".to_owned(),
      Arc::new("Failing".to_owned()) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(checks);
    let result = sut.check_health();

    assert_that!(&result.status, eq(SystemStatus::Unhealthy));
    assert_that!(&result.components.len(), eq(1));
    let failing_message = Status::Unhealthy("Failing".to_owned());
    assert_that!(
      &result.components.get("failing"),
      maybe_some(eq(&failing_message))
    );
  }

  #[test]
  fn test_mixed_check() {
    let mut checks = HashMap::new();
    checks.insert(
      "passing".to_owned(),
      Arc::new("".to_owned()) as Arc<dyn Component>,
    );
    checks.insert(
      "failing".to_owned(),
      Arc::new("Failing".to_owned()) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(checks);
    let result = sut.check_health();

    assert_that!(&result.status, eq(SystemStatus::Unhealthy));
    assert_that!(&result.components.len(), eq(2));
    assert_that!(
      &result.components.get("passing"),
      maybe_some(eq(&Status::Healthy))
    );
    let failing_message = Status::Unhealthy("Failing".to_owned());
    assert_that!(
      &result.components.get("failing"),
      maybe_some(eq(&failing_message))
    );
  }
}
