use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new();

    println!("🚀 Organa backend running");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}