use crate::handlers::account_handler::*;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn account_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/accounts",
            get(get_all_accounts_handler).post(add_account_handler),
        )
        .route(
            "/accounts/{id}",
            get(get_account_handler)
                .put(update_account_handler)
                .delete(delete_account_handler),
        )
        .with_state(state)
}
