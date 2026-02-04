use axum::Router;
use std::net::SocketAddr;

mod domain;
mod repositories;
mod http;
mod shared;
mod config;
mod db;

use shared::app_state::AppState;
use config::env::AppConfig;
use db::create_pool;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = AppConfig::from_env();

  let db_pool = create_pool(&config.database_url).await;

  // TODO: access and refresh tokens here

  let state = AppState { 
    db_pool,
    jwt_secret: config.jwt_secret,
    access_token_ttl: config.access_token_ttl,
    refresh_token_ttl: config.refresh_token_ttl,
  };

  let app = Router::new()
    .merge(http::routes::create_routes(state));

  let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
  println!("🚀 Organa backend running on {}", addr);

  axum::serve(
    tokio::net::TcpListener::bind(addr).await.unwrap(),
    app,
  )
  .await
  .unwrap();
}