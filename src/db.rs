use sqlx::PgPool;

pub async fn init_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}
