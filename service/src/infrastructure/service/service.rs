use super::ServiceSettings;
use crate::infrastructure::server::Server;

/// The actual service to use
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new service
  pub async fn new(settings: ServiceSettings) -> Self {
    log::info!("Building Service");

    let database = super::database::create_database(&settings).await;
    let healthchecks = super::healthchecks::create_healthchecks(&database);
    let player_service = super::players::create_players(&database);
    let authentication = super::authentication::create_authentication(&settings, &player_service);

    // Actually build the Web Server from all of this
    let server = Server::new(vec![
      crate::infrastructure::health::configure::configure_healthchecks(healthchecks),
      crate::authentication::configure::configure_authentication(authentication),
      crate::players::configure::configure_players(player_service),
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
  ) -> Result<crate::infrastructure::server::test::TestResponse, actix_http::error::Error> {
    crate::infrastructure::server::test::run_test(&self.server, req).await
  }
}
