use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representation of the identity of some entity
#[derive(Debug, PartialEq, Clone)]
pub struct Identity<I> {
  pub id: I,
  pub version: Uuid,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}

impl<I> Default for Identity<I>
where
  I: Default,
{
  fn default() -> Self {
    let now = Utc::now();

    Self {
      id: I::default(),
      version: Uuid::default(),
      created: now,
      updated: now,
    }
  }
}
