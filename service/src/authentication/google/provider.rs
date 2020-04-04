use super::GoogleSettings;
use crate::authentication::*;
use std::collections::HashMap;
use uritemplate::UriTemplate;
use uuid::Uuid;

/// The default URL to use to start authentication
const DEFAULT_AUTH_URL: &str =
  "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}";

/// Authentication Provider for authenticating with Google
pub struct GoogleProvider {
  auth_url: String,
  client_id: String,
  redirect_url: String,
}

impl GoogleProvider {
  /// Create a new instance of the Google Provider
  pub fn new(settings: GoogleSettings) -> Self {
    Self {
      auth_url: settings.auth_url.unwrap_or(DEFAULT_AUTH_URL.to_owned()),

      client_id: settings.client_id,
      redirect_url: settings.redirect_url,
    }
  }
}

impl Provider for GoogleProvider {
  /// Start the authentication process, generating details to redirect the user to in order for them to log in
  fn start(&self) -> StartAuthentication {
    let state = Uuid::new_v4().to_string();

    let result_url = UriTemplate::new(&self.auth_url)
      .set("client_id", self.client_id.clone())
      .set("response_type", "code")
      .set("scope", "openid email profile")
      .set("redirect_uri", self.redirect_url.clone())
      .set("state", state.clone())
      .build();

    StartAuthentication::new(result_url).with_nonce(state)
  }

  /// Complete the authentication process, returning the Player that has just authenticated
  fn complete(&self, params: HashMap<String, String>) {}
}

#[cfg(test)]
mod tests {
  use super::*;
  use galvanic_assert::{
    assert_that,
    matchers::{collection::*, variant::*, *},
  };
  use url::Url;

  #[test]
  fn test_start_authentication() {
    let sut = GoogleProvider::new(GoogleSettings {
      client_id: "googleClientId".to_owned(),
      redirect_url: "http://localhost:8000/authentication/google/callback".to_owned(),
      ..Default::default()
    });

    let result = sut.start();

    assert_that!(&result.nonce, maybe_some(any_value()));

    let parsed_url = Url::parse(&result.url).unwrap();

    assert_that!(&parsed_url.scheme(), eq("https"));
    assert_that!(&parsed_url.username(), eq(""));
    assert_that!(&parsed_url.password(), eq(None));
    assert_that!(
      &parsed_url.host_str(),
      maybe_some(eq("accounts.google.com"))
    );
    assert_that!(&parsed_url.port(), eq(None));
    assert_that!(&parsed_url.path(), eq("/o/oauth2/v2/auth"));
    assert_that!(&parsed_url.fragment(), eq(None));

    let query: Vec<(String, String)> = parsed_url.query_pairs().into_owned().collect();
    assert_that!(
      &query,
      contains_in_any_order(vec![
        ("client_id".to_owned(), "googleClientId".to_owned()),
        ("response_type".to_owned(), "code".to_owned()),
        ("scope".to_owned(), "openid email profile".to_owned()),
        (
          "redirect_uri".to_owned(),
          "http://localhost:8000/authentication/google/callback".to_owned()
        ),
        ("state".to_owned(), result.nonce.unwrap()),
      ])
    );
  }
}
