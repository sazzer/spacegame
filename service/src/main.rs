use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// The environmental settings that are used for running the app
#[derive(Debug, Deserialize)]
struct Settings {
    pub port: Option<u16>,
    pub database_url: String,

    pub google_auth_url: Option<String>,
    pub google_token_url: Option<String>,
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub google_redirect_url: Option<String>,
}

impl Settings {
    /// Construct the settings from the environment
    fn new() -> Settings {
        let mut s = Config::new();
        s.merge(Environment::default()).unwrap();

        s.try_into().unwrap()
    }
}

#[actix_rt::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let settings = Settings::new();

    spacegame_lib::main(spacegame_lib::Settings {
        port: settings.port.unwrap_or(8000),
        database_url: settings.database_url,

        google_auth_url: settings.google_auth_url,
        google_token_url: settings.google_token_url,
        google_client_id: settings.google_client_id,
        google_client_secret: settings.google_client_secret,
        google_redirect_url: settings.google_redirect_url,
    })
    .await;
}
