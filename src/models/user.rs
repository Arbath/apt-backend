use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum AccountType {
    USER,
    ORGANIZATION
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid, 
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub is_superuser: bool,
    pub created_at: DateTime<Utc>,
    pub account_type: AccountType,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub id: Uuid, 
    pub name: String,
    pub email: String,
    pub is_superuser: bool,
    pub created_at: DateTime<Utc>,
    pub account_type: AccountType,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            is_superuser: user.is_superuser,
            created_at: user.created_at,
            account_type: user.account_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UpdateProfileReq {
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserReq {
    pub name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_superuser: Option<bool>,
}