use super::Identity;

/// Generic type for some entity model to use
#[derive(Debug, PartialEq)]
pub struct Entity<ID, DATA> {
  pub identity: Identity<ID>,
  pub data: DATA,
}
