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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_something() {
    let db = crate::infrastructure::database::Database::faux();
    let _sut = PlayerRepository::new(db);
  }
}
