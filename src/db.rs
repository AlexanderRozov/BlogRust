use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;
use crate::config::Config;

pub async fn connect(config: &Config) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}

