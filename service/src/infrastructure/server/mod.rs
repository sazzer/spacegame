/// The actual web server to use
pub struct Server {}

impl Server {
  /// Construct a new web server
  pub fn new() -> Self {
    Server {}
  }

  /// Start the service running
  pub fn start(&self, port: u16) {
    log::info!("Starting web server on port {}", port);
  }
}
