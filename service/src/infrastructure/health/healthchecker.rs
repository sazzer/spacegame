use super::Component;
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

/// The health of a single component
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
  /// The component is healthy
  Healthy,

  /// The component is unhealthy. The associated String is an error message
  Unhealthy(String),
}

impl ComponentHealth {
  /// Determine if this component is healthy or not
  pub fn is_healthy(&self) -> bool {
    self == &ComponentHealth::Healthy
  }
}

impl From<Result<(), Box<dyn Error>>> for ComponentHealth {
  /// Convert a Result containing an Error into a Component Health
  fn from(e: Result<(), Box<dyn Error>>) -> ComponentHealth {
    match e {
      Ok(_) => ComponentHealth::Healthy,
      Err(e) => ComponentHealth::Unhealthy(format!("{}", e)),
    }
  }
}

/// The overall health of the system
pub struct SystemHealth {
  /// The health of all the components in the system
  pub components: HashMap<String, ComponentHealth>,
}

impl SystemHealth {
  /// Determine if the system is healthy or not
  pub fn is_healthy(&self) -> bool {
    self
      .components
      .iter()
      .all(|(_, component)| component.is_healthy())
  }
}

/// The Healthchecker is the ability to check the health of the system
#[derive(Clone)]
pub struct Healthchecker {
  /// The components to check
  components: HashMap<String, Arc<dyn Component>>,
}

impl Healthchecker {
  /// Construct a new Healthchecker
  pub fn new(components: HashMap<String, Arc<dyn Component>>) -> Self {
    Self {
      components: components,
    }
  }

  /// Check the health of the entire system
  pub fn check_health(&self) -> SystemHealth {
    SystemHealth {
      components: self
        .components
        .iter()
        .map(|(key, component)| (key.clone(), component.check_health().into()))
        .collect(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::TestComponent;
  use super::*;

  #[derive(thiserror::Error, Debug)]
  enum TestError {
    #[error("A Test Error")]
    TestEntry,
  }

  #[test]
  fn test_convert_ok_to_component_health() {
    let input: Result<(), Box<dyn Error>> = Ok(());
    let result: ComponentHealth = input.into();
    assert_eq!(ComponentHealth::Healthy, result);
  }

  #[test]
  fn test_convert_err_to_component_health() {
    let input: Result<(), Box<dyn Error>> = Err(Box::new(TestError::TestEntry));
    let result: ComponentHealth = input.into();
    assert_eq!(
      ComponentHealth::Unhealthy("A Test Error".to_owned()),
      result
    );
  }

  #[test]
  fn test_healthy_component_is_healthy() {
    assert_eq!(true, ComponentHealth::Healthy.is_healthy());
  }

  #[test]
  fn test_unhealthy_component_is_not_healthy() {
    assert_eq!(
      false,
      ComponentHealth::Unhealthy("Oops".to_owned()).is_healthy()
    );
  }

  #[test]
  fn test_empty_system_is_healthy() {
    let components = HashMap::new();
    let sut = SystemHealth { components };
    assert_eq!(true, sut.is_healthy());
  }

  #[test]
  fn test_healthy_system_is_healthy() {
    let mut components = HashMap::new();
    components.insert("passing".to_owned(), ComponentHealth::Healthy);
    let sut = SystemHealth { components };
    assert_eq!(true, sut.is_healthy());
  }

  #[test]
  fn test_unhealthy_system_is_not_healthy() {
    let mut components = HashMap::new();
    components.insert(
      "failing".to_owned(),
      ComponentHealth::Unhealthy("Oops".to_owned()),
    );
    let sut = SystemHealth { components };
    assert_eq!(false, sut.is_healthy());
  }

  #[test]
  fn test_mixed_system_is_not_healthy() {
    let mut components = HashMap::new();
    components.insert("passing".to_owned(), ComponentHealth::Healthy);
    components.insert(
      "failing".to_owned(),
      ComponentHealth::Unhealthy("Oops".to_owned()),
    );
    let sut = SystemHealth { components };
    assert_eq!(false, sut.is_healthy());
  }

  #[test]
  fn test_healthchecker_no_components_is_healthy() {
    let components = HashMap::new();
    let sut = Healthchecker::new(components);

    let result = sut.check_health();
    assert_eq!(true, result.is_healthy());
    assert_eq!(0, result.components.len());
  }

  #[test]
  fn test_healthchecker_healthy_component_is_healthy() {
    let mut components = HashMap::new();
    components.insert(
      "passing".to_owned(),
      Arc::new(TestComponent::Healthy) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(components);

    let result = sut.check_health();
    assert_eq!(true, result.is_healthy());
    assert_eq!(1, result.components.len());
    assert_eq!(
      Some(&ComponentHealth::Healthy),
      result.components.get(&"passing".to_owned())
    );
  }

  #[test]
  fn test_healthchecker_unhealthy_component_is_unhealthy() {
    let mut components = HashMap::new();
    components.insert(
      "failing".to_owned(),
      Arc::new(TestComponent::Unhealthy) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(components);

    let result = sut.check_health();
    assert_eq!(false, result.is_healthy());
    assert_eq!(1, result.components.len());
    assert_eq!(
      Some(&ComponentHealth::Unhealthy("An Error".to_owned())),
      result.components.get(&"failing".to_owned())
    );
  }

  #[test]
  fn test_healthchecker_mixed_components_are_unhealthy() {
    let mut components = HashMap::new();
    components.insert(
      "passing".to_owned(),
      Arc::new(TestComponent::Healthy) as Arc<dyn Component>,
    );
    components.insert(
      "failing".to_owned(),
      Arc::new(TestComponent::Unhealthy) as Arc<dyn Component>,
    );
    let sut = Healthchecker::new(components);

    let result = sut.check_health();
    assert_eq!(false, result.is_healthy());
    assert_eq!(2, result.components.len());
    assert_eq!(
      Some(&ComponentHealth::Unhealthy("An Error".to_owned())),
      result.components.get(&"failing".to_owned())
    );
    assert_eq!(
      Some(&ComponentHealth::Healthy),
      result.components.get(&"passing".to_owned())
    );
  }
}
