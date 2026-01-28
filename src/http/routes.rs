use axum::{routing::{get, post}, Router};

use crate::http::{
    handlers::health::health_check,
    auth::handlers::signup,
};
use crate::shared::app_state::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/signup", post(signup))
        .with_state(state)
}