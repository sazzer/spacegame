/// The actual HTTP Server
pub struct Server {}

impl Server {
  /// Create a new Server
  pub fn new() -> Self {
    log::info!("Creating HTTP Server");

    Self {}
  }

  /// Start the service running
  pub fn start(&self) {
    log::info!("Starting HTTP Server");
  }
}
