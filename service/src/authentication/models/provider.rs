use async_trait::async_trait;

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
}
