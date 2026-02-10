use anyhow::Result;
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{sync::OnceLock, time::Duration};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
}

static POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init_pool(config: &Config) -> Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout))
        .idle_timeout(Some(Duration::from_secs(config.idle_timeout)))
        .max_lifetime(Some(Duration::from_secs(config.max_lifetime)))
        .connect(config.url.as_str())
        .await?;
    POOL.set(pool)
        .map_err(|_| anyhow::anyhow!("postgres pool already initialized"))
}

pub fn pool() -> &'static PgPool {
    POOL.get().expect("postgres pool is not initialized")
}
