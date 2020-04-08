use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::ops::Deref;
use std::sync::Arc;

/// The type used to represent a function to provide configuration to Actix
pub type FnConfig = Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync>;

/// The actual HTTP Server
pub struct Server {
  configs: Vec<FnConfig>,
}

impl Server {
  /// Create a new Server
  pub fn new(configs: Vec<FnConfig>) -> Self {
    log::info!("Creating HTTP Server");

    Self { configs }
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    let configs = self.configs.clone();
    let builder = move || {
      let configs = configs.clone();
      let mut app = App::new()
        .wrap(Logger::default())
        .wrap(Cors::new().finish());
      for config in configs.iter() {
        app = app.configure(config.deref());
      }
      app
    };

    let listen_address = format!("0.0.0.0:{}", port);
    log::info!("Starting web server on {}", listen_address);
    HttpServer::new(builder)
      .bind(listen_address)
      .unwrap()
      .run()
      .await
      .unwrap();
  }
}
