use anyhow::{Result, anyhow};
use redis::Client;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub url: String,
}

static POOL: OnceLock<bb8::Pool<Client>> = OnceLock::new();

pub async fn init_pool(config: &Config) -> Result<()> {
    let client = Client::open(config.url.as_str())?;
    let pool = bb8::Pool::builder().build(client).await?;
    POOL.set(pool)
        .map_err(|_| anyhow!("redis pool already initialized"))
}

pub async fn pool() -> Result<bb8::PooledConnection<'static, Client>> {
    Ok(POOL
        .get()
        .ok_or_else(|| anyhow!("redis pool is not initialized"))?
        .get()
        .await?)
}
