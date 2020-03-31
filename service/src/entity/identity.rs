use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The identity of some entity
#[derive(Debug, PartialEq)]
pub struct Identity<T> {
  pub id: T,
  pub version: Uuid,
  pub created: DateTime<Utc>,
  pub updated: DateTime<Utc>,
}

impl<T> Default for Identity<T>
where
  T: Default,
{
  fn default() -> Self {
    Self {
      id: Default::default(),
      version: Default::default(),
      created: Utc::now(),
      updated: Utc::now(),
    }
  }
}
