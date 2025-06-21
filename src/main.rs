mod core;
mod db;
mod models;
mod repository;
mod scheduler;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Body;
use axum::http::Request;
use axum::response::Response;
use axum::routing::get;
use axum::{
    middleware::{self, Next},
    serve, Json, Router,
};
use log::info;
use std::time::Instant;

use axum_login::AuthManagerLayerBuilder;
use dotenvy::dotenv;
use http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE};
use http::Method;
use tower_http::trace::TraceLayer;
use tracing_subscriber;

use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
// use utoipa_swagger_ui::SwaggerUi;

use crate::models::Backend;
use scheduler::start_all_schedulers;

use crate::core::account::account_routes::account_routes;
use crate::core::account::login_logout_routes::login_routes;
use crate::core::asset::asset_routes::asset_routes;
use crate::core::country::country_routes::country_routes;
use crate::core::recurring_transaction::recurring_transaction_routes::recurringtransaction_routes;
use crate::core::stock::stock_routes::stock_routes;
use crate::core::transaction::transaction_routes::transaction_routes;
use crate::core::user::user_routes::user_routes;
use crate::db::pool;

/// Struct for holding environment-provided service URLs
struct Url {
    database_url: String,
    redis_url: String,
}

async fn log_all(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    info!("--> {} {}", method, uri);

    let start = Instant::now();
    let resp = next.run(req).await;
    let elapsed = start.elapsed();
    info!("<-- {} {} ({} μs)", method, uri, elapsed.as_micros());

    resp
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    // Load database and Redis URLs from .env
    let urls = Url {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        redis_url: std::env::var("REDIS_URL").expect("REDIS_URL not set"),
    };

    // Initialize Postgres connection and run migrations
    let state: Arc<sqlx::Pool<sqlx::Postgres>> = Arc::new(pool::init_db(&urls.database_url).await);

    // Start all scheduled background jobs (e.g., stock metadata updates)
    start_all_schedulers(state.clone()).await;

    // Initialize backend logic for axum-login (e.g., user/password auth)
    let backend = Backend::new(&urls.database_url)
        .await
        .expect("Failed to initialize Backend: Check DATABASE_URL");

    // Set up Redis session storage and session expiration policy
    let session_layer = pool::init_redis(&urls.redis_url).await;

    // Set up login authentication middleware with Redis-backed sessions
    let auth_layer = AuthManagerLayerBuilder::new(backend.clone(), session_layer.clone()).build();

    // Configure CORS for frontend on http://localhost:5173 (e.g., Vite or Vue dev server)
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(vec![
            "http://localhost:5173".parse().unwrap(),
            "http://3.107.148.36:5173".parse().unwrap(),
            "https://vito-tw.com".parse().unwrap(),
        ]))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCESS_CONTROL_ALLOW_ORIGIN])
        .allow_credentials(true);

    // OpenAPI documentation configuration
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        tags(
            (name = "vito", description = "vito items management API")
        )
    )]
    struct ApiDoc;

    // Adds security scheme to OpenAPI spec
    struct SecurityAddon;
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("vito_apikey"))),
                )
            }
        }
    }

    // Split OpenAPI documentation router and spec
    let (openapi_router, _api) = OpenApiRouter::with_openapi(ApiDoc::openapi()).split_for_parts();

    async fn health_check() -> Json<&'static str> {
        Json("OK")
    }

    // Compose all routes into one main router
    let routes_all = Router::new()
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone())) // Optional Swagger UI
        .route("/healthz", get(health_check))
        .merge(openapi_router) // OpenAPI JSON output
        .merge(account_routes(state.clone()))
        .merge(user_routes(state.clone()))
        .merge(asset_routes(state.clone()))
        .merge(recurringtransaction_routes(state.clone()))
        .merge(transaction_routes(state.clone()))
        .merge(stock_routes(state.clone()))
        .merge(country_routes(state.clone()))
        .merge(login_routes(backend.clone()))
        .layer(middleware::from_fn(log_all))
        .layer(CookieManagerLayer::new()) // Enable cookie support
        .layer(auth_layer) // Enable login session middleware
        .layer(session_layer) // Enable Redis session store
        .layer(cors) // Apply CORS
        .layer(TraceLayer::new_for_http());

    // Bind and serve the application
    let addr: SocketAddr = "[::]:8000".parse().unwrap();
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, routes_all).await.unwrap();
}
