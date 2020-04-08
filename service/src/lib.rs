pub mod infrastructure;
pub mod players;

pub async fn main() {
  log::info!("Starting...");

  let service = crate::infrastructure::service::Service::new().await;
  service.start(8000).await;
}
