use crate::infrastructure::database::Database;

/// Service for interacting with players
#[derive(Clone)]
pub struct PlayerService {
  _database: Database,
}

impl PlayerService {
  /// Construct a new instance of the Player Service
  pub fn new(database: Database) -> PlayerService {
    PlayerService {
      _database: database,
    }
  }
}
