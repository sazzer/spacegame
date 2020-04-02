use crate::authentication::google::GoogleSettings;

/// The settings to use to configure the service
#[derive(Debug, Default)]
pub struct ServiceSettings {
  pub database_url: String,

  pub google_settings: Option<GoogleSettings>,
}
