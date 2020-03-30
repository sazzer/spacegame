use crate::asserts::*;
use crate::server::TestServer;
use actix_web::test::TestRequest;
use galvanic_assert::assert_that;
use serde_json::json;

#[actix_rt::test]
async fn test_healthcheck() {
  env_logger::try_init().ok();

  let service = TestServer::new().await;

  let res = service
    .run_test(TestRequest::get().uri("/health").to_request())
    .await
    .unwrap();

  assert_that!(&res, has_status_code(actix_web::http::StatusCode::OK));
  assert_that!(&res, has_header("content-type", "application/json"));
  assert_that!(
    &res,
    has_json_body(json!({
      "status": "HEALTHY",
      "components": {
        "database": {
          "status": "HEALTHY"
        }
      }
    }))
  );
}
