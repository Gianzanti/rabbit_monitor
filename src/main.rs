use anyhow::Result;
use csv::Writer;
use dotenvy::dotenv;
use reqwest::Client;
use std::{env, thread, time::Duration};
// use tokio::signal;

mod rabbit_response;
use rabbit_response::RabbitResponse;

#[tokio::main]
async fn main() -> Result<()> {
    // tratar o signal (tokio)
    // signal::ctrl_c().await?;

    dotenv().ok(); // This line loads the environment variables from the ".env" file.

    let url =
        env::var("RABBIT_ENDPOINT_URL").expect("RABBIT_ENDPOINT_URL environment variable not set");
    let interval = env::var("INTERVAL_SECS")
        .expect("INTERVAL_SECS environment variable not set")
        .parse::<u64>()
        .unwrap_or(1);
    let interval = Duration::from_secs(interval);

    let username =
        env::var("RABBIT_USERNAME").expect("RABBIT_USERNAME environment variable not set");
    let password =
        env::var("RABBIT_PASSWORD").expect("RABBIT_PASSWORD environment variable not set");

    let filename = "data.csv";

    let mut writer = Writer::from_path(filename)?;

    let headers = vec![
        "Timestamp",
        "Queue",
        "Memory",
        "M_Bytes",
        "M_Total",
        "M_Ready",
        "M_Unack",
        "M_Rate",
        "M_Ready Rate",
        "M_UnAck Rate",
    ];

    writer.write_record(&headers)?; // Write header

    headers.iter().for_each(|header| {
        print!("{:^15} ", header);
    });
    print!("\n");

    let client = Client::new();

    let request_url = format!("{}/api/queues/?page=1&page_size=10", url);

    loop {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let resp = client
            .get(&request_url)
            .basic_auth(&username, Some(&password))
            .send()
            .await?
            .json::<RabbitResponse>()
            .await?;

        resp.items.iter().for_each(|item| {
            println!(
                "{:>.15} {:.<15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15} {:.>15}",
                &timestamp[5..19].to_string(),
                &item.name,
                &item.memory,
                &item.message_bytes,
                &item.messages,
                &item.messages_ready,
                &item.messages_unacknowledged,
                &item.messages_details.rate,
                &item.messages_ready_details.rate,
                &item.messages_unacknowledged_details.rate
            );

            writer
                .write_record(&[
                    &timestamp,
                    &item.name,
                    &item.memory.to_string(),
                    &item.message_bytes.to_string(),
                    &item.messages.to_string(),
                    &item.messages_ready.to_string(),
                    &item.messages_unacknowledged.to_string(),
                    &item.messages_details.rate.to_string(),
                    &item.messages_ready_details.rate.to_string(),
                    &item.messages_unacknowledged_details.rate.to_string(),
                ])
                .unwrap();

            writer.flush().unwrap();
        });

        thread::sleep(interval);
    }

    // Ok(())
}
