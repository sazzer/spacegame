use crate::authentication::ProviderName;
use crate::entity::*;
use uuid::Uuid;

/// Representation of the ID of a Player
pub type PlayerID = Uuid;

/// Representation of the name of a player
pub type PlayerName = String;

/// Representation of the URL to the avatar of a player
pub type AvatarUrl = String;

/// Representation of the ID of a Player at a login provider
pub type ProviderID = String;

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
