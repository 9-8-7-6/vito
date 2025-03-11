use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::transaction_handler::*;
use crate::models::Backend;

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
        .route(
            "/protected",
            get(|| async { "Gotta be logged in to see me!" }),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state)
}
