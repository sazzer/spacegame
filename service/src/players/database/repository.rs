use crate::infrastructure::database::Database;

/// Repository for accessing player records
#[derive(Clone)]
pub struct PlayerRepository {
  db: Database,
}

impl PlayerRepository {
  /// Construct a new Player Repository
  pub fn new(db: Database) -> Self {
    Self { db }
  }
}
