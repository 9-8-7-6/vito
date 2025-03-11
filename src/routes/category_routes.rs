use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::category_handler::*;
use crate::models::Backend;

pub fn category_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/categories",
            get(get_all_categories_handler).post(add_category_handler),
        )
        .route(
            "/categories/{id}",
            get(get_category_handler)
                .put(update_category_handler)
                .delete(delete_category_handler),
        )
        .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state)
}
