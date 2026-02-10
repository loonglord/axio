use crate::{config::Config, logging, postgres, redis, serve};
use anyhow::{Context, Result};
use axum::Router;
use tracing_appender::non_blocking::WorkerGuard;

type TaskHandle = tokio::task::JoinHandle<Result<()>>;

pub struct AppBuilder {
    config: Config,
    router_fn: Option<Box<dyn FnOnce() -> Router + Send + Sync>>,
    pre_run_fn: Option<Box<dyn FnOnce() -> TaskHandle + Send + Sync>>,
}

impl AppBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            router_fn: None,
            pre_run_fn: None,
        }
    }

    pub fn with_router<F>(mut self, callback: F) -> Self
    where
        F: FnOnce() -> Router + Send + Sync + 'static,
    {
        self.router_fn = Some(Box::new(callback));
        self
    }

    pub fn before_run<F>(mut self, callback: F) -> Self
    where
        F: FnOnce() -> TaskHandle + Send + Sync + 'static,
    {
        self.pre_run_fn = Some(Box::new(callback));
        self
    }

    pub async fn run(self) -> Result<WorkerGuard> {
        postgres::init_pool(&self.config.postgres)
            .await
            .with_context(|| "postgres initialization failed")?;

        redis::init_pool(&self.config.redis)
            .await
            .with_context(|| "redis initialization failed")?;

        if let Some(callback) = self.pre_run_fn {
            let _ = callback().await?;
        }
        let worker_guard = logging::init_logging(&self.config.logging)
            .with_context(|| "logging initialization failed")?;
        let router = self
            .router_fn
            .map(|callback| callback())
            .unwrap_or_else(|| {
                Router::new().route("/", axum::routing::get(|| async { "Hello, axio!" }))
            });
        serve::serve(&self.config.serve, router)
            .await
            .with_context(|| "service startup failed")?;

        Ok(worker_guard)
    }
}
