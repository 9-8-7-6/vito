use crate::models::Category;
use sqlx::PgPool;
use uuid::Uuid;

const QUERY_SELECT_ALL: &str = "SELECT * FROM categories";
const QUERY_SELECT_ONE: &str = "SELECT * FROM categories WHERE id = $1";
const QUERY_INSERT: &str = "INSERT INTO categories (id, name, category_type) 
        VALUES ($1, $2, $3) RETURNING *";
const QUERY_UPDATE: &str = "UPDATE categories 
        SET name = $1, category_type = $2 
        WHERE id = $3 RETURNING *";
const QUERY_DELETE: &str = "DELETE FROM categories WHERE id = $1";

pub async fn get_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    sqlx::query_as::<_, Category>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await
}

pub async fn get_category_by_id(pool: &PgPool, category_id: Uuid) -> Result<Category, sqlx::Error> {
    sqlx::query_as::<_, Category>(QUERY_SELECT_ONE)
        .bind(category_id)
        .fetch_one(pool)
        .await
}

pub async fn create_category(
    pool: &PgPool,
    name: String,
    category_type: String,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as::<_, Category>(QUERY_INSERT)
        .bind(name)
        .bind(category_type)
        .fetch_one(pool)
        .await
}

pub async fn update_category_info(
    pool: &PgPool,
    category_id: Uuid,
    name: String,
    category_type: String,
) -> Result<Category, sqlx::Error> {
    sqlx::query_as::<_, Category>(QUERY_UPDATE)
        .bind(name)
        .bind(category_type)
        .bind(category_id)
        .fetch_one(pool)
        .await
}

pub async fn delete_category(pool: &PgPool, category_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(QUERY_DELETE)
        .bind(category_id)
        .execute(pool)
        .await
        .map(|_| ())
}
