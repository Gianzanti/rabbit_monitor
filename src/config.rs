// use core::panic;
use dotenvy::dotenv;
use std::env;
use thiserror::Error;
use tokio::time::Duration;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Environment variable not found/set: {0}")]
    MissingEnvironmentVariable(#[from] std::env::VarError),
}

pub struct Config {
    pub url: String,
    pub interval: Duration,
    pub username: String,
    pub password: String,
    pub filename: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let url = env::var("RABBIT_ENDPOINT_URL").or_else(|_| {
            dotenv().ok();
            env::var("RABBIT_ENDPOINT_URL")
        })?;
        let interval = Duration::from_secs(env::var("INTERVAL_SECS")?.parse::<u64>().unwrap_or(1));
        let username = env::var("RABBIT_USERNAME")?;
        let password = env::var("RABBIT_PASSWORD")?;
        let filename = env::var("OUTPUT_LOG").unwrap_or_else(|_| "data".to_string());
        let filename = format!("{}-{}.csv", filename, chrono::Utc::now().format("%Y-%m-%d"));
        Ok(Config {
            url,
            interval,
            username,
            password,
            filename: filename.to_string(),
        })
    }
}
