use axum::Router;
use std::net::SocketAddr;

mod http;
mod shared;

use shared::app_state::AppState;

#[tokio::main]
async fn main() {
  let state = AppState {};

  let app = Router::new()
    .merge(http::routes::create_routes(state));

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  println!("🚀 Organa backend running on {}", addr);

  axum::serve(
    tokio::net::TcpListener::bind(addr).await.unwrap(),
    app,
  )
  .await
  .unwrap();
}