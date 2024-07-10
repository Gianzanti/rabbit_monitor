// use core::panic;
use dotenvy::dotenv;
use std::env;
use thiserror::Error;
use tokio::time::Duration;

#[derive(Debug, Error)]
#[error("Environment variable not found/set: {0}")]
pub struct MissingEnvironmentVariableError(String);

pub struct Config {
    pub url: String,
    pub interval: Duration,
    pub username: String,
    pub password: String,
    pub filename: String,
}

impl Config {
    pub fn new() -> Result<Self, MissingEnvironmentVariableError> {
        let url = env::var("RABBIT_ENDPOINT_URL").unwrap_or_else(|_| {
            dotenv().ok();
            // env::var("RABBIT_ENDPOINT_URL")?;
            env::var("RABBIT_ENDPOINT_URL")
                .expect("env var `RABBIT_ENDPOINT_URL` should be set in bash or in `.env` file!")

            // env::var("RABBIT_ENDPOINT_URL")
            //     .map_err(|_| MissingEnvironmentVariableError("RABBIT_ENDPOINT_URL".to_string()))

            // env::var("RABBIT_ENDPOINT_URL").unwrap_or_else(|_| {
            //     panic!("env var `RABBIT_ENDPOINT_URL` should be set in bash or in `.env` file!");
            //     Err("teste")?
            //     // Err(MissingEnvironmentVariableError(
            //     //     "RABBIT_ENDPOINT_URL".to_string(),
            //     // ))
            // })
            // .expect("env var `RABBIT_ENDPOINT_URL` should be set in bash or in `.env` file!")
            // match env::var("RABBIT_ENDPOINT_URL") {
            //     Ok(url) => url,
            //     Err(_) => Err(MissingEnvironmentVariableError(
            //         "RABBIT_ENDPOINT_URL".to_string(),
            //     )),
        });

        let interval = Duration::from_secs(
            env::var("INTERVAL_SECS")
                .expect("env var `INTERVAL_SECS` should be set in bash or in `.env` file!")
                .parse::<u64>()
                .unwrap_or(1),
        );

        let username = env::var("RABBIT_USERNAME")
            .expect("env var `RABBIT_USERNAME` should be set in bash or in `.env` file!");
        let password = env::var("RABBIT_PASSWORD")
            .expect("env var `RABBIT_PASSWORD` should be set in bash or in `.env` file!");

        let filename = env::var("OUTPUT_LOG").unwrap_or("data".to_string());
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
