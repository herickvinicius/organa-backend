use axum::{routing::{get, post}, Router};

use crate::http::{
    handlers::health::health_check,
    auth::handlers::{signup, login}
};
use crate::shared::app_state::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .with_state(state)
}