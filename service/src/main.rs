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

/// Actually start the application
#[actix_rt::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let settings = Settings::new();
    log::info!("Settings: {:?}", settings);

    let service_settings = spacegame_lib::ServiceSettings {
        database_url: settings.database_url,

        google_settings: match (
            settings.google_client_id,
            settings.google_client_secret,
            settings.google_redirect_url,
        ) {
            (Some(client_id), Some(client_secret), Some(redirect_url)) => {
                Some(spacegame_lib::authentication::google::GoogleSettings {
                    auth_url: settings.google_auth_url,
                    token_url: settings.google_token_url,
                    client_id: client_id,
                    client_secret: client_secret,
                    redirect_url: redirect_url,
                })
            }
            _ => None,
        },
    };
    spacegame_lib::main(settings.port.unwrap_or(8000), service_settings).await
}
