use crate::authentication::*;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

/// Representation of the Provider details sent to the client
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderModel {
  provider_name: ProviderName,
}

/// HTTP Handler to list all of the Authentication Providers that are available to use
#[get("/authentication")]
pub async fn list_providers(provider_registry: web::Data<ProviderRegistry>) -> impl Responder {
  let providers: Vec<ProviderModel> = provider_registry
    .list_providers()
    .into_iter()
    .map(|p| ProviderModel { provider_name: p })
    .collect();

  HttpResponse::Ok().json(providers)
}
