use crate::models::Category;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_categories(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(pool)
        .await?;
    Ok(categories)
}

pub async fn get_category_by_id(pool: &PgPool, category_id: Uuid) -> Result<Category, sqlx::Error> {
    let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(category_id)
        .fetch_one(pool)
        .await?;

    Ok(category)
}

pub async fn create_category(
    pool: &PgPool,
    name: String,
    category_type: String,
) -> Result<Category, sqlx::Error> {
    let category = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (id, name, category_type) 
        VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(name)
    .bind(category_type)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn update_category_info(
    pool: &PgPool,
    category_id: Uuid,
    name: String,
    category_type: String,
) -> Result<Category, sqlx::Error> {
    let category = sqlx::query_as::<_, Category>(
        "UPDATE categories 
        SET name = $1, category_type = $2 
        WHERE id = $3 RETURNING *",
    )
    .bind(name)
    .bind(category_type)
    .bind(category_id)
    .fetch_one(pool)
    .await?;

    Ok(category)
}

pub async fn delete_category(pool: &PgPool, category_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(category_id)
        .execute(pool)
        .await?;

    Ok(())
}
