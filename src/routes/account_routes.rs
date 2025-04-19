use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::account_handler::*;
use crate::models::Backend;

/// Define routes for account-related operations
pub fn account_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // Route for listing all accounts and creating a new one
        .route(
            "/accounts",
            get(get_all_accounts_handler).post(add_account_handler),
        )
        // Route for retrieving, updating, or deleting a specific account by ID
        .route(
            "/accounts/{id}",
            get(get_account_handler)
                .patch(update_account_handler)
                .delete(delete_account_handler),
        )
        // Optional: Add authentication middleware to protect routes
        // .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state) // Share PgPool state with all handlers
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::{
        body::{Body, Bytes},
        http::{Request, StatusCode},
    };
    use rust_decimal_macros::dec;
    use serde_json::json;
    use sqlx::PgPool;
    use std::{env, sync::Arc};
    use tower::ServiceExt;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");
        let pool = PgPool::connect(&url).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        pool
    }

    async fn spawn_test_app(pool: Arc<PgPool>) -> Router {
        super::account_routes(pool)
    }

    async fn insert_test_user(pool: &PgPool) -> Uuid {
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, username, email, hashed_password) VALUES ($1, $2, $3, $4)",
        )
        .bind(user_id)
        .bind(format!("user_{}", &user_id.to_string()[..8]))
        .bind(format!("{}@example.com", &user_id.to_string()[..8]))
        .bind("hashed_pw")
        .execute(pool)
        .await
        .unwrap();
        user_id
    }

    #[tokio::test]
    async fn test_account_routes_crud() {
        let pool = Arc::new(setup_test_db().await);
        let app = spawn_test_app(pool.clone()).await;
        let user_id = insert_test_user(&pool).await;

        // Create account
        let payload = json!({"user_id": user_id, "balance": "1000.00"});
        let req = Request::post("/accounts")
            .header("Content-Type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Get all accounts
        let req = Request::get("/accounts").body(Body::empty()).unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Get specific account
        let req = Request::get(format!("/accounts/{}", user_id))
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
        let account: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(account["balance"], "1000.00");

        // Update account
        let update_payload = json!({"balance": "2000.00"});
        let req = Request::patch(format!("/accounts/{}", user_id))
            .header("Content-Type", "application/json")
            .body(Body::from(update_payload.to_string()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Delete account
        let req = Request::delete(format!("/accounts/{}", user_id))
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
