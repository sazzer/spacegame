/// The actual database connection
#[derive(Clone)]
pub struct Database {}

impl Database {
  /// Create a new connection to the database
  pub fn new() -> Self {
    log::info!("Connecting to database");
    Self {}
  }
}
