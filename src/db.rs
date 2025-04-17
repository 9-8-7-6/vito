use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
use time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{fred::prelude::*, RedisStore};

/// Initializes the PostgreSQL database connection and runs migrations
pub async fn init_db(database_url: &str) -> PgPool {
    // Create the database if it does not exist
    if !Postgres::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        println!("Creating database...");
        Postgres::create_database(&database_url)
            .await
            .expect("Failed to create database");
    }

    // Connect to the database
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Apply SQL migrations from the migrations directory
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => println!("Migrations ran successfully."),
        Err(e) => eprintln!("Migration error: {}", e),
    }

    println!("Database migrations applied successfully!");

    pool
}

/// Initializes a Redis session manager using tower-sessions and RedisStore
pub async fn init_redis(redis_url: &str) -> SessionManagerLayer<RedisStore<Pool>> {
    // Parse Redis configuration from URL
    let config = Config::from_url(&redis_url).expect("Failed to parse Redis URL");

    // Create a Redis connection pool
    let pool = Pool::new(config, None, None, None, 6).expect("Failed to create Redis pool");

    // Start connecting to Redis in the background
    let redis_conn = pool.connect();

    // Wait until Redis connection is ready
    match pool.wait_for_connect().await {
        Ok(_) => println!("Redis connected successfully, Result {:?}!", redis_conn),
        Err(e) => panic!("Failed to connect to Redis: {:?}", e),
    }

    // Initialize the session store using Redis
    let session_store = RedisStore::new(pool);

    // Build a session manager layer with 7-day inactivity expiry
    let session_layer: SessionManagerLayer<RedisStore<_>> = SessionManagerLayer::new(session_store)
        .with_secure(false) // Set to true in production to enforce HTTPS cookies
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    session_layer
}
