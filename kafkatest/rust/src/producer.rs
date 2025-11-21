use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};

use crate::types::Event;

pub fn create() -> FutureProducer {
    let url: &str = &std::env::var("KAFKA_BROKER").unwrap_or("localhost:9092".to_string());

    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", url);

    config.create().expect("Failed in creating producer")
}

pub async fn produce(future_producer: &FutureProducer, event: Event) {
    let data = serde_json::to_string(&event).unwrap();
    let record = FutureRecord::to("rust_topic").payload(&data).key("test");

    let status_delievery = future_producer.send(record, 2000).await;

    match status_delievery {
        Ok(report) => println!("Message sent {:?}", report),
        Err(e) => println!("Error {:?}", e),
    }
}
