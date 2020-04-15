use super::*;
use crate::authentication::ProviderName;
use crate::entity::*;

/// Details of an external login for a player
#[derive(Debug, PartialEq, Clone)]
pub struct PlayerLogin {
  pub provider_name: ProviderName,
  pub provider_id: String,
  pub display_name: String,
}

/// Data to represent a player
#[derive(Debug, PartialEq, Clone)]
pub struct PlayerData {
  pub name: String,
  pub avatar: String,
  pub logins: Vec<PlayerLogin>,
}

/// The actual Player entity
pub type Player = Entity<PlayerId, PlayerData>;
