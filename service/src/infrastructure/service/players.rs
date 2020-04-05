use crate::infrastructure::database::Database;
use crate::players::service::PlayerService;

/// Create the Player Service to use
pub fn create_players(database: &Database) -> PlayerService {
  PlayerService::new(database.clone())
}
