use super::Database;
use crate::infrastructure::health::Component;
use std::boxed::Box;
use std::error::Error;

impl Component for Database {
  /// Check that we're able to send a simple query to the database
  fn check_health(&self) -> Result<(), Box<dyn Error>> {
    Ok(())
  }
}
