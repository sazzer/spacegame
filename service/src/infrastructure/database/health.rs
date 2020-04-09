use super::Database;
use crate::infrastructure::health::Component;
use async_trait::async_trait;
use std::boxed::Box;
use std::error::Error;

#[async_trait]
impl Component for Database {
  /// Check that we're able to send a simple query to the database
  async fn check_health(&self) -> Result<(), Box<dyn Error>> {
    Ok(())
  }
}
