use std::sync::Arc;

/// The actual Service that does all the work
pub struct Service {
  server: crate::infrastructure::server::Server,
}

impl Service {
  /// Create a new Service
  pub fn new() -> Self {
    log::info!("Creating Service");

    let db = crate::infrastructure::database::Database::new();

    let _player_service = crate::players::configure::new_player_service(db.clone());

    let _health_checker = crate::infrastructure::health::builder::HealthCheckerBuilder::new()
      .with_component("db", Arc::new(db.clone()))
      .build();

    let server = crate::infrastructure::server::Server::new();

    Self { server: server }
  }

  /// Start the service running
  pub fn start(&self) {
    log::info!("Starting Service");
    self.server.start();
  }
}
