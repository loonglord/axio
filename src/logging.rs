use anyhow::Result;
use serde::Deserialize;
use std::io::Write;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub level: LogLevel,
    pub writer: LogWriter,
}

#[derive(Debug, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum LogWriter {
    #[serde(rename = "file")]
    File {
        directory: String,
        file_name_prefix: String,
    },
    #[serde(rename = "stdout")]
    Stdout,
}

impl LogLevel {
    pub fn to_tracing_level(&self) -> Level {
        match self {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

pub fn init_logging(config: &Config) -> Result<WorkerGuard> {
    let (writer, ansi): (Box<dyn Write + Send + 'static>, bool) = match &config.writer {
        LogWriter::File {
            directory,
            file_name_prefix,
        } => (
            Box::new(tracing_appender::rolling::daily(
                directory,
                file_name_prefix,
            )),
            false,
        ),
        LogWriter::Stdout => (Box::new(std::io::stdout()), true),
    };
    let (non_blocking, worker_guard) = tracing_appender::non_blocking(writer);
    let filter =
        EnvFilter::from_default_env().add_directive(config.level.to_tracing_level().into());
    let layer = tracing_subscriber::fmt::layer()
        .with_ansi(ansi)
        .with_writer(non_blocking);
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();
    Ok(worker_guard)
}
