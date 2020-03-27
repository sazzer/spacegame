use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}

/// The actual web server to use
pub struct Server {}

impl Server {
  /// Construct a new web server
  pub fn new() -> Self {
    Server {}
  }

  /// Start the service running
  pub async fn start(&self, port: u16) {
    log::info!("Starting web server on port {}", port);

    let listen_address = format!("0.0.0.0:{}", port);

    HttpServer::new(|| {
      App::new()
        .route("/", web::get().to(index))
        .route("/again", web::get().to(index2))
    })
    .bind(listen_address)
    .unwrap()
    .run()
    .await
    .unwrap();
  }
}
