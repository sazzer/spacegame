use super::{
  database::{migrate::migrate_database, Database},
  server::Server,
};
use std::collections::HashMap;
use std::sync::Arc;

/// The settings to use to configure the service
#[derive(Debug, Default)]
pub struct ServiceSettings {
  pub database_url: String,
}

/// The actual service to use
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new service
  pub async fn new(settings: ServiceSettings) -> Self {
    log::info!("Building Service");
    let database: Database = Database::new(settings.database_url).await;
    migrate_database(&database).await.unwrap();

    let mut healthchecks: HashMap<String, Arc<dyn crate::infrastructure::health::Component>> =
      HashMap::new();
    healthchecks.insert("database".to_owned(), Arc::new(database.clone()));

    let server = Server::new(vec![
      crate::infrastructure::health::configure::configure_healthchecks(healthchecks),
      crate::authentication::configure::configure_authentication(),
    ]);

    Service { server }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    self.server.start(port).await;
  }

  pub async fn run_test(
    &self,
    req: actix_http::Request,
  ) -> Result<super::server::test::TestResponse, actix_http::error::Error> {
    super::server::test::run_test(&self.server, req).await
  }
}
