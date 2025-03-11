use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::asset_handler::*;
use crate::models::Backend;

pub fn asset_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/assets",
            get(get_all_assets_handler).post(add_asset_handler),
        )
        .route(
            "/assets/{id}",
            get(get_asset_handler)
                .put(update_asset_handler)
                .delete(delete_asset_handler),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state)
}
