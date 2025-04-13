mod db;
mod handlers;
mod models;
mod repository;
mod routes;
mod scheduler;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{serve, Router};
use axum_login::AuthManagerLayerBuilder;
use dotenvy::dotenv;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http::{HeaderValue, Method};

use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
// use utoipa_swagger_ui::SwaggerUi;

use crate::models::Backend;
use scheduler::start_all_schedulers;

use routes::{
    account_routes::account_routes, asset_routes::asset_routes, country_routes::country_routes,
    login_logout_routes::login_routes, recurring_transaction_routes::recurringtransaction_routes,
    stock_routes::stock_routes, transaction_routes::transaction_routes, user_routes::user_routes,
};

struct Url {
    database_url: String,
    redis_url: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let urls = Url {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        redis_url: std::env::var("REDIS_URL").expect("REDIS_URL not set"),
    };

    let state: Arc<sqlx::Pool<sqlx::Postgres>> = Arc::new(db::init_db(&urls.database_url).await);

    start_all_schedulers(state.clone()).await;

    let backend = Backend::new(&urls.database_url)
        .await
        .expect("Failed to initialize Backend: Check DATABASE_URL");

    let session_layer = db::init_redis(&urls.redis_url).await;
    let auth_layer = AuthManagerLayerBuilder::new(backend.clone(), session_layer.clone()).build();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        tags(
            (name = "vito", description = "vito items management API")
        )
    )]
    struct ApiDoc;

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
    let (openapi_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi()).split_for_parts();

    let routes_all = Router::new()
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(openapi_router)
        .merge(account_routes(state.clone()))
        .merge(user_routes(state.clone()))
        .merge(asset_routes(state.clone()))
        .merge(recurringtransaction_routes(state.clone()))
        .merge(transaction_routes(state.clone()))
        .merge(stock_routes(state.clone()))
        .merge(country_routes(state.clone()))
        .merge(login_routes(backend.clone()))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .layer(auth_layer)
        .layer(session_layer);

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, routes_all).await.unwrap();
}
