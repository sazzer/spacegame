use super::service::*;
use actix_web::web;
use std::sync::Arc;

pub fn configure_players(
  player_service: PlayerService,
) -> Arc<dyn Fn(&mut web::ServiceConfig) + Send + Sync> {
  let player_service = player_service.clone();

  Arc::new(move |cfg| {
    cfg.data(player_service.clone());
  })
}
