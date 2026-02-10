pub mod app;
pub mod config;
pub mod error;
pub mod logging;
pub mod middleware;
pub mod postgres;
pub mod redis;
pub mod serve;
pub mod validation;

pub type AppResult<T> = Result<T, error::Error>;
