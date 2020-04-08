use super::Component;
use std::collections::HashMap;
use std::sync::Arc;

/// The Healthchecker is the ability to check the health of the system
pub struct Healthchecker {
  _components: HashMap<String, Arc<dyn Component>>,
}

impl Healthchecker {
  /// Construct a new Healthchecker
  pub fn new(components: HashMap<String, Arc<dyn Component>>) -> Self {
    Self {
      _components: components,
    }
  }
}
