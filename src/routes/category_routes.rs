use crate::handlers::category_handler::*;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn category_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/categories", get(get_all_categories).post(add_category))
        .route(
            "/categories/{id}",
            get(get_category)
                .put(update_category)
                .delete(delete_category_handler),
        )
        .with_state(state)
}
