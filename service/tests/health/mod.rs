use actix_web::test::TestRequest;
use spacegame_lib::{infrastructure::service::Service, testdatabase::TestDatabase};

#[actix_rt::test]
async fn test_healthcheck() {
  env_logger::try_init().ok();

  let database = TestDatabase::new();
  let service_settings = spacegame_lib::ServiceSettings {
    database_url: database.url,
  };

  let service = Service::new(service_settings).await;

  let res = service
    .run_test(TestRequest::get().uri("/health").to_request())
    .await
    .unwrap();

  assert_eq!(actix_web::http::StatusCode::OK, res.status);
  assert_eq!(1, 2);
}
