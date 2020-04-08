use crate::players::database::PlayerRepository;

/// The Player Service to interact with players
#[derive(Clone)]
pub struct PlayerService {
  repository: PlayerRepository,
}

impl PlayerService {
  /// Create a new Player Service
  pub fn new(repository: PlayerRepository) -> Self {
    Self { repository }
  }
}
