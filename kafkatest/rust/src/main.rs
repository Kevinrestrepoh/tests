use producer::produce;
use types::Event;

mod consumer;
mod producer;
mod types;

#[tokio::main]
async fn main() {
    let producer = producer::create();

    for i in 0..20 {
        let event: Event = Event {
            id: i,
            msg: format!("Event #{}", i),
        };
        produce(&producer, event).await;
    }

    consumer::start().await;
}
