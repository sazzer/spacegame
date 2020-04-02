use crate::authentication::*;
use actix_web::{get, http::Cookie, web, HttpResponse, Responder};
use std::str::FromStr;
use std::sync::Arc;

/// HTTP Handler to start authentication against a specific provider
#[get("/authentication/{provider}")]
pub async fn start_authentication(
  path: web::Path<(String,)>,
  provider_registry: web::Data<ProviderRegistry>,
) -> impl Responder {
  let provider: Option<Arc<dyn Provider>> = ProviderName::from_str(path.0.as_ref())
    .ok()
    .and_then(|provider_name| provider_registry.find_provider(&provider_name));

  match provider {
    Some(provider) => {
      let start_details = provider.start();
      let mut response = HttpResponse::Found();
      response.header("Location", start_details.url);
      if let Some(nonce) = start_details.nonce {
        response.cookie(
          Cookie::build("authentication_nonce", nonce)
            .http_only(true)
            .finish(),
        );
      }
      response.finish()
    }
    None => HttpResponse::NotFound().finish(),
  }
}
