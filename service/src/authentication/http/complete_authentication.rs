use crate::authentication::*;
use actix_web::{get, web, HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::str::FromStr;

/// HTTP Handler to start authentication against a specific provider
#[get("/authentication/{provider}/complete")]
pub async fn complete_authentication(
  req: HttpRequest,
  path: web::Path<(String,)>,
  provider_registry: web::Data<ProviderRegistry>,
) -> Result<HttpResponse, HttpResponse> {
  let provider_name = ProviderName::from_str(path.0.as_ref())
    .map_err(|_e| HttpResponse::BadRequest().body("Invalid Provider Name"))?;

  let _provider = provider_registry
    .find_provider(&provider_name)
    .ok_or(HttpResponse::NotFound().body("Unknown Provider"))?;

  let params: HashMap<String, String> = req
    .uri()
    .query()
    .map(|v| {
      url::form_urlencoded::parse(v.as_bytes())
        .into_owned()
        .collect()
    })
    .unwrap_or_else(HashMap::new);
  log::info!("Query String: {:?}", params);

  Ok(HttpResponse::Ok().finish())
}
