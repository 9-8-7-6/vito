use crate::handlers::transaction_handler::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn transaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/transactions", get(get_all_transactions))
        .route("/transactions/{id}", get(get_transaction))
        .route("/transactions", post(add_transaction))
        .route("/transactions/{id}", put(update_transaction))
        .route("/transactions/{id}", delete(delete_transaction_handler))
        .with_state(state)
}
