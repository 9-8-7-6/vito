use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::user_handler::*;

/// Defines API routes for managing user records
pub fn user_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // GET  /users     -> Retrieve a list of all users
        // POST /users     -> Create a new user
        .route("/users", get(get_all_users_handler).post(add_user_handler))
        // GET    /users/{id} -> Retrieve a user by ID
        // PATCH  /users/{id} -> Update a user by ID
        // DELETE /users/{id} -> Delete a user by ID
        .route(
            "/users/{id}",
            get(get_user_handler)
                .patch(update_user_handler)
                .delete(delete_user_handler),
        )
        // Share the PostgreSQL connection pool with all handlers
        .with_state(state)
}
