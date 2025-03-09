use crate::models::{Backend, Credentials, User};
use crate::repository::{create_user, delete_user, get_user_by_id, get_users, update_user_info};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Form, Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

type AuthSession = axum_login::AuthSession<Backend>;

async fn login(mut auth_session: AuthSession, Form(creds): Form<Credentials>) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        #[allow(non_snake_case)]
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    Redirect::to("/protected").into_response()
}

pub async fn get_all_users_handler(State(pool): State<Arc<PgPool>>) -> Json<Vec<User>> {
    let users = get_users(&pool).await.unwrap();
    Json(users)
}

pub async fn get_user_handler(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<User>, StatusCode> {
    match get_user_by_id(&pool, user_id).await {
        Ok(Some(user)) => Ok(Json(user)),
        #[allow(non_snake_case)]
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct CreateuserRequest {
    username: String,
    email: String,
    password: String,
}

pub async fn add_user_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateuserRequest>,
) -> (StatusCode, Json<User>) {
    let hashed_password = match User::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default())),
    };

    match create_user(&pool, &payload.username, &payload.email, &hashed_password).await {
        Ok(user) => (StatusCode::CREATED, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default())),
    }
}

#[derive(Deserialize)]
pub struct UpdateuserRequest {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn update_user_handler(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateuserRequest>,
) -> (StatusCode, Json<User>) {
    let hashed_password = if let Some(password) = &payload.password {
        match User::hash_password(password) {
            Ok(hash) => Some(hash),
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default())),
        }
    } else {
        None
    };

    match update_user_info(
        &pool,
        user_id,
        payload.username.as_deref(),
        payload.first_name.as_deref(),
        payload.last_name.as_deref(),
        payload.email.as_deref(),
        hashed_password.as_deref(),
    )
    .await
    {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(User::default())),
    }
}

pub async fn delete_user_handler(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> StatusCode {
    match delete_user(&pool, user_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(sqlx::Error::RowNotFound) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
