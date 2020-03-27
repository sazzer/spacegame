mod infrastructure;

use infrastructure::service::Service;

pub fn main(port: u16) {
  log::info!("Starting Service");
  let service = Service::new();

  service.start(port);
}
