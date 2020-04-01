use super::super::ProviderName;
use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;

/// Representation of the Provider details sent to the client
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ProviderModel {
  provider_name: ProviderName,
}

/// HTTP Handler to list all of the Authentication Providers that are available to use
#[get("/authentication")]
pub async fn list_providers() -> impl Responder {
  let providers = vec![
    ProviderModel {
      provider_name: "google".to_owned(),
    },
    ProviderModel {
      provider_name: "twitter".to_owned(),
    },
  ];
  HttpResponse::Ok().json(providers)
}
