use crate::authentication::ProviderNameParseError;
use crate::http::problem::{Problem, ProblemType};
use actix_web::http::StatusCode;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AuthenticationProblem {
  #[error("Provider name is invalid")]
  InvalidProviderName,

  #[error("Requested provider is not known")]
  UnknownProvider,
}

impl ProblemType for AuthenticationProblem {
  fn error_code(&self) -> &'static str {
    match self {
      AuthenticationProblem::InvalidProviderName => {
        "tag:spacegame,2020:authentication/problems/invalid-provider-name"
      }
      AuthenticationProblem::UnknownProvider => {
        "tag:spacegame,2020:authentication/problems/unknown-provider"
      }
    }
  }
}

impl From<ProviderNameParseError> for Problem<AuthenticationProblem> {
  fn from(_e: ProviderNameParseError) -> Problem<AuthenticationProblem> {
    Problem::new(
      AuthenticationProblem::InvalidProviderName,
      StatusCode::BAD_REQUEST,
    )
  }
}
