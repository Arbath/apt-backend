use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::user::{User, UserUpdate};

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait UserRepoTrait: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> Result<User, sqlx::Error>;
    async fn find_by_username(&self, name: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_name_or_email(&self, identifier: &str) -> Result<Option<User>, sqlx::Error>;
    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn create(&self, data: User) -> Result<User, sqlx::Error>;
    async fn update(&self, id: &Uuid, data: UserUpdate) -> Result<User, sqlx::Error>;
    async fn update_password(&self, id: &Uuid, password: &String, must_change: bool) -> Result<User, sqlx::Error>;
    async fn delete(&self, user_id: &Uuid) -> Result<User, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait TokenRepoTrait: Send + Sync {
    async fn save_token(&self, token: &str, user_id: Uuid, expires_at: DateTime<Utc>) -> Result<(), sqlx::Error>;
    async fn exists(&self, token: &str) -> Result<bool, sqlx::Error>;
    async fn revoke(&self, token: &str) -> Result<(), sqlx::Error>;
}