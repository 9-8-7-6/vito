use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use axum_login::AuthnBackend;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tower_sessions::Session;
use uuid::Uuid;

use crate::models::{user, Backend, Credentials, User};

#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub async fn api_register(
    State(backend): State<Backend>,
    payload: Json<RegisterPayload>,
) -> Result<Json<Value>, StatusCode> {
    if backend
        .get_user_by_username(&payload.username)
        .await
        .unwrap_or(None)
        .is_some()
    {
        return Ok(Json(json!({
            "status": "fail",
            "message": "Username already exists"
        })));
    }

    if backend
        .get_user_by_email(&payload.email)
        .await
        .unwrap_or(None)
        .is_some()
    {
        return Ok(Json(json!({
            "status": "fail",
            "message": "Email already registered"
        })));
    }

    let hashed_password =
        User::hash_password(&payload.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = User {
        id: Uuid::new_v4(),
        username: payload.username.clone(),
        email: payload.email.clone(),
        hashed_password,
        first_name: "".to_string(),
        last_name: "".to_string(),
        is_staff: false,
        is_active: true,
        date_joined: chrono::Utc::now(),
    };

    backend
        .create_user_(&new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    backend
        .create_account_(&new_user)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "status": "success",
        "message": "User registered successfully"
    })))
}

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
                "message": "Invalid account,please register first."
            })));
        }
        Err(_) => {
            return Ok(Json(json!({
                "status": "fail",
                "message": "Something wrong happens, please check again."
            })));
        }
    };

    session
        .insert("user_id", user.id.to_string())
        .await
        .expect(&format!("Failed to insert user {} session", user.id));

    println!(
        "Insert session {:?} into redis",
        session.get::<String>("user_id").await
    );

    let session_id = session.id().map(|id| id.to_string()).unwrap_or_default();
    let session_cookie = Cookie::build(("id", session_id))
        .path("/")
        .http_only(false)
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

    let cookie = Cookie::build(("id", ""))
        .path("/")
        .http_only(false)
        .max_age(time::Duration::seconds(0))
        .build();

    session
        .delete()
        .await
        .expect("Failed to delete session in redis");

    cookies.remove(cookie);

    Json(json!({ "status": "success", "message": "Logged out successfully" }))
}

pub async fn api_delete_account(
    session: Session,
    cookies: Cookies,
    State(backend): State<Backend>,
) -> Result<Json<Value>, StatusCode> {
    let user_id: Option<Uuid> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user_id = user_id.unwrap();
    backend
        .delete_user(&user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    session.clear().await;
    let cookie = Cookie::build(("id", ""))
        .path("/")
        .http_only(false)
        .max_age(time::Duration::seconds(-1))
        .build();
    cookies.remove(cookie);

    Ok(Json(json!({
        "status": "success",
        "message": "Account deleted successfully"
    })))
}

pub async fn check_session(
    State(backend): State<Backend>,
    session: Session,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let (is_valid, user_id) = backend
        .is_session_valid(&session)
        .await
        .unwrap_or((false, "".to_string()));
    let user_id = Uuid::parse_str(&user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    if is_valid {
        let user = backend
            .get_user(&user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if user.is_some() {
            return Ok(Json(json!({
                "status": "success",
                "message": "Session is valid"
            })));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
