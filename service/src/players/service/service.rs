use crate::players::database::PlayerRepository;
#[cfg(test)]
use faux;

/// The Player Service to interact with players
#[cfg_attr(test, faux::create)]
#[cfg_attr(not(test), derive(Clone))]
pub struct PlayerService {
  pub(super) repository: PlayerRepository,
}

#[cfg_attr(test, faux::methods)]
impl PlayerService {
  /// Create a new Player Service
  pub fn new(repository: PlayerRepository) -> Self {
    Self { repository }
  }
}
