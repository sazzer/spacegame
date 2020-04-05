use super::service::PlayerService;
use crate::players::*;

impl PlayerRegistration for PlayerService {
  /// Register a player with the given details
  fn register_player(
    _name: PlayerName,
    _avatar: AvatarUrl,
    _login: Login,
  ) -> Result<PlayerEntity, RegistrationError> {
    todo!()
  }
}
