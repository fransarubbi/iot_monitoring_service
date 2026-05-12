use sqlx::{postgres::PgPoolOptions, PgPool};
use anyhow::Result;

pub async fn connect(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5) // No necesitás muchas para un watchdog
        .connect(database_url)
        .await?;

    Ok(pool)
}