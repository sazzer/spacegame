use super::server::Server;

/// The actual service to use
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new service
  pub fn new() -> Self {
    log::info!("Building Service");
    let server = Server::new();

    Service { server }
  }

  /// Start the service running
  pub fn start(&self, port: u16) {
    self.server.start(port);
  }
}
