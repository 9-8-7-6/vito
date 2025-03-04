use crate::handlers::user_handler::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn user_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        .route("/users", get(get_all_users))
        .route("/users/{id}", get(get_user))
        .route("/users", post(add_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user_handler))
        .with_state(state)
}
