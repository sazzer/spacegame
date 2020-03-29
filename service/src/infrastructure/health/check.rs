use async_trait::async_trait;

/// The status of some component in the system
pub type Status = Result<(), String>;

/// Trait that any component able to report on it's health can implement
#[async_trait]
pub trait Component: Send + Sync {
  /// Check the health of this component
  async fn check_health(&self) -> Status;
}
