// Required imports for timestamps, database operations, and UUIDs
use chrono::Utc;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::User;

// SQL query constants
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

/// Fetch all users from the database
pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>(QUERY_SELECT_ALL)
        .fetch_all(pool)
        .await?;
    Ok(users)
}

/// Fetch a single user by their UUID
pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_SELECT_ONE)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Fetch a user by their username (used for login or lookup)
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

/// Fetch a user by their email (used for registration or lookup)
pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_SELECT_BY_EMAIL)
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}

/// Create a new user account in the database
pub async fn create_user(
    pool: &PgPool,
    user_id: &Uuid,
    username: &str,
    email: &str,
    hashed_password: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(QUERY_INSERT)
        .bind(user_id) // User ID (UUID)
        .bind(username) // Username
        .bind(email) // Email
        .bind(Utc::now()) // Date joined
        .bind(hashed_password) // Hashed password
        .bind(false) // is_staff: default false
        .bind(true) // is_active: default true
        .fetch_one(pool)
        .await?;

    Ok(user)
}

/// Update user profile information selectively (only provided fields)
pub async fn update_user_info(
    pool: &PgPool,
    user_id: Uuid,
    username: Option<&str>,
    first_name: Option<&str>,
    last_name: Option<&str>,
    email: Option<&str>,
    country: Option<&str>,
    timezone: Option<&str>,
    hashed_password: Option<&str>,
) -> Result<User, sqlx::Error> {
    // Return error if no fields are given for update
    if username.is_none()
        && first_name.is_none()
        && last_name.is_none()
        && email.is_none()
        && country.is_none()
        && timezone.is_none()
        && hashed_password.is_none()
    {
        return Err(sqlx::Error::RowNotFound);
    }

    // Build the update query dynamically
    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE users SET ");

    if let Some(username) = username {
        builder.push("username = ").push_bind(username).push(", ");
    }

    if let Some(first_name) = first_name {
        builder
            .push("first_name = ")
            .push_bind(first_name)
            .push(", ");
    }

    if let Some(last_name) = last_name {
        builder.push("last_name = ").push_bind(last_name).push(", ");
    }

    if let Some(email) = email {
        builder.push("email = ").push_bind(email).push(", ");
    }

    if let Some(country) = country {
        builder.push("country = ").push_bind(country).push(", ");
    }

    if let Some(timezone) = timezone {
        builder.push("timezone = ").push_bind(timezone).push(", ");
    }

    if let Some(hashed_password) = hashed_password {
        builder
            .push("hashed_password = ")
            .push_bind(hashed_password)
            .push(", ");
    }

    // Always update the `updated_at` field
    builder.push("updated_at = ").push_bind(Utc::now());

    builder.push(" WHERE id = ").push_bind(user_id);
    builder.push(" RETURNING *");

    // Execute the dynamic query and return the updated user
    let query = builder.build_query_as::<User>();
    let user = query.fetch_one(pool).await?;

    Ok(user)
}

/// Delete a user by ID
pub async fn delete_user(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    let result = sqlx::query(QUERY_DELETE)
        .bind(user_id)
        .execute(pool)
        .await?;

    // If no rows were affected, the user didn't exist
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}
