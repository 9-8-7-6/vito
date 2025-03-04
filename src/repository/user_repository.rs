use crate::models::User;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn create_user(pool: &PgPool, username: &str, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (id, username, first_name, last_name, email, date_joined)
        VALUES ($1, $2, '', '', $3, $4) RETURNING *",
    )
    .bind(Uuid::new_v4())
    .bind(username)
    .bind(email)
    .bind(Utc::now())
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn update_user_info(
    pool: &PgPool,
    user_id: Uuid,
    username: Option<&str>,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "UPDATE users 
        SET username = COALESCE($1, username),
            first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            email = COALESCE($4, email),
            is_active = COALESCE($5, is_active),
            updated_at = now()
        WHERE id = $6 RETURNING *",
    )
    .bind(username)
    .bind(first_name)
    .bind(last_name)
    .bind(email)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}
