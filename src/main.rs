use anyhow::Result;
use colored::*;
use dotenvy::dotenv;
use reqwest::Client;
use std::env;
use tokio::time::{sleep, Duration};
// use tokio::signal;

mod csv_writer;
mod rabbit_response;

use rabbit_response::RabbitResponse;

#[tokio::main]
async fn main() -> Result<()> {
    // tratar o signal (tokio)
    // signal::ctrl_c().await?;

    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let url =
        env::var("RABBIT_ENDPOINT_URL").expect("RABBIT_ENDPOINT_URL environment variable not set");

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

    let mut writer = csv_writer::RabbitCSV::new(&filename);

    let client = Client::new();

    let request_url = format!("{}/api/queues/?page=1&page_size=10", url);

    loop {
        let mut resp = client
            .get(&request_url)
            .basic_auth(&username, Some(&password))
            .send()
            .await?
            .json::<RabbitResponse>()
            .await?;

        resp.items.iter_mut().for_each(|item| {
            // item.timestamp = Some(timestamp.clone());
            println!(
                "{:>.15} {:.<15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15}",
                &item.timestamp[5..19].to_string().blue(),
                &item.name.dimmed(),
                &item.memory,
                &item.message_bytes,
                &item.messages,
                &item.messages_ready,
                &item.messages_unacknowledged,
                &item.messages_details.rate,
                &item.messages_ready_details.rate,
                &item.messages_unacknowledged_details.rate
            );

            let _ = writer.csv_writer.serialize(&item);
            let _ = writer.csv_writer.flush();
        });

        sleep(interval).await;
    }

    // Ok(())
}
