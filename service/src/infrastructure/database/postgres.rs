#[cfg(test)]
use faux;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;

/// The actual database connection
#[cfg_attr(test, faux::create)]
#[cfg_attr(not(test), derive(Clone))]
pub struct Database {
  pool: Pool<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
}

#[cfg_attr(test, faux::methods)]
impl Database {
  /// Create a new connection to the database
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
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Clone for Database {
  fn clone(&self) -> Self {
    Self {
      pool: self.pool.clone(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::testing::database::TestDatabase;

  #[actix_rt::test]
  async fn test_connect_to_database() {
    let test_database = TestDatabase::new();
    Database::new(test_database.url).await;
  }
}
