use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub exp: i64,
}

pub fn generate_access_token(
    user_id: Uuid,
    secret: &str,
    ttl: Duration,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expires_at = (OffsetDateTime::now_utc() + ttl).unix_timestamp();

    let claims = AccessTokenClaims {
        sub: user_id,
        exp: expires_at
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}