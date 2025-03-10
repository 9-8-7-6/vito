use crate::models::{Backend, Credentials};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use axum_login::AuthnBackend;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tower_sessions::Session;

pub async fn api_login(
    cookies: Cookies,
    session: Session,
    State(backend): State<Backend>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>, StatusCode> {
    let credentials = Credentials {
        username: payload.username.clone(),
        password: payload.password.clone(),
    };

    let user = match backend.authenticate(credentials).await {
        Ok(Some(user)) => user,
        Ok(Option::None) => {
            return Ok(Json(json!({
                "status": "fail",
                "message": "Invalid username or password"
            })));
        }
        Err(_) => {
            return Ok(Json(json!({
                "status": "fail",
                "message": "Invalid username or password"
            })));
        }
    };

    if session
        .insert("user_id", user.id.to_string())
        .await
        .is_err()
        || session
            .insert("username", user.username.clone())
            .await
            .is_err()
    {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let session_id = session.id().map(|id| id.to_string()).unwrap_or_default();
    let session_cookie = Cookie::build(("session_id", session_id))
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
