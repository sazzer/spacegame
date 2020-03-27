mod infrastructure;

use infrastructure::service::Service;

pub async fn main(port: u16) {
  log::info!("Starting Service");
  let service = Service::new();

  service.start(port).await;
}
