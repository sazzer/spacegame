use super::GoogleSettings;
use crate::authentication::*;
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;
use uritemplate::UriTemplate;
use uuid::Uuid;

/// The default URL to use to start authentication
const DEFAULT_AUTH_URL: &str =
  "https://accounts.google.com/o/oauth2/v2/auth{?client_id,response_type,scope,redirect_uri,state}";

/// The default URL to use to get the Google token details
const DEFAULT_TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v4/token";

/// Authentication Provider for authenticating with Google
pub struct GoogleProvider {
  settings: GoogleSettings,
}

impl GoogleProvider {
  /// Create a new instance of the Google Provider
  pub fn new(settings: GoogleSettings) -> Self {
    Self { settings: settings }
  }
}

#[async_trait]
impl Provider for GoogleProvider {
  /// Start the authentication process, generating details to redirect the user to in order for them to log in
  fn start(&self) -> StartAuthentication {
    let state = Uuid::new_v4().to_string();

    let result_url = UriTemplate::new(
      self
        .settings
        .auth_url
        .as_ref()
        .unwrap_or(&DEFAULT_AUTH_URL.to_owned()),
    )
    .set("client_id", self.settings.client_id.clone())
    .set("response_type", "code")
    .set("scope", "openid email profile")
    .set("redirect_uri", self.settings.redirect_url.clone())
    .set("state", state.clone())
    .build();

    StartAuthentication::new(result_url).with_nonce(state)
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
      .post(DEFAULT_TOKEN_URL)
      .form(&params)
      .send()
      .await
      .map_err(|e| {
        log::warn!("Failed to communicate with Google: {}", e);
        AuthenticationError::UnknownError
      })?;
    log::debug!("Response from Google: {:#?}", response);

    if response.status() != reqwest::StatusCode::OK {
      log::warn!(
        "An error occurred authenticating with Google: {:?}",
        response.text().await
      );
      return Err(AuthenticationError::UnknownError);
    }

    let token_response: TokenResponse = response.json().await.map_err(|e| {
      log::warn!("Failed to deserialize token from Google: {}", e);
      AuthenticationError::UnknownError
    })?;

    log::info!("Token from Google: {:?}", token_response);
    Ok("".to_owned())
  }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
  access_token: String,
  expires_in: u32,
  scope: String,
  token_type: String,
  id_token: String,
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
