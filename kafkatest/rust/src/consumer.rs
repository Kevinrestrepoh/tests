use rdkafka::{
    ClientConfig, Message,
    consumer::{BaseConsumer, CommitMode, Consumer},
};

use crate::types::Event;

pub async fn start() {
    let consumer = create();
    consume(consumer).await;
}

fn create() -> BaseConsumer {
    let url = std::env::var("KAFKA_BROKER").unwrap_or("localhost:9092".to_string());

    let mut config = ClientConfig::new();
    config
        .set("bootstrap.servers", &url)
        .set("group.id", "go")
        .set("enable.auto.commit", "false")
        .set("auto.offset.reset", "earliest");

    config.create().expect("Failed to create consumer")
}

async fn consume(consumer: BaseConsumer) {
    consumer.subscribe(&["go_topic"]).expect("Cannot subscribe");

    loop {
        match consumer.poll(None) {
            None => {
                continue;
            }
            Some(Err(e)) => println!("Kafka error: {:?}", e),
            Some(Ok(msg)) => {
                if let Some(payload) = msg.payload() {
                    let event: Result<Event, _> = serde_json::from_slice(payload);
                    match event {
                        Ok(ev) => println!("Received struct: {:?}", ev),
                        Err(_) => println!("Invalid JSON"),
                    }
                } else {
                    println!("No message payload or invalid UTF-8");
                }
                consumer.commit_message(&msg, CommitMode::Async).unwrap();
            }
        }
    }
}
