use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
};

use crate::{
    shared::app_state::AppState,
    repositories::user_repository::UserRepository,
    http::auth::dto::{
        SignupRequest,
        SignupResponse,
        LoginRequest,
        LoginResponse,
    },
    shared::crypto::{hash_password, verify_password},
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

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let repo = UserRepository::new(&state.db_pool);

    let user = match repo.find_by_email(&payload.email).await {
        Ok(Some(user)) => user,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let password_ok = verify_password(&payload.password, &user.password_hash);

    if !password_ok {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let response = LoginResponse {
        id: user.id,
        email: user.email,
    };

    Ok((StatusCode::OK, Json(response)))
}