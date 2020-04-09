use super::Database;
use crate::infrastructure::health::Component;
use async_trait::async_trait;
use std::boxed::Box;
use std::error::Error;

#[async_trait]
impl Component for Database {
  /// Check that we're able to send a simple query to the database
  async fn check_health(&self) -> Result<(), Box<dyn Error>> {
    let connection = self.checkout().await?;
    connection.simple_query("SELECT 1").await?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::infrastructure::database::Database;
  use crate::testing::database::TestDatabase;

  #[actix_rt::test]
  async fn test_database_health() {
    let test_database = TestDatabase::new();
    let database = Database::new(test_database.url).await;

    let result = database.check_health().await;
    assert_eq!(true, result.is_ok());
  }
}
