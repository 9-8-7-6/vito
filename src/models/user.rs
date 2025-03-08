use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_staff: bool,
    pub is_active: bool,
    pub date_joined: DateTime<Utc>,
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.id.as_bytes()
    }
}

#[derive(Clone, Default)]
struct Backend {
    users: HashMap<Uuid, User>,
}

#[derive(Clone)]
struct Credentials {
    user_id: Uuid,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    
    async fn authenticate(
        &self,
        Credentials { user_id }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(&user_id).cloned())
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        Ok(self.users.get(user_id).cloned())
    }
}
