use async_trait::async_trait;

/// The status of some component in the system
#[derive(Debug, PartialEq)]
pub enum Status {
  /// The component is healthy
  Healthy,
  /// The component is unhealthy
  Unhealthy(String),
}

/// Trait that any component able to report on it's health can implement
#[async_trait]
pub trait Component: Send + Sync {
  /// Check the health of this component
  async fn check_health(&self) -> Status;
}

impl<S, E> From<Result<S, E>> for Status
where
  E: std::fmt::Display,
{
  fn from(r: Result<S, E>) -> Status {
    match r {
      Ok(_) => Status::Healthy,
      Err(e) => Status::Unhealthy(format!("{}", e)),
    }
  }
}
