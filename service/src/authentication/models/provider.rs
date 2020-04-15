use super::AuthenticationError;
use crate::players::Player;
use async_trait::async_trait;
use std::collections::HashMap;

/// Details needed to start authentication with a provider
pub struct StartAuthentication {
  pub redirect_url: String,
  pub nonce: Option<String>,
}

/// Trait representing any provider that can authenticate a user with an external system
#[async_trait]
pub trait Provider: Send + Sync {
  /// Start authentication with the provider
  fn start(&self) -> StartAuthentication;

  /// Complete the authentication process, returning the Player that has just authenticated
  async fn complete(&self, params: HashMap<String, String>) -> Result<Player, AuthenticationError>;
}
