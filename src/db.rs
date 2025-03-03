use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};

pub async fn init_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    if !Postgres::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        println!("Creating database...");
        Postgres::create_database(&database_url)
            .await
            .expect("Failed to create database");
    }

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("✅ Database migrations applied successfully!");

    pool
}
