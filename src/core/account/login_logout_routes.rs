use axum::{
    routing::{options, post},
    Router,
};

use crate::core::account::login_logout_handler::*;
use crate::models::Backend;

/// Defines authentication and account management routes
pub fn login_routes(backend: Backend) -> Router {
    Router::new()
        // POST /logout     -> Logs the user out
        // OPTIONS /logout  -> CORS preflight request (optional but common in frontend-heavy apps)
        .route("/logout", post(api_logout))
        .route("/logout", options(api_logout))
        // POST /delete_account -> Deletes the current authenticated user's account
        .route("/delete_account", post(api_delete_account))
        // POST /login     -> Authenticate user credentials and set session
        .route("/login", post(api_login))
        // POST /register  -> Create a new user account
        .route("/register", post(api_register))
        // POST /auth/check -> Verify if current session is valid (used for frontend checks)
        .route("/auth/check", post(check_session))
        // Share authentication backend state (PgPool wrapped in Backend)
        .with_state(backend)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use serde_json::json;
    use sqlx::PgPool;
    use std::env;
    use tower::ServiceExt;
    use tower_cookies::CookieManagerLayer;
    use tower_sessions::{MemoryStore, SessionManagerLayer};
    use uuid::Uuid;

    async fn setup_test_backend() -> Backend {
        dotenvy::from_filename(".env.test").ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&db_url).await.unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        Backend::new(&db_url).await.unwrap()
    }

    fn build_test_app(backend: Backend) -> Router {
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store);
        let cookie_layer = CookieManagerLayer::new();

        login_routes(backend)
            .layer(session_layer)
            .layer(cookie_layer)
    }

    #[tokio::test]
    async fn test_register_login_check_logout() {
        let backend = setup_test_backend().await;
        let app = build_test_app(backend);

        // === 註冊新使用者 ===
        let test_user = format!("user_{}", &Uuid::new_v4().to_string()[..8]);
        let test_email = format!("{test_user}@example.com");
        let register_payload = json!({
            "username": test_user,
            "password": "testpass123",
            "email": test_email
        });

        let req = Request::post("/register")
            .header("Content-Type", "application/json")
            .body(Body::from(register_payload.to_string()))
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // === 登入取得 session ===
        let login_payload = json!({
            "username": test_user,
            "password": "testpass123"
        });

        let req = Request::post("/login")
            .header("Content-Type", "application/json")
            .body(Body::from(login_payload.to_string()))
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // 儲存 cookie
        let cookies = res
            .headers()
            .get("set-cookie")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // === 驗證 session 是否有效 ===
        let req = Request::post("/auth/check")
            .header("Cookie", cookies.clone())
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // === 登出並清除 session ===
        let req = Request::post("/logout")
            .header("Cookie", cookies.clone())
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // === 驗證登出後 session 已失效 ===
        let req = Request::post("/auth/check")
            .header("Cookie", cookies)
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
    }
}
