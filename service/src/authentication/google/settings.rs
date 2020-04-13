/// The default URL to use to start authentication with Google
const DEFAULT_GOOGLE_AUTH_URL: &str =
  "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}";

/// Settings to use for Google Authentication
#[derive(Debug)]
pub struct GoogleSettings {
  pub auth_url: String,
  pub client_id: String,
  pub redirect_url: String,
}

impl Default for GoogleSettings {
  fn default() -> Self {
    Self {
      auth_url: DEFAULT_GOOGLE_AUTH_URL.to_owned(),
      client_id: "".to_owned(),
      redirect_url: "".to_owned(),
    }
  }
}

impl GoogleSettings {
  /// Specify the authentication URL
  pub fn with_auth_url(self, auth_url: Option<String>) -> Self {
    Self {
      auth_url: auth_url.unwrap_or(self.auth_url),
      ..self
    }
  }

  /// Specify the Client ID
  pub fn with_client_id(self, client_id: Option<String>) -> Self {
    Self {
      client_id: client_id.unwrap_or(self.client_id),
      ..self
    }
  }
  /// Specify the Redirect URL
  pub fn with_redirect_url(self, redirect_url: Option<String>) -> Self {
    Self {
      redirect_url: redirect_url.unwrap_or(self.redirect_url),
      ..self
    }
  }

  /// Determine whether the settings are configured
  pub fn is_configured(&self) -> bool {
    !self.client_id.is_empty() && !self.redirect_url.is_empty()
  }
}