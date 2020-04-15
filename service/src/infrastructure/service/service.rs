use std::sync::Arc;

/// The actual Service that does all the work
pub struct Service {
  server: crate::infrastructure::server::Server,
}

/// Settings needed to run the service
pub struct Settings {
  pub database_url: String,

  pub google_settings: crate::authentication::google::GoogleSettings,
}

impl Service {
  /// Create a new Service
  pub async fn new(settings: Settings) -> Self {
    log::info!("Creating Service");

    let db = crate::infrastructure::database::Database::new(settings.database_url).await;
    crate::infrastructure::database::migrate::migrate_database(&db)
      .await
      .unwrap();

    let player_service = crate::players::configure::new_player_service(db.clone());

    let provider_registry = crate::authentication::configure::configure_provider_registry(
      player_service,
      settings.google_settings,
    );

    let healthchecker = crate::infrastructure::health::builder::HealthCheckerBuilder::new()
      .with_component("db", Arc::new(db.clone()))
      .build();

    let server = crate::infrastructure::server::Server::new(vec![
      crate::infrastructure::health::configure::configure_healthchecks(healthchecker),
      crate::authentication::configure::configure_authentication(provider_registry),
    ]);

    Self { server: server }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    log::info!("Starting Service");
    self.server.start(port).await;
  }

  /// Actually make a test request against the service
  pub async fn test_request(
    &self,
    req: actix_http::Request,
  ) -> Result<crate::infrastructure::server::testing::TestResponse, actix_http::error::Error> {
    crate::infrastructure::server::testing::run_test(&self.server, req).await
  }
}
