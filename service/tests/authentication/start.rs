use crate::service::TestService;
use actix_web::test::TestRequest;
use insta::{assert_display_snapshot, assert_json_snapshot};

#[actix_rt::test]
async fn integration_test_start_authentication_invalid() {
  let test_service = TestService::new().await;

  let req = TestRequest::get().uri("/authentication/%20").to_request();

  let res = test_service.request(req).await.unwrap();

  assert_display_snapshot!(res.headers());
  assert_json_snapshot!(res.to_json().unwrap());
}

#[actix_rt::test]
async fn integration_test_start_authentication_unknown() {
  let test_service = TestService::new().await;

  let req = TestRequest::get()
    .uri("/authentication/unknown")
    .to_request();

  let res = test_service.request(req).await.unwrap();

  assert_display_snapshot!(res.headers());
  assert_json_snapshot!(res.to_json().unwrap());
}
