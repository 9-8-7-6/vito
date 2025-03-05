use crate::models::Category;
use crate::repository::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_all_categories(
    State(pool): State<Arc<PgPool>>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    match get_categories(&pool).await {
        Ok(categories) => Ok(Json(categories)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_category(
    State(pool): State<Arc<PgPool>>,
    Path(category_id): Path<Uuid>,
) -> Result<Json<Category>, StatusCode> {
    match get_category_by_id(&pool, category_id).await {
        Ok(category) => Ok(Json(category)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub category_type: String,
}

pub async fn add_category(
    State(pool): State<Arc<PgPool>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<(StatusCode, Json<Category>), StatusCode> {
    match create_category(&pool, payload.name, payload.category_type).await {
        Ok(category) => Ok((StatusCode::CREATED, Json(category))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub category_type: String,
}

pub async fn update_category(
    State(pool): State<Arc<PgPool>>,
    Path(category_id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<(StatusCode, Json<Category>), StatusCode> {
    match update_category_info(&pool, category_id, payload.name, payload.category_type).await {
        Ok(category) => Ok((StatusCode::OK, Json(category))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_category_handler(
    State(pool): State<Arc<PgPool>>,
    Path(category_id): Path<Uuid>,
) -> StatusCode {
    match delete_category(&pool, category_id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
