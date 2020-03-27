#![feature(async_closure)]

mod infrastructure;

use infrastructure::service::Service;

pub async fn main(port: u16) {
  log::info!("Starting Service");
  let service = Service::new().await;

  service.start(port).await;
}
