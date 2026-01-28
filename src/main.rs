use axum::Router;
use std::net::SocketAddr;

mod http;
mod shared;
mod config;

use shared::app_state::AppState;
use config::env::AppConfig;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = AppConfig::from_env();

  let state = AppState {};

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