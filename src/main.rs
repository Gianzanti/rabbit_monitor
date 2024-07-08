use anyhow::Result;
use colored::*;
use reqwest::Client;
use tokio::time::sleep;
// use tokio::signal;

mod config;
mod csv_writer;
mod rabbit_response;

#[tokio::main]
async fn main() -> Result<()> {
    // tratar o signal (tokio)
    // signal::ctrl_c().await?;

    let config = config::Config::new();

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

    let mut writer = csv_writer::RabbitCSV::new(&config.filename, &headers);

    let client = Client::new();

    let request_url = format!("{}/api/queues/?page=1&page_size=10", &config.url);

    loop {
        let mut resp = client
            .get(&request_url)
            .basic_auth(&config.username, Some(&config.password))
            .send()
            .await?
            .json::<rabbit_response::RabbitResponse>()
            .await?;

        print!("{}[2J", 27 as char);
        headers.iter().for_each(|header| {
            print!("{:^17} ", header.bold().bright_white());
        });
        println!();

        resp.items.iter_mut().enumerate().for_each(|(idx, item)| {
            let cont = format!(
                "|{:>15} | {:_<15} | {:_>15} | {:_>15} | {:_>15} | {:_>15} | {:_>15} | {:_>15} |",
                &item.timestamp.format("%m-%d %H:%M:%S").to_string(),
                &item.name,
                &item.messages_ready,
                &item.messages_unacknowledged,
                &item.messages,
                &item.message_stats.publish_details.rate,
                &item.message_stats.deliver_get_details.rate,
                &item.message_stats.ack_details.rate,
            );

            if idx % 2 == 0 {
                println!("{}", cont.bold().on_truecolor(200, 200, 200));
            } else {
                println!("{}", cont.bold().on_truecolor(150, 150, 255));
            }

            let _ = writer.csv_writer.serialize(&item);
            let _ = writer.csv_writer.flush();
        });
        print!("\n\n\n\n\n");

        sleep(config.interval).await;
    }
}
