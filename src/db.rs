use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::{Result, Context};
use crate::config::Config;
use std::time::Duration;
use tokio::time::sleep;

pub async fn connect(config: &Config) -> Result<PgPool> {
    const MAX_RETRIES: u32 = 10;
    const INITIAL_DELAY_SECS: u64 = 2;

    let mut last_error = None;
    
    for attempt in 0..MAX_RETRIES {
        match PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&config.database_url)
            .await
        {
            Ok(pool) => {
                tracing::info!("Successfully connected to database");
                
                // Run migrations
                sqlx::migrate!("./migrations")
                    .run(&pool)
                    .await
                    .context("Failed to run database migrations")?;
                
                tracing::info!("Database migrations completed successfully");
                return Ok(pool);
            }
            Err(e) => {
                last_error = Some(e);
                if attempt < MAX_RETRIES - 1 {
                    let delay = INITIAL_DELAY_SECS * (attempt + 1) as u64;
                    tracing::warn!(
                        "Failed to connect to database (attempt {}/{}), retrying in {}s...",
                        attempt + 1,
                        MAX_RETRIES,
                        delay
                    );
                    sleep(Duration::from_secs(delay)).await;
                }
            }
        }
    }

    Err(last_error.unwrap())
        .context("Failed to connect to database after multiple retries")
}

