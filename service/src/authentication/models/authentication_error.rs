#[derive(Debug, PartialEq, thiserror::Error)]
pub enum AuthenticationError {
  #[error("An unknown error occurred")]
  UnknownError,
}
