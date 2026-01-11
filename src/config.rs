use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub session_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://blog:blog@localhost:5432/blog".to_string());
        
        let session_secret = env::var("SESSION_SECRET")
            .unwrap_or_else(|_| "dev-secret-key-change-in-production".to_string());
        
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        Self {
            database_url,
            session_secret,
            port,
        }
    }
}

