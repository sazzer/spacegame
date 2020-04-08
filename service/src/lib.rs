pub mod infrastructure;
pub mod players;

pub fn main() {
  log::info!("Starting...");

  let service = crate::infrastructure::service::Service::new();
  service.start();
}
