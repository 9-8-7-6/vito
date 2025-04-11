use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::User;
use crate::repository::{create_user, delete_user, get_user_by_id, get_users, update_user_info};

#[derive(Deserialize)]
pub struct CreateuserRequest {
    user_id: Uuid,
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct UpdateuserRequest {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
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
        Ok(None) => {
            eprintln!("User {} not found.", user_id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(err) => {
            eprintln!("Error fetching user {}: {:?}", user_id, err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_user_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateuserRequest>,
) -> (StatusCode, Json<User>) {
    let hashed_password = match User::hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(err) => {
            eprintln!("Password hashing failed: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(dummy_user()));
        }
    };

    match create_user(
        &pool,
        &payload.user_id,
        &payload.username,
        &payload.email,
        &hashed_password,
    )
    .await
    {
        Ok(user) => (StatusCode::CREATED, Json(user)),
        Err(err) => {
            eprintln!("Failed to create user: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dummy_user()))
        }
    }
}

pub async fn update_user_handler(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateuserRequest>,
) -> (StatusCode, Json<User>) {
    let hashed_password = if let Some(password) = &payload.password {
        match User::hash_password(password) {
            Ok(hash) => Some(hash),
            Err(err) => {
                eprintln!("Password hashing failed: {:?}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(dummy_user()));
            }
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
        Err(err) => {
            eprintln!("Failed to update user {}: {:?}", user_id, err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(dummy_user()))
        }
    }
}

pub async fn delete_user_handler(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> StatusCode {
    match delete_user(&pool, user_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("User {} not found for deletion.", user_id);
            StatusCode::NOT_FOUND
        }
        Err(err) => {
            eprintln!("Failed to delete user {}: {:?}", user_id, err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

fn dummy_user() -> User {
    User::default()
}
