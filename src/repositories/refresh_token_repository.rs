use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::refresh_token::RefreshToken;

pub struct RefreshTokenRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> RefreshTokenRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        token_hash: &str,
        expires_at: time::OffsetDateTime,
    ) -> Result<RefreshToken, sqlx::Error> {
        sqlx::query_as!(
            RefreshToken,
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                user_id,
                token_hash,
                expires_at,
                created_at,
                revoked_at
            "#,
            user_id,
            token_hash,
            expires_at
        )
        .fetch_one(self.pool)
        .await
    }

    pub async fn find_valid(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, sqlx::Error> {
        sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT
                id,
                user_id,
                token_hash,
                expires_at,
                created_at,
                revoked_at
            FROM refresh_tokens
            WHERE
                token_hash = $1 AND
                revoked_at IS NULL AND
                expires_at > NOW()
            "#,
            token_hash,
        )
        .fetch_optional(self.pool)
        .await
    }

    pub async fn revoke(
        &self,
        id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                UPDATE refresh_tokens
                SET revoked_at = NOW()
                WHERE id = $1
            "#,
            id
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }

    pub async fn revoke_all_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE user_id = $1
                AND revoked_at IS NULL
            "#,
            user_id
        )
        .execute(self.pool)
        .await?;

        Ok(())
    }
}