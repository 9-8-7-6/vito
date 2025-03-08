use crate::models::CategoryList;
use crate::repository::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_all_categories_handler(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    match get_categories(&pool).await {
        Ok(categories) => CategoryList(categories).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_category_handler(
    State(pool): State<Arc<PgPool>>,
    Path(category_id): Path<Uuid>,
) -> impl IntoResponse {
    match get_category_by_id(&pool, category_id).await {
        Ok(category) => category.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub category_type: String,
}

pub async fn add_category_handler(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    match create_category(&pool, payload.name, payload.category_type).await {
        Ok(category) => category.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub category_type: String,
}

pub async fn update_category_handler(
    State(pool): State<Arc<PgPool>>,
    Path(category_id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> impl IntoResponse {
    match update_category_info(&pool, category_id, payload.name, payload.category_type).await {
        Ok(category) => category.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn delete_category_handler(
    State(pool): State<Arc<PgPool>>,
    Path(category_id): Path<Uuid>,
) -> impl IntoResponse {
    match delete_category(&pool, category_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
