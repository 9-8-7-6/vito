use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
use time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{fred::prelude::*, RedisStore};

pub async fn init_db(database_url: &str) -> PgPool {
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

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => println!("Migrations ran successfully."),
        Err(e) => eprintln!("Migration error: {}", e),
    }

    println!("✅ Database migrations applied successfully!");

    pool
}

pub async fn init_redis(redis_url: &str) -> SessionManagerLayer<RedisStore<Pool>> {
    let config = Config::from_url(&redis_url).expect("Failed to parse Redis URL");

    let pool = Pool::new(config, None, None, None, 6).expect("Failed to create Redis pool");
    let redis_conn = pool.connect();

    match pool.wait_for_connect().await {
        Ok(_) => println!("✅ Redis connected successfully, Result {:?}!", redis_conn),
        Err(e) => panic!("❌ Failed to connect to Redis: {:?}", e),
    }

    let session_store = RedisStore::new(pool);
    let session_layer: SessionManagerLayer<RedisStore<_>> = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    session_layer
}
