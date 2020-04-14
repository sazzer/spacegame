use super::GoogleSettings;
use crate::authentication::{AuthenticationError, Provider, StartAuthentication};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use uritemplate::UriTemplate;
use uuid::Uuid;

/// Authentication Provider for authenticating with Google
pub struct GoogleProvider {
  settings: GoogleSettings,
}

impl GoogleProvider {
  /// Construct the Google Provider
  pub fn new(settings: GoogleSettings) -> Self {
    Self { settings: settings }
  }
}

/// The shape of the Access Token details retrieved from Google
#[derive(Debug, Deserialize)]
struct GoogleToken {
  id_token: Option<String>,
}

#[async_trait]
impl Provider for GoogleProvider {
  /// Start authentication with the provider
  fn start(&self) -> StartAuthentication {
    let nonce = Uuid::new_v4().to_string();

    let result_url = UriTemplate::new(self.settings.auth_url.as_ref())
      .set("client_id", self.settings.client_id.clone())
      .set("response_type", "code")
      .set("scope", "openid email profile")
      .set("redirect_uri", self.settings.redirect_url.clone())
      .set("state", nonce.clone())
      .build();

    StartAuthentication {
      redirect_url: result_url,
      nonce: Some(nonce),
    }
  }

  /// Complete the authentication process, returning the Player that has just authenticated
  async fn complete(&self, params: HashMap<String, String>) -> Result<String, AuthenticationError> {
    let client = reqwest::Client::new();
    let params = [
      ("grant_type", "authorization_code"),
      ("client_id", self.settings.client_id.as_ref()),
      ("client_secret", self.settings.client_secret.as_ref()),
      ("redirect_uri", self.settings.redirect_url.as_ref()),
      ("code", params.get("code").unwrap().as_ref()),
    ];
    let response = client
      .post(&self.settings.token_url)
      .form(&params)
      .send()
      .await
      .map_err(|e| {
        log::warn!("Failed to communicate with Google: {}", e);
        AuthenticationError::UnknownError
      })?;
    log::debug!("Response from Google: {:#?}", response);
    let body: GoogleToken = response.json().await.map_err(|e| {
      log::warn!("Failed to deserialize response from Google: {}", e);
      AuthenticationError::UnknownError
    })?;
    log::debug!("Response from Google: {:#?}", body);

    Ok("".to_owned())
  }
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

    let parsed_url = Url::parse(&result.redirect_url).unwrap();

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
