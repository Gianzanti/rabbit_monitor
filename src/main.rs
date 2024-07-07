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

    let headers = vec![
        "Timestamp",
        "Queue",
        "M_Ready",
        "M_Unack",
        "M_Total",
        "Rate Incoming",
        "Rate Del/Get",
        "Rate Ack",
    ];

    let mut writer = csv_writer::RabbitCSV::new(&filename, &headers);

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

        print!("{}[2J", 27 as char);
        println!("Headers");

        // Write header
        headers.iter().for_each(|header| {
            print!("{:^15} ", header.bright_white());
        });
        print!("\n");

        let mut idx = 0;
        resp.items.iter_mut().for_each(|item| {
            let cont = format!(
                "{:>15} {:_<15} {:_>15} {:_>15} {:_>15} {:_>15} {:_>15} {:_>15}",
                &item.timestamp.format("%m-%d %H:%M:%S").to_string().blue(),
                &item.name.dimmed(),
                &item.messages_ready,
                &item.messages_unacknowledged,
                &item.messages,
                &item.message_stats.publish_details.rate,
                &item.message_stats.deliver_get_details.rate,
                &item.message_stats.ack_details.rate,
            );

            if idx % 2 == 0 {
                println!("{cont}");
            } else {
                println!("{}", cont.bold().on_purple());
            }

            let _ = writer.csv_writer.serialize(&item);
            let _ = writer.csv_writer.flush();

            idx += 1;
        });
        print!("\n");
        print!("\n");
        print!("\n");
        print!("\n");
        print!("\n");

        sleep(interval).await;
    }
}
