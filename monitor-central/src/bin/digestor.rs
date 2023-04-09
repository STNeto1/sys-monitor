use std::env;

use anyhow::Result;
use aws_sdk_sqs::Client;
use dotenv::dotenv;
use monitor_central::Stat;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("No .env file found");

    let config = aws_config::load_from_env().await;
    let aws_client = Client::new(&config);

    loop {
        let rcv_msg = aws_client
            .receive_message()
            .queue_url(&env::var("SQS_QUEUE_URL").expect("SQS_QUEUE_URL must be set"))
            .send()
            .await?;

        for msg in rcv_msg.messages.unwrap_or_default() {
            let body = msg.body.unwrap_or_default();

            match serde_json::from_str::<Stat>(body.as_str()) {
                Ok(stat_msg) => println!("Received message: {:?}", stat_msg),
                Err(e) => println!("Error parsing message: {}", e),
            }
        }
    }
}
