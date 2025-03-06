use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub category_type: String,
}

impl IntoResponse for Category {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct CategoryList(pub Vec<Category>);

impl IntoResponse for CategoryList {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
