use super::problem::AuthenticationProblem;
use crate::authentication::{Provider, ProviderName, ProviderRegistry};
use crate::http::problem::Problem;
use actix_http::Response;
use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse};
use std::collections::HashMap;

/// Complete authentication with the specified provider
pub async fn complete_authentication(
  req: HttpRequest,
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

  let params: HashMap<String, String> = req
    .uri()
    .query()
    .map(|v| {
      url::form_urlencoded::parse(v.as_bytes())
        .into_owned()
        .collect()
    })
    .unwrap_or_else(HashMap::new);

  log::info!(
    "Completing authentication with provider {:?} and params {:?}",
    provider_name,
    params
  );

  provider.complete(params).await;

  Ok(HttpResponse::Ok().finish())
}
