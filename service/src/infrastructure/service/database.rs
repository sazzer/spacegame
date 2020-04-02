use super::ServiceSettings;
use crate::infrastructure::database::{migrate::migrate_database, Database};

/// Create the database connection to work with
pub async fn create_database(settings: &ServiceSettings) -> Database {
  let database: Database = Database::new(&settings.database_url).await;
  migrate_database(&database).await.unwrap();

  database
}
