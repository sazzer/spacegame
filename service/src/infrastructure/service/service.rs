use std::sync::Arc;

/// The actual Service that does all the work
pub struct Service {
  server: crate::infrastructure::server::Server,
}

/// Settings needed to run the service
pub struct Settings {
  pub database_url: String,
}

impl Service {
  /// Create a new Service
  pub async fn new(settings: Settings) -> Self {
    log::info!("Creating Service");

    let db = crate::infrastructure::database::Database::new(settings.database_url).await;
    crate::infrastructure::database::migrate::migrate_database(&db)
      .await
      .unwrap();

    let _player_service = crate::players::configure::new_player_service(db.clone());

    let healthchecker = crate::infrastructure::health::builder::HealthCheckerBuilder::new()
      .with_component("db", Arc::new(db.clone()))
      .build();

    let server = crate::infrastructure::server::Server::new(vec![
      crate::infrastructure::health::configure::configure_healthchecks(healthchecker),
    ]);

    Self { server: server }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    log::info!("Starting Service");
    self.server.start(port).await;
  }
}
