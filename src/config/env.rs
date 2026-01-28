use std::env;

pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env()  -> Self {
        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("APP_PORT must be a valid u16");

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        Self {
            port,
            database_url,
        }
    }
}