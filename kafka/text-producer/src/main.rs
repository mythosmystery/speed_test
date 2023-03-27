use std::sync::Arc;

use bonfire_dependencies::models::twilio_model::TwilioNewIncomingMessage;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() {
    let num_sent: f64 = 10000.0;

    let producer: FutureProducer = rdkafka::ClientConfig::new()
        .set("bootstrap.servers", "10.13.37.32:9092")
        .set("message.timeout.ms", "5000")
        .set("partitioner", "consistent_random")
        .create()
        .expect("Producer creation error");

    let producer_ref = Arc::new(producer);

    let start_time = std::time::Instant::now();

    let mut handles = vec![];

    for i in 0..num_sent as i32 {
        let test_data = TwilioNewIncomingMessage {
            body: format!("Test message {}", i),
            from: "+15005550006".to_string(),
            to: "+15207809324".to_string(),
            contact_list_id: "633b8a2ebac159a2946ddfb5".to_string(),
            org_id: "625f1aad2bb341c0cea11c39".to_string(),
            sender_id: "623333ddca5822ca4e19404e".to_string(),
            files: vec![],
        };

        let test_string = serde_json::to_string(&test_data).unwrap();

        let producer = producer_ref.clone();

        let handle = tokio::spawn(async move {
            let record: FutureRecord<'_, String, String> =
                FutureRecord::to("send-text").payload(&test_string);

            match producer
                .send::<String, String, Timeout>(record, Timeout::Never)
                .await
            {
                Ok(_) => (),
                Err((e, _)) => {
                    println!("Error while sending message: {}", e);
                }
            }
        });
        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    let end_time = std::time::Instant::now();
    let elapsed = end_time.duration_since(start_time);

    println!(
        "Elapsed time: {:?}, messages/second {:?}",
        elapsed,
        num_sent / elapsed.as_secs_f64()
    );
}
