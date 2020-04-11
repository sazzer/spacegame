pub mod authentication;
pub mod http;
pub mod infrastructure;
pub mod players;
pub mod testing;

/// The settings with which to run the service
pub struct Settings {
  pub port: u16,
  pub database_url: String,
}

/// Actually run the service
pub async fn main(settings: Settings) {
  log::info!("Starting...");

  let service_settings = crate::infrastructure::service::Settings {
    database_url: settings.database_url,
  };

  let service = crate::infrastructure::service::Service::new(service_settings).await;
  service.start(settings.port).await;
}
