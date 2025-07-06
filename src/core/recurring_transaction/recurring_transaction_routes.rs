use axum::{routing::get, Router};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{core::recurring_transaction::recurringtransaction_handler::*, models::Backend};

/// Defines routes for managing recurring transactions
pub fn recurringtransaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // GET    /recurring_transactions -> Fetch all recurring transactions
        // POST   /recurring_transactions -> Create a new recurring transaction
        .route(
            "/recurring_transactions",
            get(get_all_recurring_transactions_handler).post(add_recurring_transaction_handler),
        )
        // GET    /recurring_transactions/{id} -> Fetch one by ID
        // PATCH  /recurring_transactions/{id} -> Update by ID
        // DELETE /recurring_transactions/{id} -> Delete by ID
        .route(
            "/recurring_transactions/{id}",
            get(get_recurring_transaction_handler)
                .patch(update_recurring_transaction_handler)
                .delete(delete_recurring_transaction_handler),
        )
        // Enable this to protect routes with login middleware
        .route_layer(login_required!(Backend, login_url = "/login"))
        .with_state(state) // Inject database connection into all handlers
}
#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use chrono::Utc;
    use rust_decimal_macros::dec;
    use serde_json::json;
    use sqlx::PgPool;
    use std::{env, sync::Arc, usize};
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::repository::create_account;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&url).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        pool
    }

    async fn spawn_test_app(pool: Arc<PgPool>) -> Router {
        super::recurringtransaction_routes(pool)
    }

    async fn insert_user_and_asset(pool: &PgPool) -> (Uuid, Uuid) {
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, username, email, hashed_password) VALUES ($1, $2, $3, $4)",
        )
        .bind(user_id)
        .bind(format!("user_{}", &user_id.to_string()[..8]))
        .bind(format!("{}@test.com", &user_id.to_string()[..8]))
        .bind("hashed_pw")
        .execute(pool)
        .await
        .unwrap();

        create_account(pool, user_id, dec!(0.00)).await.unwrap();

        let asset_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at)
                     VALUES ($1, $2, $3, $4, now(), now())",
        )
        .bind(asset_id)
        .bind(user_id)
        .bind("cash")
        .bind(dec!(1000.00))
        .execute(pool)
        .await
        .unwrap();

        (user_id, asset_id)
    }

    #[tokio::test]
    async fn test_recurring_transaction_crud() {
        let pool = Arc::new(setup_test_db().await);
        let app = spawn_test_app(pool.clone()).await;

        let (account_id, asset_id) = insert_user_and_asset(&pool).await;

        // Create
        let payload = json!({
            "account_id": account_id,
            "asset_id": asset_id,
            "amount": "123.45",
            "interval": "Monthly",
            "transaction_type": "Income"
        });
        let req = Request::post("/recurring_transactions")
            .header("Content-Type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);
        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let created: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let transaction_id = created["id"].as_str().unwrap();

        // Get by ID
        let req = Request::get(format!("/recurring_transactions/{}", transaction_id))
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Patch (update)
        let update_payload = json!({
            "amount": "456.78",
            "interval": "Weekly",
            "next_execution": Utc::now(),
            "is_active": false
        });
        let req = Request::patch(format!("/recurring_transactions/{}", transaction_id))
            .header("Content-Type", "application/json")
            .body(Body::from(update_payload.to_string()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Get All
        let req = Request::get("/recurring_transactions")
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Delete
        let req = Request::delete(format!("/recurring_transactions/{}", transaction_id))
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
