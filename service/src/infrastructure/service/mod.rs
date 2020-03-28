use super::{database::Database, server::Server};
use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Arc;

async fn index(data: web::Data<Database>) -> impl Responder {
  data.ping().await;

  HttpResponse::Ok().body("Hello")
}

fn config(cfg: &mut web::ServiceConfig) {
  cfg.route("/", web::get().to(index));
}

/// The actual service to use
pub struct Service {
  server: Server,
}

impl Service {
  /// Construct a new service
  pub async fn new() -> Self {
    log::info!("Building Service");
    let database: Database =
      Database::new("postgres://spacegame:spacegame@localhost:45432/spacegame").await;

    let mut healthchecks: HashMap<String, Arc<dyn crate::infrastructure::health::Component>> =
      HashMap::new();
    healthchecks.insert("database".to_owned(), Arc::new(database.clone()));

    let server = Server::new(vec![
      Arc::new(config),
      crate::infrastructure::health::configure::configure_healthchecks(healthchecks),
      Arc::new(move |cfg| {
        cfg.data(database.clone());
      }),
    ]);

    Service { server }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    self.server.start(port).await;
  }
}
