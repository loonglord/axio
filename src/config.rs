use crate::{
    logging::Config as LoggingConfig, postgres::Config as PostgresConfig,
    redis::Config as RedisConfig, serve::Config as ServeConfig,
};
use anyhow::Result;
use serde::Deserialize;

macro_rules! deserialize_section {
    ($name:ident, $type:ty, $context:expr) => {
        fn $name<'de, D>(deserializer: D) -> std::result::Result<$type, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            <$type>::deserialize(deserializer)
                .map_err(|e| serde::de::Error::custom(format!("{}: {}", $context, e)))
        }
    };
}

deserialize_section!(deserialize_serve, ServeConfig, "[serve]");
deserialize_section!(deserialize_logging, LoggingConfig, "[logging]");
deserialize_section!(deserialize_postgres, PostgresConfig, "[postgres]");
deserialize_section!(deserialize_redis, RedisConfig, "[redis]");

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(deserialize_with = "deserialize_serve")]
    pub serve: ServeConfig,
    #[serde(deserialize_with = "deserialize_logging")]
    pub logging: LoggingConfig,
    #[serde(deserialize_with = "deserialize_postgres")]
    pub postgres: PostgresConfig,
    #[serde(deserialize_with = "deserialize_redis")]
    pub redis: RedisConfig,
}
impl Config {
    pub fn load(name: &str) -> Result<Self> {
        Ok(::config::Config::builder()
            .add_source(::config::File::with_name(name))
            .build()?
            .try_deserialize()?)
    }
}
