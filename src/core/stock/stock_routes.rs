use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::core::stock::stock_handler::*;

/// Defines routes for managing stock holdings and stock metadata
pub fn stock_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // GET /stock-holding/account/{account_id}
        // -> Retrieve all stock holdings for a specific account
        .route(
            "/stock-holding/account/{account_id}",
            get(get_stock_holdings_by_account_handler),
        )
        // POST /stock-holding
        // -> Create a new stock holding
        .route("/stock-holding", post(create_stock_holding_handler))
        // PUT /stock-holding/{id}
        // -> Update a specific stock holding by ID
        // DELETE /stock-holding/{id}
        // -> Delete a specific stock holding by ID
        .route(
            "/stock-holding/{id}",
            put(update_stock_holding_handler).delete(delete_stock_holding_handler),
        )
        // GET /stock-metadata
        // -> Retrieve all stock metadata entries
        .route("/stock-metadata", get(get_all_stock_metadata_handler))
        // GET /stock-metadata/{id}
        // -> Get specific stock metadata by ID
        // PUT /stock-metadata/{id}
        // -> Update stock metadata by ID
        // DELETE /stock-metadata/{id}
        // -> Delete stock metadata by ID
        .route(
            "/stock-metadata/{id}",
            get(get_stock_metadata_by_id_handler)
                .put(update_stock_metadata_handler)
                .delete(delete_stock_metadata_handler),
        )
        // Optional: Require authentication for all stock-related routes
        // .route_layer(login_required!(Backend, login_url = "/login"))
        // Inject shared database pool into all route handlers
        .with_state(state)
}
