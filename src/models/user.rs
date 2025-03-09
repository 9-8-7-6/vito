use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        PasswordHash::new(&self.hashed_password)
            .ok()
            .and_then(|parsed_hash| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .ok()
            })
            .is_some()
    }
}

#[derive(Clone, Default)]
pub struct Backend {
    users: Arc<RwLock<HashMap<Uuid, User>>>,
}

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
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
        let users = self.users.read().unwrap();

        if let Some(user) = users.values().find(|u| u.username == username) {
            if user.verify_password(&password) {
                return Ok(Some(user.clone()));
            }
        }
        Ok(None)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let users = self.users.read().unwrap();
        Ok(users.get(user_id).cloned())
    }
}
