use super::server::Server;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Arc;

trait FakeService {
  fn name(&self) -> &String;
}
struct FakeServiceImpl {
  name: String,
}

impl FakeService for FakeServiceImpl {
  fn name(&self) -> &String {
    &self.name
  }
}

async fn index(data: web::Data<Arc<dyn FakeService>>) -> impl Responder {
  let output = format!("Hello {}: {:p}\n\n", &data.name(), &data.name());
  HttpResponse::Ok().body(output)
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
  pub fn new() -> Self {
    log::info!("Building Service");
    let service = Arc::new(FakeServiceImpl {
      name: "Name".to_owned(),
    });

    let server = Server::new(vec![
      Arc::new(config),
      Arc::new(move |cfg| {
        cfg.data(service.clone() as Arc<dyn FakeService>);
      }),
    ]);

    Service { server }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    self.server.start(port).await;
  }
}
