#![feature(async_closure)]

pub mod infrastructure;
pub mod testdatabase;

use infrastructure::service::Service;

pub use infrastructure::service::ServiceSettings;

pub async fn main(port: u16, settings: ServiceSettings) {
  log::info!("Starting Service");
  let service = Service::new(settings).await;

  service.start(port).await;
}
