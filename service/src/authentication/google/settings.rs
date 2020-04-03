/// Settings needed to configure the Google Provider
#[derive(Debug, Clone)]
pub struct GoogleSettings {
  pub auth_url: Option<String>,

  pub client_id: String,
  pub redirect_url: String,
}

impl Default for GoogleSettings {
  fn default() -> Self {
    Self {
      auth_url: None,

      client_id: "".to_owned(),
      redirect_url: "".to_owned(),
    }
  }
}
