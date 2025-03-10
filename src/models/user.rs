use crate::repository::{get_user_by_id, get_user_by_username};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

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
            Err(_) => false,
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
            Err(_) => Ok(None),
        }
    }
}
