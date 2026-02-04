use axum::{
    extract::State,
    http::{
        StatusCode,
        header,
        HeaderMap,
    },
    response::IntoResponse,
    Json,
};
use time::OffsetDateTime;

use crate::{
    shared::{
        app_state::AppState,
        crypto::{
            verify_password,
            hash_password,
        },
        jwt::generate_access_token,
        refresh_token::{
            generate_refresh_token,
            hash_refresh_token,
        },
    },
    repositories::{
        user_repository::UserRepository,
        refresh_token_repository::RefreshTokenRepository,
    },
    http::auth::dto::{
        SignupRequest,
        LoginRequest,
        AuthResponse,
    },
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let user_repo = UserRepository::new(&state.db_pool);
    let refresh_token_repo = RefreshTokenRepository::new(&state.db_pool);

    // hash the password
    let password_hash = hash_password(&payload.password);

    // create user
    let user = user_repo
        .create(&payload.email, &password_hash)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // generate access token
    let access_token = generate_access_token(
        user.id,
        &state.jwt_secret,
        state.access_token_ttl,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // generate refresh token
    let refresh_token = generate_refresh_token();
    let refresh_token_hash = hash_refresh_token(&refresh_token);
    let expires_at = OffsetDateTime::now_utc() + state.refresh_token_ttl;
    
    // persist refresh token
    refresh_token_repo
        .create(
            user.id,
            &refresh_token_hash,
            expires_at,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // set cookie
    let cookie = format!(
        "refresh_token={}; HttpOnly; Path=/auth/refresh; Max-Age={}",
        refresh_token,
        state.refresh_token_ttl.whole_seconds()
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        cookie.parse().unwrap(),
    );

    // response
    Ok((
        StatusCode::CREATED,
        headers,
        Json(AuthResponse {
            access_token,
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // set repos
    let user_repo = UserRepository::new(&state.db_pool);
    let refresh_token_repo = RefreshTokenRepository::new(&state.db_pool);

    // verify user and password
    let user = match user_repo.find_by_email(&payload.email).await {
        Ok(Some(user)) => user,
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify_password(&payload.password, &user.password_hash) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // create access token
    let access_token = generate_access_token(
        user.id,
        &state.jwt_secret,
        state.access_token_ttl,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // create refresh token
    let refresh_token = generate_refresh_token();
    let refresh_token_hash = hash_refresh_token(&refresh_token);
    let expires_at =
        OffsetDateTime::now_utc() + state.refresh_token_ttl;

    // persist refresh token
    refresh_token_repo
        .create(user.id, &refresh_token_hash, expires_at)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // set cookie
    let cookie = format!(
        "refresh_token={}; HttpOnly; Path=/auth/refresh; Max-Age={}",
        refresh_token,
        state.refresh_token_ttl.whole_seconds()
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        cookie.parse().unwrap(),
    );

    // response
    Ok((
        StatusCode::OK,
        headers,
        Json(AuthResponse {
            access_token,
        }),
    ))
}

pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, StatusCode> {
    // read cookie
    let cookie_header = headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let refresh_token = cookie_header
        .split(',')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            cookie
                .strip_prefix("refresh_token=")
                .map(|v| v.to_string())
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let refresh_token_hash = hash_refresh_token(&refresh_token);

    // find hash on db
    let repo = RefreshTokenRepository::new(&state.db_pool);
    let stored = repo
        .find_valid(&refresh_token_hash)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // check expiration
    if stored.expires_at < OffsetDateTime::now_utc() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // rotate refresh token
    repo.revoke(stored.id).await.ok();

    let new_refresh = generate_refresh_token();
    let new_refresh_hash = hash_refresh_token(&new_refresh);

    repo.create(
        stored.user_id,
        &new_refresh_hash,
        OffsetDateTime::now_utc() + state.refresh_token_ttl,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // generate new access token
    let access_token = generate_access_token(
        stored.user_id,
        &state.jwt_secret,
        state.access_token_ttl,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // set cookie
    let cookie = format!(
        "refresh_token={}; HttpOnly; Path=/auth/refresh; Max-Age={}",
        refresh_token,
        state.refresh_token_ttl.whole_seconds()
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        cookie.parse().unwrap(),
    );

    // response
    Ok((
        StatusCode::OK,
        headers,
        Json(AuthResponse {
            access_token,
        }),
    ))
}