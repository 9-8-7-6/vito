use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::User;

const QUERY_SELECT_ALL: &str = "SELECT * FROM users";
const QUERY_SELECT_ONE: &str = "SELECT * FROM users WHERE id = $1";
const QUERY_SELECT_BY_USERNAME: &str = "SELECT * FROM users WHERE username = $1";
const QUERY_SELECT_BY_EMAIL: &str = "SELECT * FROM users WHERE email = $1";
const QUERY_INSERT: &str = "
    INSERT INTO users (
        id, username, first_name, last_name, email, date_joined, hashed_password, is_staff, is_active
    ) VALUES (
        $1, $2, '', '', $3, $4, $5, $6, $7
    ) 
    RETURNING *
";
const QUERY_DELETE: &str = "DELETE FROM users WHERE id = $1";

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await?;
    Ok(users)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_SELECT_ONE)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_SELECT_BY_USERNAME)
        .bind(username)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_SELECT_BY_EMAIL)
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

pub async fn create_user(
    pool: &PgPool,
    user_id: &Uuid,
    username: &str,
    email: &str,
    hashed_password: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_INSERT)
        .bind(user_id)
        .bind(username)
        .bind(email)
        .bind(Utc::now())
        .bind(hashed_password)
        .bind(false)
        .bind(true)
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
    hashed_password: Option<&str>,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_UPDATE)
        .bind(username)
        .bind(first_name)
        .bind(last_name)
        .bind(email)
        .bind(hashed_password)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query(QUERY_DELETE)
        .bind(user_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
