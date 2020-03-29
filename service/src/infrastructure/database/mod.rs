use crate::infrastructure::health::*;
use async_trait::async_trait;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;

/// The database connection to work with
#[derive(Clone)]
pub struct Database {
  pool: Pool<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
}

impl Database {
  /// Create a new database connection
  pub async fn new<S>(url: S) -> Self
  where
    S: Into<String>,
  {
    let url = url.into();
    log::info!("Connecting to database: {}", url);

    let config = tokio_postgres::config::Config::from_str(&url).unwrap();
    let manager = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

    let pool = Pool::builder()
      .min_idle(Some(3))
      .max_size(5)
      .connection_timeout(std::time::Duration::from_secs(10))
      .build(manager)
      .await
      .unwrap();

    Database { pool }
  }

  pub async fn ping(&self) {
    let sql = format!("SELECT {:p}", &self);

    log::info!("Pinging database");
    let conn = self.pool.get().await.unwrap();
    conn.simple_query(&sql).await.unwrap();

    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    log::info!("Pinged database");
  }
}

#[async_trait]
impl Component for Database {
  async fn check_health(&self) -> Status {
    log::info!("Pinging database");
    let conn = self
      .pool
      .get()
      .await
      .map_err(|e| format!("Failed to get connection: {}", e))?;
    conn
      .simple_query("SELECT 1")
      .await
      .map_err(|e| format!("Failed to ping database: {}", e))?;
    Ok(())
  }
}
