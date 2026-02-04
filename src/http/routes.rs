use axum::{routing::{get, post}, Router};

use crate::http::{
    handlers::health::health_check,
    auth::handlers::{signup, login, refresh}
};
use crate::shared::app_state::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/auth/signup", post(signup))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .with_state(state)
}