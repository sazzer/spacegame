use actix_http::{error::Error, Request};
use spacegame_lib::{
  infrastructure::{
    server::testing::TestResponse,
    service::{Service, Settings},
  },
  testing::database::TestDatabase,
};

/// Wrapper around the service to test
pub struct TestService<'a> {
  // This needs to be owned here but doesn't currently get used
  _db: TestDatabase<'a>,
  service: Service,
}

impl<'a> TestService<'a> {
  /// Construct the new wrapper to test
  pub async fn new() -> TestService<'a> {
    let _ = env_logger::try_init();

    let db = TestDatabase::new();
    let service_settings = Settings {
      database_url: db.url.clone(),
      google_settings: spacegame_lib::authentication::google::GoogleSettings::default(),
    };
    let service = Service::new(service_settings).await;

    TestService {
      _db: db,
      service: service,
    }
  }

  /// Send an HTTP Request to the service
  pub async fn request(&self, req: Request) -> Result<TestResponse, Error> {
    self.service.test_request(req).await
  }
}
