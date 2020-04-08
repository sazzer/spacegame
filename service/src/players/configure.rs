use super::database::PlayerRepository;
use super::service::PlayerService;
use crate::infrastructure::database::Database;

/// Construct a new Player Service with the given Database
pub fn new_player_service(database: Database) -> PlayerService {
  log::info!("Building Player Service");
  let repository = PlayerRepository::new(database);
  let service = PlayerService::new(repository);

  service
}
