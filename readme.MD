# Rabbit Monitor

## Description

This is a simple RabbitMQ monitor that will monitor the queues and print on the console the number of messages in each queue (and some statistics).
It will save the data in a CSV file.

## Configuration

The configuration is done via environment variables. You can set the following variables manually or create a `.env` file in the root of the project.

```bash
RABBIT_ENDPOINT_URL=http://127.0.0.1:8084 # The RabbitMQ management URL
RABBIT_USERNAME=admin # The RabbitMQ username
RABBIT_PASSWORD=password # The RabbitMQ password
INTERVAL_SECS=1 # The interval in seconds to check the queues
```

## Compilation

To compile the monitor, you can use the following command:

```bash
cargo build --release
```

## Running

To run the monitor, you can use the following command:

```bash
./target/release/rabbit-monitor-rust
```
