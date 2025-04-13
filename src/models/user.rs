use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::sync::Arc;
use tower_sessions::session::Session;
use uuid::Uuid;

use crate::repository::{
    create_account, create_user, delete_user, get_user_by_email, get_user_by_id,
    get_user_by_username,
};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_staff: bool,
    pub is_active: bool,
    pub date_joined: DateTime<Utc>,
    pub hashed_password: String,
    pub country: Option<String>,
    pub timezone: Option<String>,
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.hashed_password.as_bytes()
    }
}

impl User {
    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| e.to_string())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        match PasswordHash::new(&self.hashed_password) {
            Ok(parsed_hash) => Argon2::default()
                .verify_password(password.as_bytes(), &parsed_hash)
                .is_ok(),
            Err(err) => {
                eprintln!("Invalid password hash format: {:?}", err);
                false
            }
        }
    }
}

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Backend {
    db: Arc<PgPool>,
}

impl Backend {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db = Arc::new(
            PgPoolOptions::new()
                .max_connections(10)
                .connect(db_url)
                .await?,
        );
        Ok(Self { db })
    }
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        get_user_by_email(&self.db, email).await
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        get_user_by_username(&self.db, username).await
    }

    pub async fn create_user_(&self, user: &User) -> Result<(), sqlx::Error> {
        create_user(
            &self.db,
            &user.id,
            &user.username,
            &user.email,
            &user.hashed_password,
        )
        .await
        .map(|_| ())
    }

    pub async fn create_account_(&self, user: &User) -> Result<(), sqlx::Error> {
        match create_account(&self.db, user.id.clone(), Decimal::new(0, 2)).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_user(&self, user_id: &Uuid) -> Result<(), sqlx::Error> {
        delete_user(&self.db, user_id.clone()).await
    }

    pub async fn is_session_valid(
        &self,
        session: &Session,
    ) -> Result<(bool, String), tower_sessions::session::Error> {
        let user_id: Option<String> = session.get("user_id").await?;

        if let Some(user_id) = user_id {
            if let Ok(uuid) = Uuid::parse_str(&user_id) {
                return Ok((true, user_id));
            }
        }

        Ok((false, "".to_string()))
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { username, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        match get_user_by_username(&self.db, &username).await {
            Ok(Some(user)) => {
                if user.verify_password(&password) {
                    return Ok(Some(user));
                }
            }
            Ok(Option::None) => {}
            Err(err) => {
                eprintln!("Database error: {:?}", err);
            }
        }
        Ok(None)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        match get_user_by_id(&self.db, user_id.clone()).await {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("get_user error: {:?}", err);
                Ok(None)
            }
        }
    }
}
