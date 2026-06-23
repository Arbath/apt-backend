use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

// Table links
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Link{
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub is_active: bool,
    pub institute_id: i32,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LinkCreate{
    pub name: String,
    pub slug: String,
    pub description: String,
    pub is_active: bool,
    pub started_at: DateTime<Utc>,
    pub ended_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LinkUpdate{
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
}

// Table logs
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LogActivity{
    pub id: Uuid,
    pub activity: String,
    pub user_id: Uuid, // relasi dengan users
    pub user_username: String,
    pub created_at: DateTime<Utc>,
}