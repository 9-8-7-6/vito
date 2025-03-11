use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::user_handler::*;

pub fn user_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/users", get(get_all_users_handler).post(add_user_handler))
        .route(
            "/users/{id}",
            get(get_user_handler)
                .put(update_user_handler)
                .delete(delete_user_handler),
        )
        .with_state(state)
}
