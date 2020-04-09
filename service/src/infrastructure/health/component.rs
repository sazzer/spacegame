use async_trait::async_trait;
use std::boxed::Box;
use std::error::Error;

/// Trait that anything able to check it's own health is able to implement
#[async_trait]
pub trait Component: Send + Sync {
  /// Check the health of this component, returning whatever error occurred if it was unhealthy
  async fn check_health(&self) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
#[derive(Debug, thiserror::Error, Clone)]
pub enum TestComponent {
  #[error("Not an Error")]
  Healthy,
  #[error("An Error")]
  Unhealthy,
}

#[cfg(test)]
#[async_trait]
impl Component for TestComponent {
  async fn check_health(&self) -> Result<(), Box<dyn Error>> {
    match self {
      TestComponent::Healthy => Ok(()),
      TestComponent::Unhealthy => Err(Box::new(self.clone())),
    }
  }
}
