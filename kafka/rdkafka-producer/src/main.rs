use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};
use serde_json::json;

const TOPIC: &str = "speed-test";
const NUM_MESSAGES: i32 = 1000;

#[tokio::main]
async fn main() {
    let producer = ClientConfig::new()
        .set("bootstrap.servers", "10.13.37.32:9092")
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer<_>>()
        .expect("Producer creation error");

    let start_time = std::time::Instant::now();

    for n in 0..NUM_MESSAGES {
        let data = json!({
            "id": n,
            "message": "Hello, world!",
        });

        match producer
            .send::<String, String, Timeout>(
                FutureRecord::to(TOPIC).payload(&data.to_string()),
                Timeout::Never,
            )
            .await
        {
            Ok(_) => (),
            Err((e, _)) => {
                println!("Error while sending message: {}", e);
            }
        }
    }

    let end_time = std::time::Instant::now();
    let elapsed = end_time.duration_since(start_time);
    let num_sent: f64 = NUM_MESSAGES.into();
    println!(
        "Elapsed time: {:?}, messages/second {:?}",
        elapsed,
        num_sent / elapsed.as_secs_f64()
    );
}
