use axum::{
    routing::{get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::core::currency::currency_holding_handler::*;

/// Defines routes for managing currency holdings
pub fn currency_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // GET /currency-holding/account/{account_id}
        // -> Retrieve all currency holdings for a specific account
        .route(
            "/currency-holding/account/{account_id}",
            get(get_currency_holdings_by_account_handler),
        )
        // POST /currency-holding
        // -> Create or update a currency holding
        .route("/currency-holding", post(create_currency_holding_handler))
        // PUT /currency-holding/{id}
        // -> Update specific currency holding (e.g., amount_held or average price)
        // DELETE /currency-holding/{id}
        // -> Delete a currency holding by ID
        .route(
            "/currency-holding/{id}",
            put(update_currency_holding_handler).delete(delete_currency_holding_handler),
        )
        .with_state(state)
}
