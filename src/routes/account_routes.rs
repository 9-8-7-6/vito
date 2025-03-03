use crate::handlers::account_handler::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn account_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/accounts", get(get_all_accounts))
        .route("/accounts/{id}", get(get_account))
        .route("/accounts", post(add_account))
        .route("/accounts/{id}", put(update_account))
        .route("/accounts/{id}", delete(delete_account_handler))
        .with_state(state)
}
