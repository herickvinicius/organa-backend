use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
}