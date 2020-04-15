use super::PlayerService;
use crate::players::*;
use async_trait::async_trait;

#[async_trait]
impl PlayerRegistration for PlayerService {
  /// Attempt to register a player account, or return the existing one if it's already registered
  async fn register_player(
    &self,
    details: Registration,
  ) -> Result<Player, PlayerRegistrationError> {
    log::info!("Registering player: {:?}", details);
    todo!()
  }
}
