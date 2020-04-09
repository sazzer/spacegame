pub mod infrastructure;
pub mod players;

/// The settings with which to run the service
pub struct Settings {
  pub port: u16,
}

/// Actually run the service
pub async fn main(settings: Settings) {
  log::info!("Starting...");

  let service = crate::infrastructure::service::Service::new().await;
  service.start(settings.port).await;
}
