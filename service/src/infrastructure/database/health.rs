use super::Database;
use crate::infrastructure::health::*;
use async_trait::async_trait;

#[async_trait]
impl Component for Database {
  async fn check_health(&self) -> Status {
    log::info!("Pinging database");
    let conn = self
      .checkout()
      .await
      .map_err(|e| format!("Failed to get connection: {}", e))?;
    conn
      .simple_query("SELECT 1")
      .await
      .map_err(|e| format!("Failed to ping database: {}", e))?;
    Ok(())
  }
}
