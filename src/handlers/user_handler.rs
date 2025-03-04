use crate::models::User;
use crate::repository::{create_user, delete_user, get_user_by_id, get_users, update_user_info};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_all_users(State(pool): State<Arc<PgPool>>) -> Json<Vec<User>> {
    let users = get_users(&pool).await.unwrap();
    Json(users)
}

pub async fn get_user(State(pool): State<Arc<PgPool>>, Path(user_id): Path<Uuid>) -> Json<User> {
    let user = get_user_by_id(&pool, user_id).await.unwrap();
    Json(user)
}

#[derive(Deserialize)]
pub struct CreateuserRequest {
    username: String,
    email: String,
}

pub async fn add_user(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateuserRequest>,
) -> (StatusCode, Json<User>) {
    match create_user(&pool, &payload.username, &payload.email).await {
        Ok(user) => (StatusCode::CREATED, Json(user)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(User {
                id: Uuid::nil(),
                username: "".to_string(),
                email: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                is_staff: false,
                is_active: false,
                date_joined: Utc::now(),
            }),
        ),
    }
}

#[derive(Deserialize)]
pub struct UpdateuserRequest {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

pub async fn update_user(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateuserRequest>,
) -> (StatusCode, Json<User>) {
    match update_user_info(
        &pool,
        user_id,
        payload.username.as_deref(),
        payload.first_name.as_deref(),
        payload.last_name.as_deref(),
        payload.email.as_deref(),
    )
    .await
    {
        Ok(user) => (StatusCode::OK, Json(user)),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(User {
                id: Uuid::nil(),
                username: "".to_string(),
                first_name: "".to_string(),
                last_name: "".to_string(),
                email: "".to_string(),
                is_staff: false,
                is_active: false,
                date_joined: Utc::now(),
            }),
        ),
    }
}

pub async fn delete_user_handler(
    State(pool): State<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> StatusCode {
    match delete_user(&pool, user_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
