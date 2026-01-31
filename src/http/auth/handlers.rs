use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};

use crate::{
    shared::app_state::AppState,
    repositories::user_repository::UserRepository,
    http::auth::dto::{SignupRequest, SignupResponse},
    shared::crypto::hash_password,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = UserRepository::new(&state.db_pool);

    let password_hash = hash_password(&payload.password);

    let user = repo
        .create(&payload.email, &password_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = SignupResponse {
        id: user.id,
        email: user.email,
    };

    Ok((StatusCode::CREATED, Json(response)))
}