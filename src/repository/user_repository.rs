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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{migrate::MigrateDatabase, PgPool, Postgres};
    use std::env;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        dotenvy::from_filename(".env.test").ok();
        let test_database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

        if !Postgres::database_exists(&test_database_url)
            .await
            .unwrap_or(false)
        {
            Postgres::create_database(&test_database_url)
                .await
                .expect("Failed to create test database");
        }

        let pool = PgPool::connect(&test_database_url)
            .await
            .expect("Failed to connect to the test database");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    #[tokio::test]
    async fn integration_test_user_crud() {
        let pool = setup_test_db().await;
        let user_id = Uuid::new_v4();
        let username = format!("testuser_{}", &user_id.to_string()[..8]);
        let email = format!("test_{}@example.com", &user_id.to_string()[..8]);
        let hashed_password = "password123";

        let created = create_user(&pool, &user_id, &username, &email, hashed_password)
            .await
            .expect("create_user failed");
        assert_eq!(created.id, user_id);

        let fetched = get_user_by_id(&pool, user_id)
            .await
            .expect("get_user_by_id failed")
            .unwrap();
        assert_eq!(fetched.username, username);

        let updated = update_user_info(
            &pool,
            user_id,
            None,
            Some("Updated"),
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .expect("update_user_info failed");
        assert_eq!(updated.first_name, "Updated");

        delete_user(&pool, user_id)
            .await
            .expect("delete_user failed");

        let deleted = get_user_by_id(&pool, user_id)
            .await
            .expect("get_user_by_id after delete");
        assert!(deleted.is_none());
    }
}
