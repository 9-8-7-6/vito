use crate::models::{Backend, Credentials, User};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use axum_login::AuthSession;
use axum_login::AuthnBackend;
use log::info;
use serde::Deserialize;
use serde_json::{json, Value};
use time::Duration;
use tower_cookies::{cookie::SameSite, Cookie, Cookies};
use tower_sessions::Session;
use uuid::Uuid;

/// Payload structure for user registration
#[derive(Debug, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub password: String,
    pub email: String,
}

/// Payload structure for user login
#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

/// Register a new user account
/// - Checks for duplicate username or email
/// - Hashes the password
/// - Creates user and associated account in the database
pub async fn api_register(
    State(backend): State<Backend>,
    payload: Json<RegisterPayload>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    // Check for duplicate username
    if let Ok(Some(_)) = backend.get_user_by_username(&payload.username).await {
        return Ok((
            StatusCode::CONFLICT,
            Json(json!({
                "status": "fail",
                "message": "Username already exists",
                "code": 409
            })),
        ));
    }

    // Check for duplicate email
    if let Ok(Some(_)) = backend.get_user_by_email(&payload.email).await {
        return Ok((
            StatusCode::CONFLICT,
            Json(json!({
                "status": "fail",
                "message": "Email already registered",
                "code": 409
            })),
        ));
    }

    // Hash the password
    let hashed_password = User::hash_password(&payload.password).map_err(|err| {
        eprintln!("Password hashing failed: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Create new User struct
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
        country: None,
        timezone: None,
    };

    // Save user to database
    backend.create_user_(&new_user).await.map_err(|err| {
        eprintln!("Failed to create user: {:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Create default account for the new user
    backend.create_account_(&new_user).await.map_err(|err| {
        eprintln!(
            "Failed to create account for user {}: {:?}",
            new_user.id, err
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "success",
            "message": "User registered successfully"
        })),
    ))
}

/// Log in a user using username/password and start a session
pub async fn api_login(
    mut auth_session: AuthSession<Backend>,
    State(backend): State<Backend>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>, StatusCode> {
    let credentials = Credentials {
        username: payload.username.clone(),
        password: payload.password.clone(),
    };

    let user: User = match backend.authenticate(credentials).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Ok(Json(json!({
                "status": "fail",
                "message": "Invalid account or password",
                "code": 401
            })));
        }
        Err(_) => {
            return Ok(Json(json!({
                "status": "fail",
                "message": "Internal error",
                "code": 500
            })));
        }
    };

    if let Err(e) = auth_session.login(&user).await {
        eprintln!("Session login failed: {:?}", e);
        return Ok(Json(json!({
            "status": "fail",
            "message": "Failed to start session",
            "code": 500
        })));
    }

    Ok(Json(json!({
        "status": "success",
        "message": "Login successful",
        "uuid": user.id,
    })))
}

/// Logout current user: clear session and cookie
pub async fn api_logout(
    mut auth_session: AuthSession<Backend>,
    cookies: Cookies,
) -> Json<serde_json::Value> {
    if let Err(err) = auth_session.logout().await {
        eprintln!("Failed to logout: {:?}", err);
    }

    let expired_cookie = Cookie::build(("id", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .max_age(Duration::ZERO)
        .build();

    cookies.add(expired_cookie);

    Json(json!({
        "status": "success",
        "message": "Logged out and cookie cleared"
    }))
}

/// Delete the currently authenticated user’s account
pub async fn api_delete_account(
    mut auth_session: axum_login::AuthSession<Backend>,
    cookies: Cookies,
    State(backend): State<Backend>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let Some(user) = auth_session.user.clone() else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    backend
        .delete_user(&user.id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(e) = auth_session.logout().await {
        eprintln!("Logout error: {:?}", e);
    }

    let expired_cookie = Cookie::build(("id", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .max_age(Duration::ZERO)
        .build();
    cookies.add(expired_cookie);

    Ok(Json(json!({
        "status": "success",
        "message": "Account deleted and session cleared"
    })))
}

/// Check if current session is valid and bound to a user
pub async fn check_session(
    auth_session: AuthSession<Backend>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(user) = auth_session.user {
        return Ok(Json(json!({
            "status": "success",
            "message": "Session is valid",
            "user_id": user.id,
            "username": user.username,
        })));
    }

    Err(StatusCode::UNAUTHORIZED)
}
