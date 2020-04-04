use crate::authentication::*;
use actix_web::{get, http::Cookie, web, HttpResponse};
use std::str::FromStr;

/// HTTP Handler to start authentication against a specific provider
#[get("/authentication/{provider}")]
pub async fn start_authentication(
  path: web::Path<(String,)>,
  provider_registry: web::Data<ProviderRegistry>,
) -> Result<HttpResponse, HttpResponse> {
  let provider_name = ProviderName::from_str(path.0.as_ref())
    .map_err(|_e| HttpResponse::BadRequest().body("Invalid Provider Name"))?;

  let provider = provider_registry
    .find_provider(&provider_name)
    .ok_or(HttpResponse::NotFound().body("Unknown Provider"))?;

  let start_details = provider.start();

  let response = HttpResponse::Found()
    .header("Location", start_details.url)
    .if_some(start_details.nonce, |nonce, response| {
      response.cookie(
        Cookie::build("authentication_nonce", nonce)
          .http_only(true)
          .finish(),
      );
    })
    .finish();

  Ok(response)
}
