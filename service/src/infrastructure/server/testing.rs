use actix_cors::Cors;
use actix_http::{error::Error, Request};
use actix_service::Service;
use actix_web::{
  http::{HeaderMap, StatusCode},
  middleware::Logger,
  App,
};
use bytes::Bytes;
use serde_json::Value;
use std::ops::Deref;

/// The response from a test request to the server
pub struct TestResponse {
  pub status: StatusCode,
  pub headers: HeaderMap,
  pub body: Bytes,
}

impl TestResponse {
  /// Build the headers of the response
  pub fn headers(&self) -> String {
    let headers = self
      .headers
      .iter()
      .map(|(name, value)| format!("{}: {}", name, value.to_str().unwrap()))
      .collect::<Vec<String>>()
      .join("\n");

    format!("HTTP/1.1 {}\n{}", self.status, headers)
  }

  /// Convert the response body to JSON
  pub fn to_json(&self) -> Result<Value, serde_json::error::Error> {
    serde_json::from_slice(&self.body)
  }
}

/// Actually run a test against the given HTTP Server
pub async fn run_test(server: &super::Server, req: Request) -> Result<TestResponse, Error> {
  let mut app = App::new()
    .wrap(Logger::default())
    .wrap(Cors::new().finish());
  for config in server.configs.iter() {
    app = app.configure(config.deref());
  }

  let mut app = actix_web::test::init_service(app).await;
  let res = app.call(req).await;

  match res {
    Ok(res) => {
      log::info!("Success");
      let status = res.status();
      let headers = res.headers().clone();
      let body = actix_web::test::read_body(res).await;

      Ok(TestResponse {
        status: status,
        headers: headers,
        body: body,
      })
    }
    Err(err) => Err(err),
  }
}
