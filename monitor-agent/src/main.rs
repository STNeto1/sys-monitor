use std::{env, thread, time::Duration};

use anyhow::Result;
use aws_sdk_sqs::Client;
use dotenv::dotenv;
use monitor_central::Stat;
use sysinfo::{System, SystemExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("No .env file found");

    let config = aws_config::load_from_env().await;
    let aws_client = Client::new(&config);

    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        let stat_data = Stat {
            system_identifier: sys.host_name().unwrap_or("unknown".to_string()),
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
            uptime: sys.uptime(),
        };

        match send_message(&aws_client, &stat_data).await {
            Ok(_) => println!("Sent message: {:?}", stat_data),
            Err(e) => {
                println!("Error sending message: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(10));
    }
}

async fn send_message(client: &Client, stat: &Stat) -> Result<()> {
    let _result = client
        .send_message()
        .queue_url(&env::var("SQS_QUEUE_URL").expect("SQS_QUEUE_URL must be set"))
        .message_body(&serde_json::to_string(stat)?)
        .send()
        .await?;

    Ok(())
}
