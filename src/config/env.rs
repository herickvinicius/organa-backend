use std::env;
use time::Duration;

pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
    pub access_token_ttl: Duration,
    pub refresh_token_ttl: Duration,
}

impl AppConfig {
    pub fn from_env()  -> Self {
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("APP_PORT must be a valid u16");

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        
        let jwt_secret = env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set");
        
        let access_token_ttl = env::var("ACCESS_TOKEN_TTL")
            .expect("ACCESS_TOKEN_TTL not set")
            .parse::<i64>()
            .expect("ACCESS_TOKEN_TTL must be an integer");

        let refresh_token_ttl = env::var("REFRESH_TOKEN_TTL")
            .expect("REFRESH_TOKEN_TTL not set")
            .parse::<i64>()
            .expect("REFRESH_TOKEN_TTL must be an integer");

        Self {
            port,
            database_url,
            jwt_secret,
            access_token_ttl: Duration::seconds(access_token_ttl),
            refresh_token_ttl: Duration::seconds(refresh_token_ttl),
        }
    }
}