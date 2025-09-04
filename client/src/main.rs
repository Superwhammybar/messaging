use std::{thread, time};

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct CreateQueuePayload {
    name: String,
}
#[derive(Serialize, Deserialize)]
struct FetchPayload {
    pub count: usize,
    pub queue_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Message {
    pub id: String,
    pub body: String,
    pub timestamp: u64,
    pub delivery_attempts: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessagePayload {
    pub queue_name: String,
    pub ids: Vec<String>,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let queue_name = "test".to_string();
    let payload = FetchPayload {
        count: 1,
        queue_name: queue_name.clone(),
    };
    client
        .post("http://localhost:3000/queue")
        .json(&CreateQueuePayload {
            name: queue_name.clone(),
        })
        .send()
        .await
        .unwrap();
    loop {
        let response = client
            .post("http://localhost:3000/message/fetch")
            .json(&payload)
            .send()
            .await;
        let data = match response {
            Ok(res) => from_str::<Vec<Message>>(&res.text().await.unwrap()).unwrap(),
            Err(err) => {
                println!("Error {:?}", err);
                vec![]
            }
        };

        println!("Received {} messages", data.iter().len());
        for message in data {
            println!("{:?}", message);
            let retry = rand::thread_rng().gen_bool(1.0 / 4.0);
            let payload = MessagePayload {
                queue_name: queue_name.clone(),
                ids: vec![message.id],
            };

            client
                .post(format!(
                    "http://localhost:3000/message/{}",
                    if retry { "retry" } else { "remove" }
                ))
                .json(&payload)
                .send()
                .await
                .unwrap();
        }
        thread::sleep(time::Duration::from_secs(5));
    }
}
