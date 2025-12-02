mod models;
mod rabbitmq;

#[tokio::main]
async fn main() {
    let ch = rabbitmq::connect_rabbitmq("amqp://admin:admin@localhost:5672/");

    rabbitmq::consume(&ch).await;
}
