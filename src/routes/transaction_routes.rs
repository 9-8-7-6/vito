use crate::handlers::transaction_handler::*;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn transaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/transactions",
            get(get_all_transactions_handler).post(add_transaction_handler),
        )
        .route(
            "/transactions/{id}",
            get(get_transaction_handler)
                .put(update_transaction_handler)
                .delete(delete_transaction_handler),
        )
        .with_state(state)
}
