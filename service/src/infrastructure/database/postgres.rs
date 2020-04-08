#[cfg(test)]
use faux;

/// The actual database connection
#[cfg_attr(test, faux::create)]
#[cfg_attr(not(test), derive(Clone))]
pub struct Database {}

#[cfg_attr(test, faux::methods)]
impl Database {
  /// Create a new connection to the database
  pub async fn new() -> Self {
    log::info!("Connecting to database");
    Self {}
  }
}

#[cfg(test)]
#[cfg_attr(test, faux::methods)]
impl Clone for Database {
  fn clone(&self) -> Self {
    Self {}
  }
}
