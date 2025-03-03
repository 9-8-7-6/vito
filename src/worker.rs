use dotenvy::dotenv;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use sqlx::PgPool;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = Arc::new(
        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to DB"),
    );

    let conn = Connection::connect(
        "amqp://guest:guest@rabbitmq:5672/",
        ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect to RabbitMQ");

    let channel = conn
        .create_channel()
        .await
        .expect("Failed to create channel");

    let queue = channel
        .queue_declare(
            "tasks",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue");

    println!("📥 Waiting for messages...");

    let mut consumer = channel
        .basic_consume(
            "tasks",
            "rust_worker",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to start consumer");

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let message = String::from_utf8(delivery.data.clone()).unwrap();
            println!("✅ Received task: {}", message);

            if let Err(err) = store_task(pool.clone(), message).await {
                eprintln!("❌ Failed to store task: {:?}", err);
            }

            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack");
        }
    }
}

async fn store_task(pool: Arc<PgPool>, task: String) -> Result<(), sqlx::Error> {
    // sqlx::query!("INSERT INTO tasks (description) VALUES ($1)", task)
    // .execute(&*pool)
    // .await?;
    Ok(())
}
