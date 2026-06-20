use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "role_users", rename_all = "lowercase")]
pub enum RoleUsers {
    AUDITEE,
    AUDITOR,
    ASESOR,
    ADMIN
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid, 
    pub username: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub institute_id: Option<i32>,
    pub role: RoleUsers,
    pub is_banned: bool,
    pub must_change_password: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserReq {
    pub username: String,
    pub email: String,
    pub name: String,
    pub institute_id: Option<i32>,
    pub role: RoleUsers,
}
impl UserReq {
    pub fn normalize(mut self) -> Self {
        self.username = self.username.to_lowercase().replace(' ', "-");
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub institute_id: Option<i32>,
    pub role: Option<RoleUsers>,
}
impl UserUpdate {
    pub fn normalize(mut self) -> Self {
        self.username = self.username.map(|u| u.to_lowercase().replace(' ', "-"));
        self
    }
}