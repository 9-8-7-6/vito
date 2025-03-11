mod db;
mod handlers;
mod models;
mod repository;
mod routes;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{serve, Router};
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::models::Backend;

use routes::{
    account_routes::account_routes, asset_routes::asset_routes, category_routes::category_routes,
    login_logout_routes::login_routes, recurring_transaction_routes::recurringtransaction_routes,
    transaction_routes::transaction_routes, user_routes::user_routes,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL not set");

    let state = Arc::new(db::init_db(&database_url).await);
    let backend = Backend::new(&database_url).await.unwrap();
    let session_layer = db::init_redis(&redis_url).await;

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
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .merge(openapi_router)
        .merge(account_routes(state.clone()))
        .merge(user_routes(state.clone()))
        .merge(asset_routes(state.clone()))
        .merge(category_routes(state.clone()))
        .merge(recurringtransaction_routes(state.clone()))
        .merge(transaction_routes(state.clone()))
        .merge(login_routes(backend.clone()))
        .layer(CookieManagerLayer::new())
        .layer(session_layer);

    let addr: SocketAddr = "0.0.0.0:8000".parse().unwrap();
    println!("🚀 Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, routes_all).await.unwrap();
}
