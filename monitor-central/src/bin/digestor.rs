use anyhow::Result;
use aws_sdk_sqs::Client;
use dotenv::dotenv;
use monitor_central::{get_from_env, Stat};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("No .env file found");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&get_from_env("DATABASE_URL"))
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let config = aws_config::load_from_env().await;
    let aws_client = Client::new(&config);

    loop {
        let rcv_msg = aws_client
            .receive_message()
            .queue_url(&get_from_env("SQS_QUEUE_URL"))
            .send()
            .await?;

        for msg in rcv_msg.messages.unwrap_or_default() {
            let body = msg.body.unwrap_or_default();

            match serde_json::from_str::<Stat>(body.as_str()) {
                Ok(stat_msg) => {
                    match sqlx::query(
                        r#"
                    INSERT INTO stats (system_identifier, total_memory, used_memory, total_swap, used_swap, uptime, timestamp)
                    VALUES ($1, $2, $3, $4, $5, $6, $7);
                    "#,
                    )
                    .bind(&stat_msg.system_identifier)
                    .bind(&stat_msg.total_memory)
                    .bind(&stat_msg.used_memory)
                    .bind(&stat_msg.total_swap)
                    .bind(&stat_msg.used_swap)
                    .bind(&stat_msg.uptime)
                    .bind(&stat_msg.timestamp)
                    .execute(&pool)
                    .await {
                        Ok(_) => println!("Successfully inserted stat"),
                        Err(e) => println!("Error inserting stat: {}", e),
                    }
                }
                Err(e) => println!("Error parsing message: {}", e),
            }
        }
    }
}
