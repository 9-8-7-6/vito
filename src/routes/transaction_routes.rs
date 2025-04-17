use axum::{
    routing::{get, patch, post},
    Router,
};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::transaction_handler::*;
use crate::models::Backend;

/// Defines routes for managing financial transactions
pub fn transaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // POST /transactions
        // -> Create a new transaction and apply balance changes
        .route("/transactions", post(add_transaction_handler))

        // GET    /transactions/{id} -> Retrieve a transaction by ID
        // PATCH  /transactions/{id} -> Update a transaction and re-calculate balances
        // DELETE /transactions/{id} -> Delete a transaction and roll back balances
        .route(
            "/transactions/{id}",
            get(get_transaction_by_transaction_id_handler)
                .patch(update_transaction_handler)
                .delete(delete_transaction_handler),
        )

        // GET /transactions/account/{id}
        // -> Retrieve all enriched transactions for a specific account
        .route(
            "/transactions/account/{id}",
            get(get_transaction_by_account_id_handler),
        )

        // Optional: Enable this line to restrict transaction routes to authenticated users
        // .route_layer(login_required!(Backend, login_url = "/login"))

        // Share the database connection pool with all route handlers
        .with_state(state)
}
