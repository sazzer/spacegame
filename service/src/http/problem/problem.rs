use actix_web::http::StatusCode;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

/// Trait to represent the type of a problem
pub trait ProblemType: Display + Debug {
  /// The actual problem type
  fn error_code(&self) -> &'static str;
}

/// Representation of a Problem expressed in terms of RFC-7807
#[derive(Debug)]
pub struct Problem<T> {
  pub(super) error: T,
  pub(super) status: StatusCode,
  pub(super) detail: Option<String>,
  pub(super) instance: Option<String>,
  pub(super) extra: HashMap<String, Value>,
}

impl<T> Problem<T>
where
  T: ProblemType,
{
  /// Create a new Problem instance
  pub fn new(error: T, status: StatusCode) -> Self {
    Self {
      error: error,
      status: status,
      detail: None,
      instance: None,
      extra: HashMap::new(),
    }
  }

  /// Set the Detail of the Problem instance
  pub fn with_detail(self, detail: String) -> Self {
    Self {
      error: self.error,
      status: self.status,
      detail: Some(detail),
      instance: self.instance,
      extra: self.extra,
    }
  }

  /// Set the Instance of the Problem instance
  pub fn with_instance(self, instance: String) -> Self {
    Self {
      error: self.error,
      status: self.status,
      detail: self.detail,
      instance: Some(instance),
      extra: self.extra,
    }
  }

  /// Set some extra data on the Problem instance
  pub fn with_extra<K, V>(self, key: K, value: V) -> Self
  where
    K: Into<String>,
    V: Serialize,
  {
    let mut extra = self.extra;
    extra.insert(key.into(), serde_json::to_value(value).unwrap());

    Self {
      error: self.error,
      status: self.status,
      detail: self.detail,
      instance: self.instance,
      extra: extra,
    }
  }
}

impl<T> Display for Problem<T>
where
  T: ProblemType,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.error)
  }
}
