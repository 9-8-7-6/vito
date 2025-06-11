use crate::models::{Backend, Credentials, User};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use axum_login::AuthnBackend;
use log::info;
use serde::Deserialize;
use serde_json::{json, Value};
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
) -> Result<Json<Value>, StatusCode> {
    // Check for duplicate username
    if let Ok(Some(_)) = backend.get_user_by_username(&payload.username).await {
        return Ok(Json(json!({
            "status": "fail",
            "message": "Username already exists",
            "code": 409
        })));
    }

    // Check for duplicate email
    if let Ok(Some(_)) = backend.get_user_by_email(&payload.email).await {
        return Ok(Json(json!({
            "status": "fail",
            "message": "Email already registered",
            "code": 409
        })));
    }

    // Hash the password
    let hashed_password = match User::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Password hashing failed: {:?}", err);
            return Ok(Json(json!({
                "status": "fail",
                "message": "Internal error while hashing password",
                "code": 500
            })));
        }
    };

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
    if let Err(err) = backend.create_user_(&new_user).await {
        eprintln!("Failed to create user: {:?}", err);
        return Ok(Json(json!({
            "status": "fail",
            "message": "Failed to create user",
            "code": 500
        })));
    }

    // Create default account for the new user
    if let Err(err) = backend.create_account_(&new_user).await {
        eprintln!(
            "Failed to create account for user {}: {:?}",
            new_user.id, err
        );
        return Ok(Json(json!({
            "status": "fail",
            "message": "Failed to create account",
            "code": 500
        })));
    }

    Ok(Json(json!({
        "status": "success",
        "message": "User registered successfully"
    })))
}

/// Log in a user using username/password and start a session
pub async fn api_login(
    cookies: Cookies,
    session: Session,
    State(backend): State<Backend>,
    payload: Json<LoginPayload>,
) -> Result<Json<Value>, StatusCode> {
    // Build credential object
    let credentials = Credentials {
        username: payload.username.clone(),
        password: payload.password.clone(),
    };

    // Authenticate the user
    let user: User = match backend.authenticate(credentials).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            eprintln!(
                "Login failed: invalid credentials for '{}'",
                payload.username
            );
            return Ok(Json(json!({
                "status": "fail",
                "message": "Invalid account or password",
                "code": 401
            })));
        }
        Err(err) => {
            eprintln!("Authentication error: {:?}", err);
            return Ok(Json(json!({
                "status": "fail",
                "message": "Internal server error during login",
                "code": 500
            })));
        }
    };

    // Store user_id in session
    if let Err(err) = session.insert("user_id", user.id.to_string()).await {
        eprintln!("Failed to insert session for user {}: {:?}", user.id, err);
        return Ok(Json(json!({
            "status": "fail",
            "message": "Failed to set session",
            "code": 500
        })));
    }

    // Set session ID in cookie (non-HttpOnly for client-side access)
    let session_id = session.id().map(|id| id.to_string()).unwrap_or_default();
    let clear_old_cookie = Cookie::build(("id", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .max_age(time::Duration::ZERO)
        .build();
    cookies.add(clear_old_cookie);

    let session_cookie = Cookie::build(("id", session_id))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .max_age(time::Duration::days(7))
        .build();

    cookies.add(session_cookie);

    Ok(Json(json!({
        "status": "success",
        "message": "Login successful",
        "uuid": user.id,
    })))
}

/// Logout current user: clear session and cookie
pub async fn api_logout(session: Session, cookies: Cookies) -> Json<Value> {
    if let Some(cookie) = cookies.get("id") {
        info!("Found session id cookie: {}", cookie.value());
    } else {
        info!("No session id cookie present");
    }

    if let Err(e) = session.flush().await {
        info!("session.flush() error: {:?}", e);
    }
    info!("Finish flush session");

    let clear_host = Cookie::build(("id", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .max_age(time::Duration::seconds(0))
        .build();

    cookies.add(clear_host);
    info!("cookie 'id' cleared in response");

    Json(json!({ "status": "success", "message": "Logged out successfully" }))
}

/// Delete the currently authenticated user’s account
pub async fn api_delete_account(
    session: Session,
    cookies: Cookies,
    State(backend): State<Backend>,
) -> Result<Json<Value>, StatusCode> {
    // Get user ID from session
    let user_id: Option<Uuid> = session.get("user_id").await.unwrap_or(None);
    if user_id.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Delete user
    let user_id = user_id.unwrap();
    backend
        .delete_user(&user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Clean up session and cookie
    session.clear().await;
    session.delete().await.ok();

    let exp_host = Cookie::build(("id", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .build();
    cookies.add(exp_host);

    let exp_domain = Cookie::build(("id", ""))
        .path("/")
        .max_age(time::Duration::seconds(0))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::None)
        .build();
    cookies.remove(exp_domain);

    Ok(Json(json!({
        "status": "success",
        "message": "Account deleted successfully"
    })))
}

/// Check if current session is valid and bound to a user
pub async fn check_session(
    State(backend): State<Backend>,
    session: Session,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let (is_valid, user_id_str) = backend
        .is_session_valid(&session)
        .await
        .unwrap_or((false, "".to_string()));

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user_id = Uuid::parse_str(&user_id_str).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = backend
        .get_user(&user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if user.is_some() {
        return Ok(Json(json!({
            "status": "success",
            "message": "Session is valid",
        })));
    }

    Err(StatusCode::UNAUTHORIZED)
}
