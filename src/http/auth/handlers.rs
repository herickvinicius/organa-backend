use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    shared::app_state::AppState,
    repositories::user_repository::UserRepository,
    http::auth::dto::SignupRequest,
    shared::crypto::hash_password,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<StatusCode, StatusCode> {
    let repo = UserRepository::new(&state.db_pool);

    let password_hash = hash_password(&payload.password);

    match repo.create(&payload.email, &password_hash).await {
    Ok(_) => Ok(StatusCode::CREATED),
    Err(err) => {
        eprintln!("❌ signup error: {:?}", err);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
}
