use actix_web::{web, App, HttpServer};
use std::ops::Deref;
use std::sync::Arc;

/// The actual web server to use
pub struct Server {
  configs: Vec<Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync>>,
}

impl Server {
  /// Construct a new web server
  pub fn new(configs: Vec<Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync>>) -> Self {
    Server { configs }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    log::info!("Starting web server on port {}", port);

    let listen_address = format!("0.0.0.0:{}", port);

    let configs = self.configs.clone();

    HttpServer::new(move || {
      let configs = configs.clone();
      let mut app = App::new();
      for config in configs.iter() {
        app = app.configure(config.deref());
      }
      app
    })
    .bind(listen_address)
    .unwrap()
    .run()
    .await
    .unwrap();
  }
}
