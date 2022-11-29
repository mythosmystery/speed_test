use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};
use serde_json::json;

const MESSAGE_COUNT: u32 = 99999;
fn main() {
    let mut producer = Producer::from_hosts(vec!["localhost:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let mut buf = String::with_capacity(2);
    let start_time = std::time::Instant::now();

    for i in 0..MESSAGE_COUNT {
        let value = json!({
            "id": i,
            "message": "Hello, world!",
        });

        producer
            .send(&Record::from_value(
                "speed-topic",
                value.to_string().as_bytes(),
            ))
            .unwrap();
        buf.clear();
    }

    println!(
        "Elapsed time: {:?}, Time per message: {:?}",
        start_time.elapsed(),
        start_time.elapsed() / MESSAGE_COUNT
    );
}
