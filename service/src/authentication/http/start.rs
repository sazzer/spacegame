use super::problem::AuthenticationProblem;
use crate::authentication::{Provider, ProviderName, ProviderNameParseError, ProviderRegistry};
use crate::http::problem::Problem;
use actix_http::Response;
use actix_web::{
  http::{header, Cookie, StatusCode},
  web, HttpResponse,
};

impl From<ProviderNameParseError> for Problem<AuthenticationProblem> {
  fn from(_e: ProviderNameParseError) -> Problem<AuthenticationProblem> {
    Problem::new(
      AuthenticationProblem::InvalidProviderName,
      StatusCode::BAD_REQUEST,
    )
  }
}

/// Start authentication with the specified provider
pub async fn start_authentication(
  provider_registry: web::Data<ProviderRegistry>,
  path: web::Path<(String,)>,
) -> Result<Response, Problem<AuthenticationProblem>> {
  let provider_name: ProviderName = path.0.parse()?;
  let provider: &dyn Provider = provider_registry.get(&provider_name).ok_or(
    Problem::new(
      AuthenticationProblem::UnknownProvider,
      StatusCode::NOT_FOUND,
    )
    .with_extra("provider", &provider_name),
  )?;

  log::info!("Starting authentication with provider: {:?}", provider_name);

  let start_details = provider.start();

  Ok(
    HttpResponse::Found()
      .header(header::LOCATION, start_details.redirect_url)
      .if_some(start_details.nonce, |nonce, res| {
        res.cookie(
          Cookie::build("spacegame_authentication", nonce)
            .http_only(true)
            .finish(),
        );
      })
      .finish(),
  )
}
