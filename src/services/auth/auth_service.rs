use time::{
    Duration,
    OffsetDateTime,
};
use uuid::Uuid;

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

pub struct AuthService<'a> {
    refresh_tokens: RefreshTokenRepository<'a>,
    jwt_secret: &'a str,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
}

pub struct AuthResult {
    pub access_token: String,
    pub refresh_token: String,
}

impl<'a> AuthService<'a> {
    pub fn new(
        refresh_tokens: RefreshTokenRepository<'a>,
        jwt_secret: &'a str,
        access_token_ttl: Duration,
        refresh_token_ttl: Duration,
    ) -> Self {
        Self {
            refresh_tokens,
            jwt_secret,
            access_token_ttl,
            refresh_token_ttl,
        }
    }

    pub async fn authenticate(
        &self,
        user_id: Uuid,
    ) -> Result<AuthResult, AuthError> {
        // generate access token
        let access_token = generate_access_token(
            user_id,
            self.jwt_secret,
            self.access_token_ttl,
        ).map_err(|_| AuthError::JwtError)?;

        // generate refresh token
        let refresh_token = generate_refresh_token();
        let refresh_token_hash = hash_refresh_token(&refresh_token);

        let expires_at =
            OffsetDateTime::now_utc() + self.refresh_token_ttl;

        // persist
        self.refresh_tokens
            .create(
                user_id,
                &refresh_token_hash,
                expires_at,
            )
            .await
            .map_err(|_| AuthError::DatabaseError)?;
        
        Ok((access_token, refresh_token))
    }

    pub async fn refresh(
        &self,
        refresh_token: &str,
    ) -> Result<AuthResult, AuthError> {
        let refresh_token_hash = hash_refresh_token(refresh_token);

        // search for a valid match on db
        // find_valid already garantee that the token found is
        // not revoked nor expired
        let stored = self
            .refresh_tokens
            .find_valid(&refresh_token_hash)
            .await
            .map_err(|_| AuthError::DatabaseError)?
            .ok_or(AuthError::InvalidRefreshToken)?;
        
        // revoke old refresh_token
        self.refresh_tokens
            .revoke(stored.id)
            .await
            .map_err(|_| AuthError::DatabaseError)?;
        
        // generate a new refresh_token
        let new_refresh_token = generate_refresh_token();
        let new_refresh_token_hash = hash_refresh_token(&new_refresh_token);

        let expires_at = OffsetDateTime::now_utc() + self.refresh_token_ttl;

        // persist new refresh_token
        self.refresh_tokens
            .create(
                stored.user_id,
                &new_rerfesh_token_hash,
                expires_at,
            )
            .await
            .map_err(|_| AuthError::DatabaseError)?;
        
        // generate new access_token
        let access_token = generate_access_token(
            stored.user_id,
            self.jwt_secret,
            self.access_token_ttl
        )
        .map_err(|_| AuthError::JwtError)?;

        Ok((access_token, new_refresh_token))
    }
}