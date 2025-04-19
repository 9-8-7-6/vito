use axum::{
    routing::{get, patch, post},
    Router,
};
use axum_login::login_required;
use sqlx::PgPool;
use std::sync::Arc;

use crate::handlers::transaction_handler::*;
use crate::models::Backend;

/// Defines routes for managing financial transactions
pub fn transaction_routes(state: Arc<PgPool>) -> Router {
    Router::new()
        // POST /transactions
        // -> Create a new transaction and apply balance changes
        .route("/transactions", post(add_transaction_handler))
        // GET    /transactions/{id} -> Retrieve a transaction by ID
        // PATCH  /transactions/{id} -> Update a transaction and re-calculate balances
        // DELETE /transactions/{id} -> Delete a transaction and roll back balances
        .route(
            "/transactions/{id}",
            get(get_transaction_by_transaction_id_handler)
                .patch(update_transaction_handler)
                .delete(delete_transaction_handler),
        )
        // GET /transactions/account/{id}
        // -> Retrieve all enriched transactions for a specific account
        .route(
            "/transactions/account/{id}",
            get(get_transaction_by_account_id_handler),
        )
        // Optional: Enable this line to restrict transaction routes to authenticated users
        // .route_layer(login_required!(Backend, login_url = "/login"))
        // Share the database connection pool with all route handlers
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::{body::Body, http::Request};
    use serde_json::json;
    use sqlx::PgPool;
    use std::env;
    use std::sync::Arc;
    use tower::ServiceExt;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&url).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        pool
    }

    async fn insert_user_and_account(pool: &PgPool) -> Uuid {
        let user_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO users (id, username, email, hashed_password) VALUES ($1, $2, $3, $4)",
        )
        .bind(user_id)
        .bind(format!("user_{}", &user_id.to_string()[..8]))
        .bind(format!("{}@test.com", &user_id.to_string()[..8]))
        .bind("hashed")
        .execute(pool)
        .await
        .unwrap();

        sqlx::query("INSERT INTO accounts (account_id, balance, created_at, updated_at) VALUES ($1, $2, now(), now())")
            .bind(user_id)
            .bind(1000.00)
            .execute(pool)
            .await
            .unwrap();

        user_id
    }

    #[tokio::test]
    async fn test_transaction_crud_flow() {
        let pool = Arc::new(setup_test_db().await);
        let app = transaction_routes(pool.clone());
        let user_id = insert_user_and_account(&pool).await;

        // Create asset
        let asset_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO assets (id, account_id, asset_type, balance, created_at, updated_at)
                     VALUES ($1, $2, 'cash', 1000.00, now(), now())",
        )
        .bind(asset_id)
        .bind(user_id)
        .execute(&*pool)
        .await
        .unwrap();

        // Create transaction
        let tx_payload = json!({
            "from_asset_id": asset_id,
            "to_asset_id": null,
            "transaction_type": "Expense",
            "amount": "100.00",
            "fee": "5.00",
            "from_account_id": user_id,
            "to_account_id": null,
            "transaction_time": null,
            "notes": "Test expense"
        });

        let req = Request::post("/transactions")
            .header("Content-Type", "application/json")
            .body(Body::from(tx_payload.to_string()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let body = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        let created_tx: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let tx_id = created_tx["id"].as_str().unwrap();

        // Get transaction
        let req = Request::get(format!("/transactions/{}", tx_id))
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Update transaction
        let update_payload = json!({"amount": "120.00"});
        let req = Request::patch(format!("/transactions/{}", tx_id))
            .header("Content-Type", "application/json")
            .body(Body::from(update_payload.to_string()))
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Delete transaction
        let req = Request::delete(format!("/transactions/{}", tx_id))
            .body(Body::empty())
            .unwrap();
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
