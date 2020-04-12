pub mod authentication;
pub mod http;
pub mod infrastructure;
pub mod players;
pub mod testing;

/// The settings with which to run the service
pub struct Settings {
  pub port: u16,
  pub database_url: String,

  pub google_auth_url: Option<String>,
  pub google_client_id: Option<String>,
  pub google_redirect_url: Option<String>,
}

/// Actually run the service
pub async fn main(settings: Settings) {
  log::info!("Starting...");

  let service_settings = crate::infrastructure::service::Settings {
    database_url: settings.database_url,

    google_settings: crate::authentication::google::GoogleSettings::default()
      .with_auth_url(settings.google_auth_url)
      .with_client_id(settings.google_client_id)
      .with_redirect_url(settings.google_redirect_url),
  };

  let service = crate::infrastructure::service::Service::new(service_settings).await;
  service.start(settings.port).await;
}
