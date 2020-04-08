use super::{Component, Healthchecker};
use std::collections::HashMap;
use std::sync::Arc;

/// Builder to help building the healthchecker
pub struct HealthCheckerBuilder {
  components: HashMap<String, Arc<dyn Component>>,
}

impl HealthCheckerBuilder {
  /// Create a new Builder
  pub fn new() -> Self {
    Self {
      components: HashMap::new(),
    }
  }

  /// Add a new component to the builder
  pub fn with_component<S>(mut self, name: S, component: Arc<dyn Component>) -> Self
  where
    S: Into<String>,
  {
    let name = name.into();

    self.components.insert(name, component);
    self
  }

  /// Build the healthchecker
  pub fn build(self) -> Healthchecker {
    log::info!("Building Healthchecker");
    Healthchecker::new(self.components)
  }
}
