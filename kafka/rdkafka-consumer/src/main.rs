use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageData {
    id: u32,
    message: String,
}

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "my-group")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["speed-test"])
        .expect("Can't subscribe to specified topic");

    loop {
        let msg = consumer.recv().await;

        match msg {
            Ok(m) => {
                let payload = m.payload_view::<str>().unwrap().unwrap();
                let message_data: MessageData = serde_json::from_str(payload).unwrap();
                println!("Received message: {:#?}", message_data);
            }
            Err(e) => {
                println!("Error while receiving message: {}", e);
            }
        }
    }
}
