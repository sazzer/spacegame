use config::{Config, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// The environmental settings that are used for running the app
#[derive(Debug, Deserialize)]
struct Settings {
    pub port: Option<u16>,
    pub database_url: String,
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
    };
    spacegame_lib::main(settings.port.unwrap_or(8000), service_settings).await
}
