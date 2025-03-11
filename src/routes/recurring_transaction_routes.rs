use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::recurringtransaction_handler::*;

pub fn recurringtransaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/recurring_transactions",
            get(get_all_recurring_transactions_handler).post(add_recurring_transaction_handler),
        )
        .route(
            "/recurring_transactions/{id}",
            get(get_recurring_transaction_handler)
                .put(update_recurring_transaction_handler)
                .delete(delete_recurring_transaction_handler),
        )
        .with_state(state)
}
