#![feature(async_closure)]
#![feature(trait_alias)]

pub mod entity;
pub mod infrastructure;
pub mod players;
pub mod testdatabase;

use infrastructure::service::Service;

pub use infrastructure::service::ServiceSettings;

pub async fn main(port: u16, settings: ServiceSettings) {
  log::info!("Starting Service");
  let service = Service::new(settings).await;

  service.start(port).await;
}
