use actix_cors::Cors;
use actix_service::Service;
use actix_web::{middleware::Logger, web, App, HttpServer};
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

    log::info!("Starting web server on port {}", port);
    let listen_address = format!("0.0.0.0:{}", port);
    HttpServer::new(builder)
      .bind(listen_address)
      .unwrap()
      .run()
      .await
      .unwrap();
  }

  pub async fn run_test(
    &self,
    req: actix_http::Request,
  ) -> Result<TestResponse, actix_http::error::Error> {
    let mut app = App::new()
      // .wrap(Logger::default())
      .wrap(Cors::new().finish());
    for config in self.configs.iter() {
      app = app.configure(config.deref());
    }

    let mut app = actix_web::test::init_service(app).await;
    let res = app.call(req).await;

    match res {
      Ok(res) => {
        log::info!("Success");
        let status = res.status();
        let headers = res.headers().clone();
        let body = actix_web::test::read_body(res).await;

        Ok(TestResponse {
          status: status,
          headers: headers,
          body: body,
        })
      }
      Err(err) => Err(err),
    }
  }
}

pub struct TestResponse {
  pub status: actix_web::http::StatusCode,
  pub headers: actix_http::http::HeaderMap,
  pub body: bytes::Bytes,
}
