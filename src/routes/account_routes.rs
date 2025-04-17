use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::account_handler::*;
use crate::models::Backend;

/// Define routes for account-related operations
pub fn account_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // Route for listing all accounts and creating a new one
        .route(
            "/accounts",
            get(get_all_accounts_handler).post(add_account_handler),
        )
        // Route for retrieving, updating, or deleting a specific account by ID
        .route(
            "/accounts/{id}",
            get(get_account_handler)
                .patch(update_account_handler)
                .delete(delete_account_handler),
        )
        // Optional: Add authentication middleware to protect routes
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state) // Share PgPool state with all handlers
}
