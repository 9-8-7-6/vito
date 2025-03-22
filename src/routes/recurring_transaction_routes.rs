use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::recurringtransaction_handler::*;
use crate::models::Backend;

pub fn recurringtransaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/recurring_transactions",
            get(get_all_recurring_transactions_handler).post(add_recurring_transaction_handler),
        )
        .route(
            "/recurring_transactions/{id}",
            get(get_recurring_transaction_handler)
                .patch(update_recurring_transaction_handler)
                .delete(delete_recurring_transaction_handler),
        )
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state)
}
