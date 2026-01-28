use axum::{routing::get, Router};

use crate::http::handlers::health::health_check;
use crate::shared::app_state::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}