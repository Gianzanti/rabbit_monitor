use dotenvy::dotenv;
use std::env;
use tokio::time::Duration;

pub struct Config {
    pub url: String,
    pub interval: Duration,
    pub username: String,
    pub password: String,
    pub filename: String,
}

impl Config {
    pub fn new() -> Self {
        // check if the environment variables are set
        match env::var("RABBIT_ENDPOINT_URL") {
            Ok(_) => {}
            Err(_) => {
                println!("Environment variables not set, reading .env file ...");
                dotenv().ok();
            }
        }

        let url = env::var("RABBIT_ENDPOINT_URL")
            .expect("RABBIT_ENDPOINT_URL environment variable not set");

        let interval = Duration::from_secs(
            env::var("INTERVAL_SECS")
                .expect("INTERVAL_SECS environment variable not set")
                .parse::<u64>()
                .unwrap_or(1),
        );

        let username =
            env::var("RABBIT_USERNAME").expect("RABBIT_USERNAME environment variable not set");
        let password =
            env::var("RABBIT_PASSWORD").expect("RABBIT_PASSWORD environment variable not set");

        let filename = "data.csv";

        Config {
            url,
            interval,
            username,
            password,
            filename: filename.to_string(),
        }
    }
}
