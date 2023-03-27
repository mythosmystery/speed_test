use std::thread::sleep;

use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::{BorrowedMessage, OwnedMessage},
    ClientConfig, Message,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageData {
    id: u32,
    message: String,
}

#[inline(always)]
async fn process_message(msg: OwnedMessage) {
    let payload = msg.payload_view::<str>().unwrap().unwrap();
    let message_data: MessageData = serde_json::from_str(payload).unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Received message: {:#?}", message_data);
}

// #[tokio::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "my-group")
        .set("bootstrap.servers", "10.13.37.32:9092")
        .set("enable.partition.eof", "false")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["speed-test"])
        .expect("Can't subscribe to specified topic");

    let start_time = std::time::Instant::now();

    let mut num_received = 0;

    loop {
        println!("Waiting for message... {}", num_received);
        let msg = consumer.recv().await;

        let msg = msg.unwrap().detach();

        tokio::spawn(async move {
            println!("Thread {} spawned", num_received);
            process_message(msg).await;
        });
        println!("Iteration {} done", num_received);
        num_received += 1;

        if num_received >= 1000 {
            sleep(std::time::Duration::from_secs(1));
            break;
        }
    }

    let end_time = std::time::Instant::now();

    let elapsed = end_time.duration_since(start_time);

    println!(
        "Elapsed time: {:?}, messages/sec: {:?}",
        elapsed,
        num_received as f64 / elapsed.as_secs_f64()
    );
}
