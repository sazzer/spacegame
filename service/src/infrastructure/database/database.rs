use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;
use thiserror::Error;

/// The database connection to work with
#[derive(Clone)]
pub struct Database {
  pool: Pool<PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
}

#[derive(Error, Debug)]
pub enum DatabaseError {
  #[error("Error checking out connection: {0}")]
  CheckoutError(String),
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

  /// Check out a connection with which we can send queries to the database
  pub async fn checkout(
    &self,
  ) -> Result<
    PooledConnection<'_, PostgresConnectionManager<tokio_postgres::tls::NoTls>>,
    DatabaseError,
  > {
    self
      .pool
      .get()
      .await
      .map_err(|e| DatabaseError::CheckoutError(format!("{}", e)))
  }
}
