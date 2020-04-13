use super::model::*;
use actix_web::{http::StatusCode, Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

impl Responder for SystemHealthModel {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let status_code = match self.status {
      Status::Healthy => StatusCode::OK,
      Status::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };

    ready(Ok(HttpResponse::build(status_code).json(self)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use actix_web::test::TestRequest;
  use std::collections::HashMap;
  #[actix_rt::test]
  async fn test_healthy_responder() {
    let health = SystemHealthModel {
      status: Status::Healthy,
      components: HashMap::new(),
    };

    let req = TestRequest::get().to_http_request();

    let response = health.respond_to(&req).await.unwrap();

    assert_eq!(StatusCode::OK, response.status());
  }
}
