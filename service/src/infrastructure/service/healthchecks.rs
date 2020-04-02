use crate::infrastructure::database::Database;
use crate::infrastructure::health::healthchecker::Healthchecker;
use std::collections::HashMap;
use std::sync::Arc;

/// Create the healthchecks for the service
pub fn create_healthchecks(database: &Database) -> Healthchecker {
  let mut healthchecks: HashMap<String, Arc<dyn crate::infrastructure::health::Component>> =
    HashMap::new();
  healthchecks.insert("database".to_owned(), Arc::new(database.clone()));

  Healthchecker::new(healthchecks)
}
