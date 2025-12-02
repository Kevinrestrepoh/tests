use lapin::{
    Channel, Connection, ConnectionProperties,
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};

use crate::models::{self, Item};

pub async fn connect_rabbitmq(url: String) -> Channel {
    let conn = Connection::connect(url, ConnectionProperties::default())
        .await
        .expect("Failed to connect RabbitMQ");

    conn.create_channel()
        .await
        .expect("Failed to create Channel")
}

pub async fn consume(ch: &Channel) {
    declare_queue_and_bind(ch);

    let mut consumer = ch
        .basic_consume(
            "item_qeue",
            "rust_consuer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to start consumer");

    while let Some(d) = consumer.next().await {
        if let Ok(d) = d {
            handle_msg(&d.data).await;
            d.ack(BasicAckOptions::default()).await.unwrap();
        }
    }
}

async fn declare_queue_and_bind(channel: &Channel) {
    let q = channel
        .queue_declare(
            "item_qeue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue declaration failed");

    channel
        .queue_bind(
            "item_qeue",
            "exchange",
            q.name(),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue binding failed");
}

async fn handle_msg(payload: &[u8]) {
    let item: Item = match serde_json::from_slice(payload) {
        Ok(u) => u,
        Err(err) => {
            eprintln!("Failed to parse msg: {}", err);
            return;
        }
    };
}
