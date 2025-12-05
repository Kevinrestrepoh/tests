use futures_lite::StreamExt;
use lapin::{
    Channel, Connection, ConnectionProperties,
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
};

use crate::models::Item;

pub async fn connect_rabbitmq(url: &str) -> Channel {
    let conn = Connection::connect(url, ConnectionProperties::default())
        .await
        .expect("Failed to connect RabbitMQ");

    conn.create_channel()
        .await
        .expect("Failed to create Channel")
}

pub async fn consume(ch: &Channel) {
    declare_exchange(ch).await;
    declare_queue_and_bind(ch).await;

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

async fn declare_exchange(channel: &Channel) {
    channel
        .exchange_declare(
            "test_exchange",
            lapin::ExchangeKind::Direct,
            ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                internal: false,
                nowait: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("Exchange declaration failed");
}

async fn declare_queue_and_bind(channel: &Channel) {
    let q = channel
        .queue_declare(
            "item_qeue",
            QueueDeclareOptions {
                durable: true,
                auto_delete: false,
                exclusive: false,
                nowait: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .expect("Queue declaration failed");

    channel
        .queue_bind(
            "item_qeue",
            "test_exchange",
            q.name().as_str(),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue binding failed");
}

async fn handle_msg(payload: &[u8]) {
    let _item: Item = match serde_json::from_slice(payload) {
        Ok(u) => u,
        Err(err) => {
            eprintln!("Failed to parse msg: {}", err);
            return;
        }
    };

    println!("Item received: {:?}", _item);
}
