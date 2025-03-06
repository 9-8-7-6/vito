use crate::handlers::recurringtransaction_handler::*;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn recurringtransaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/recurring_transactions",
            get(get_all_recurring_transactions).post(add_recurring_transaction),
        )
        .route(
            "/recurring_transactions/{id}",
            get(get_recurring_transaction)
                .put(update_recurring_transaction_handler)
                .delete(delete_recurring_transaction_handler),
        )
        .with_state(state)
}
