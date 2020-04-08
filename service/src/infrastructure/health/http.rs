use super::healthchecker::*;
use actix_web::{get, web, Responder};

#[get("/health")]
pub async fn check_health(_healthchecker: web::Data<Healthchecker>) -> impl Responder {
  "Hello".to_owned()
}
