use crate::error::{Error, Result};
use crate::repository::get_user_by_username;
use axum::extract::{Json, State};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;
use tower_cookies::{Cookie, Cookies};
use tower_sessions::Session;

pub async fn api_login(
    cookies: Cookies,
    session: Session,
    State(pool): State<Arc<PgPool>>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>> {
    let user = match get_user_by_username(&pool, &payload.username).await {
        Ok(Some(user)) => user,
        #[allow(non_snake_case)]
        Ok(None) => return Err(Error::LoginFail),
        Err(_) => return Err(Error::DatabaseError),
    };

    if user.verify_password(&payload.password) {
        session
            .insert("user_id", user.id.to_string())
            .await
            .map_err(|_| Error::SessionError)?;
        session
            .insert("username", user.username.clone())
            .await
            .map_err(|_| Error::SessionError)?;

        let session_cookie = Cookie::build((
            "session_id",
            session
                .id()
                .map(|id| id.to_string())
                .unwrap_or("".to_string()),
        ))
        .path("/")
        .http_only(true)
        .build();
        cookies.add(session_cookie);

        Ok(Json(json!({
            "status": "success",
            "message": "Login successful",
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email
            }
        })))
    } else {
        Err(Error::LoginFail)
    }
}

pub async fn api_logout(session: Session, cookies: Cookies) -> Json<Value> {
    session.clear().await;

    let cookie = Cookie::build(("session_id", ""))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(-1))
        .build();
    cookies.remove(cookie);

    Json(json!({ "status": "success", "message": "Logged out successfully" }))
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}
