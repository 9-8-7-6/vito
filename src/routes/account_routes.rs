use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::account_handler::*;
use crate::models::Backend;

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
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state)
}
