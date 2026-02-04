use sqlx::PgPool;
use time::Duration;

#[derive(Clone)]
pub struct AppState {
  pub db_pool: PgPool,

  pub jwt_secret: String,
  pub access_token_ttl: Duration,
  pub refresh_token_ttl:Duration,
}