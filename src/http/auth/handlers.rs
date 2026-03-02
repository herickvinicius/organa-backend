use axum::{
    extract::State,
    http::{
        StatusCode,
        HeaderMap,
    },
    response::IntoResponse,
    Json,
};

use crate::{
    shared::{
        app_state::AppState,
        crypto::{
            verify_password,
            hash_password,
        },
    },
    repositories::{
        user_repository::UserRepository,
        refresh_token_repository::RefreshTokenRepository,
    },
    services::auth::AuthService,
    http::{
        auth::dto::{
            SignupRequest,
            LoginRequest,
            AuthResponse,
        },
        cookies::{
            set_refresh_cookie,
            read_refresh_cookie,
        },
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

    // Authenticate
    let auth_service = AuthService::new(
        refresh_token_repo,
        &state.jwt_secret,
        state.access_token_ttl,
        state.refresh_token_ttl,
    );

    let result = auth_service
        .authenticate(user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let access_token = result.access_token;
    let refresh_token = result.refresh_token;

    let mut headers = HeaderMap::new();
    set_refresh_cookie(
        &mut headers,
        &refresh_token,
        state.refresh_token_ttl,
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

    let auth_service = AuthService::new(
        refresh_token_repo,
        &state.jwt_secret,
        state.access_token_ttl,
        state.refresh_token_ttl,
    );

    let result = auth_service
        .authenticate(user.id)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let access_token = result.access_token;
    let refresh_token = result.refresh_token;

    let mut headers = HeaderMap::new();
    set_refresh_cookie(
        &mut headers,
        &refresh_token,
        state.refresh_token_ttl,
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
    let refresh_token = read_refresh_cookie(&headers)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Refresh
    let refresh_token_repo = RefreshTokenRepository::new(&state.db_pool);

    let auth_service = AuthService::new(
        refresh_token_repo,
        &state.jwt_secret,
        state.access_token_ttl,
        state.refresh_token_ttl,
    );

    let result = auth_service
        .refresh(&refresh_token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let access_token = result.access_token;
    let new_refresh_token = result.refresh_token;
    
    // set cookie
    let mut headers = HeaderMap::new();
    set_refresh_cookie(
        &mut headers,
        &new_refresh_token,
        state.refresh_token_ttl,
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