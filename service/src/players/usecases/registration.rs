use crate::players::*;

/// Errors that can occur when registering a player
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum PlayerRegistrationError {
  #[error("An unknown error occurred")]
  UnknownError,
}

/// Details needed to register a player
pub struct Registration {
  pub name: String,
  pub avatar: Option<String>,
  pub login: PlayerLogin,
}

/// Use case for registering players
pub trait PlayerRegistration {
  /// Attempt to register a player account, or return the existing one if it's already registered
  fn register_player(&self, details: Registration) -> Result<Player, PlayerRegistrationError>;
}
