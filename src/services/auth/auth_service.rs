use time::{
    Duration,
    OffsetDateTime,
};
use uuid::Uuid;
use sqlx::PgPool;

use crate::{
    repositories::refresh_token_repository::RefreshTokenRepository,
    shared::{
        jwt::generate_access_token,
        refresh_token::{
            generate_refresh_token,
            hash_refresh_token,
        },
    },
};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    InvalidRefreshToken,
    TokenExpired,
    TokenRevoked,
    DatabaseError,
    JwtError,
    InternalError,
}

#[derive(Clone)]
pub struct AuthService {
    db_pool:PgPool,
    jwt_secret: String,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
}

pub struct AuthResult {
    pub access_token: String,
    pub refresh_token: String,
}

impl AuthService {
    pub fn new(
        db_pool: PgPool,
        jwt_secret: String,
        access_token_ttl: Duration,
        refresh_token_ttl: Duration,
    ) -> Self {
        Self {
            db_pool,
            jwt_secret,
            access_token_ttl,
            refresh_token_ttl,
        }
    }

    pub async fn authenticate(
        &self,
        user_id: Uuid,
    ) -> Result<AuthResult, AuthError> {
        
        let refresh_token_repo = RefreshTokenRepository::new(&self.db_pool);

        // generate access token
        let access_token = generate_access_token(
            user_id,
            &self.jwt_secret,
            self.access_token_ttl,
        ).map_err(|_| AuthError::JwtError)?;

        // generate refresh token
        let refresh_token = generate_refresh_token();
        let refresh_token_hash = hash_refresh_token(&refresh_token);

        let expires_at =
            OffsetDateTime::now_utc() + self.refresh_token_ttl;

        // persist refresh token
        refresh_token_repo
            .create(
                user_id,
                &refresh_token_hash,
                expires_at,
            )
            .await
            .map_err(|_| AuthError::DatabaseError)?;
        
        Ok(AuthResult {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh(
        &self,
        refresh_token: &str,
    ) -> Result<AuthResult, AuthError> {
        let refresh_token_repo = RefreshTokenRepository::new(&self.db_pool);
        let refresh_token_hash = hash_refresh_token(refresh_token);

        // search for a valid match on db
        // find_valid already garantee that the token found is
        // not revoked nor expired
        let stored = refresh_token_repo
            .find_valid(&refresh_token_hash)
            .await
            .map_err(|_| AuthError::DatabaseError)?
            .ok_or(AuthError::InvalidRefreshToken)?;
        
        // revoke old refresh_token
        refresh_token_repo
            .revoke(stored.id)
            .await
            .map_err(|_| AuthError::DatabaseError)?;
        
        // generate a new refresh_token
        let new_refresh_token = generate_refresh_token();
        let new_refresh_token_hash = hash_refresh_token(&new_refresh_token);

        let expires_at = OffsetDateTime::now_utc() + self.refresh_token_ttl;

        // persist new refresh_token
        refresh_token_repo
            .create(
                stored.user_id,
                &new_refresh_token_hash,
                expires_at,
            )
            .await
            .map_err(|_| AuthError::DatabaseError)?;
        
        // generate new access_token
        let access_token = generate_access_token(
            stored.user_id,
            &self.jwt_secret,
            self.access_token_ttl
        )
        .map_err(|_| AuthError::JwtError)?;

        Ok(AuthResult {
            access_token,
            refresh_token: new_refresh_token,
        })
    }
}