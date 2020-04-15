use super::Identity;

/// Base struct for all entities to extend
#[derive(Debug, PartialEq, Clone)]
pub struct Entity<I, D> {
  pub identity: Identity<I>,
  pub data: D,
}
