use super::*;
use crate::authentication::ProviderName;
use crate::entity::*;

/// Representation of a single login at a single login provider
pub struct Login {
  pub provider: ProviderName,
  pub provider_id: ProviderID,
  pub display_name: String,
}

/// The data that makes up a Player entity
pub struct PlayerData {
  pub name: PlayerName,
  pub avatar: AvatarUrl,
  pub logins: Vec<Login>,
}

/// The actual Player entity
pub type PlayerEntity = Entity<PlayerID, PlayerData>;
