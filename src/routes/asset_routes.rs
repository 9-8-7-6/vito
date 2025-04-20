use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::asset_handler::*;

/// Defines routes for asset-related operations
pub fn asset_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // GET /assets     -> fetch all assets
        // POST /assets    -> create a new asset
        .route(
            "/assets",
            get(get_all_assets_handler).post(add_asset_handler),
        )
        // GET /assets/{id}     -> fetch all assets by user ID
        // PATCH /assets/{id}   -> update asset information
        // DELETE /assets/{id}  -> delete asset
        .route(
            "/assets/{id}",
            get(get_asset_handler)
                .patch(update_asset_handler)
                .delete(delete_asset_handler),
        )
        // Uncomment to enforce login on all asset routes
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state) // Share PgPool state with all route handlers
}
