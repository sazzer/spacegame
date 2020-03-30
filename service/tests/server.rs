use actix_http::{error::Error, Request};
use spacegame_lib::{
  infrastructure::{server::test::TestResponse, service::Service},
  testdatabase::TestDatabase,
};

pub struct TestServer<'d> {
  #[allow(dead_code)] // This isn't dead, but is here to get dropped at the right time
  database: TestDatabase<'d>,
  service: Service,
}

impl<'d> TestServer<'d> {
  pub async fn new() -> TestServer<'d> {
    let database = TestDatabase::new();
    let service_settings = spacegame_lib::ServiceSettings {
      database_url: database.url.clone(),
    };
    let service = Service::new(service_settings).await;

    TestServer { database, service }
  }

  pub async fn run_test(&self, req: Request) -> Result<TestResponse, Error> {
    self.service.run_test(req).await
  }
}
