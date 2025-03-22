use axum::{
    routing::{get, patch, post},
    Router,
};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::transaction_handler::*;
use crate::models::Backend;

pub fn transaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/transactions", post(add_transaction_handler))
        .route(
            "/transactions/{id}",
            get(get_transaction_by_transaction_id_handler)
                .patch(update_transaction_handler)
                .delete(delete_transaction_handler),
        )
        .route(
            "/transactions/account/{id}",
            get(get_transaction_by_account_id_handler),
        )
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state)
}
