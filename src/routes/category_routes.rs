use crate::handlers::category_handler::*;
use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn category_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/categories", get(get_all_categories_handler).post(add_category_handler))
        .route(
            "/categories/{id}",
            get(get_category_handler)
                .put(update_category_handler)
                .delete(delete_category_handler),
        )
        .with_state(state)
}
