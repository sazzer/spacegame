use super::problem::AuthenticationProblem;
use crate::authentication::{Provider, ProviderName, ProviderNameParseError, ProviderRegistry};
use crate::http::problem::Problem;
use actix_http::Response;
use actix_web::{get, http::StatusCode, web, HttpResponse};

impl From<ProviderNameParseError> for Problem<AuthenticationProblem> {
  fn from(_e: ProviderNameParseError) -> Problem<AuthenticationProblem> {
    Problem::new(
      AuthenticationProblem::InvalidProviderName,
      StatusCode::BAD_REQUEST,
    )
  }
}

/// Start authentication with the specified provider
#[get("/authentication/{provider}")]
pub async fn start_authentication(
  provider_registry: web::Data<ProviderRegistry>,
  path: web::Path<(String,)>,
) -> Result<Response, Problem<AuthenticationProblem>> {
  let provider_name: ProviderName = path.0.parse()?;
  let provider: &dyn Provider = provider_registry.get(&provider_name).ok_or(Problem::new(
    AuthenticationProblem::UnknownProvider,
    StatusCode::NOT_FOUND,
  ))?;

  log::info!("Starting authentication with provider: {:?}", provider_name);
  Ok(HttpResponse::Ok().finish())
}
