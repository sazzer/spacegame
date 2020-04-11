use crate::authentication::{Provider, ProviderName, ProviderNameParseError, ProviderRegistry};
use actix_http::Response;
use actix_web::{get, web, HttpResponse};

impl From<ProviderNameParseError> for Response {
  fn from(_e: ProviderNameParseError) -> Response {
    HttpResponse::BadRequest().finish()
  }
}

/// Start authentication with the specified provider
#[get("/authentication/{provider}")]
pub async fn start_authentication(
  provider_registry: web::Data<ProviderRegistry>,
  path: web::Path<(String,)>,
) -> Result<Response, Response> {
  let provider_name: ProviderName = path.0.parse()?;
  let provider: &dyn Provider = provider_registry
    .get(&provider_name)
    .ok_or(HttpResponse::NotFound().finish())?;

  log::info!("Starting authentication with provider: {:?}", provider_name);
  Ok(HttpResponse::Ok().finish())
}
