use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  let app = Router::new();

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  println!("🚀 Organa backend running on {}", addr);

  axum::serve(
    tokio::net::TcpListener::bind(addr).await.unwrap(),
    app,
  )
  .await
  .unwrap();
}