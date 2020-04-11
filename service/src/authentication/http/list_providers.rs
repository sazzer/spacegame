use crate::authentication::{ProviderName, ProviderRegistry};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

/// API Model for the list of Authentication Provider names
#[derive(Serialize)]
struct ProviderListModel<'a> {
  providers: Vec<&'a ProviderName>,
}

/// List the authentication providers that we can work with
#[get("/authentication")]
pub async fn list_providers(provider_registry: web::Data<ProviderRegistry>) -> impl Responder {
  let names = provider_registry.provider_names().collect();

  HttpResponse::Ok().json(ProviderListModel { providers: names })
}
