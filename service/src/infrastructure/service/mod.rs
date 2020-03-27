use super::server::Server;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}

fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(index));
}

fn config2(cfg: &mut web::ServiceConfig) {
  cfg.route("/again", web::get().to(index2));
}

/// The actual service to use
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new service
  pub fn new() -> Self {
    log::info!("Building Service");
    let server = Server::new(vec![Arc::new(config), Arc::new(config2)]);

    Service { server }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    self.server.start(port).await;
  }
}
