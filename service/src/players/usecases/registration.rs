use crate::players::model::*;

/// Errors that can happen when registering a new user
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum RegistrationError {
  #[error("An unknown error occurred")]
  UnknownError,
}

/// Use case trait for registering players
pub trait PlayerRegistration {
  /// Register a player with the given details
  fn register_player(
    name: PlayerName,
    avatar: AvatarUrl,
    login: Login,
  ) -> Result<PlayerEntity, RegistrationError>;
}
