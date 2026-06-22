use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::lecturer::{ApprovalStatus, LecturerResponse};

// Table lecturer_recognition_categories
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionCategory {
    pub id: i32,
    pub name: String,
    pub description: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionCategoryCreate {
    pub name: String,
    pub description: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionCategoryUpdate {
    pub name: Option<String>,
    pub description: Option<String>
}

// Table lecturer_recognitions
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionLecturer {
    pub id: Uuid,
    pub description: String,
    pub proof_links: Value,
    pub orbitaind_at: DateTime<Utc>,
    pub status: ApprovalStatus,
    pub lecturer_id: Uuid,
    pub category_name: String,
    pub category_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionLecturerCreate {
    pub description: String,
    pub proof_links: Value,
    pub orbitaind_at: DateTime<Utc>,
    pub lecturer_id: Uuid,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionLecturerUpdate {
    pub description: Option<String>,
    pub proof_links: Option<Value>,
    pub orbitaind_at: Option<DateTime<Utc>>,
    pub category_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionLecturerResponse {
    pub id: Uuid,
    pub description: String,
    pub proof_links: Value,
    pub orbitaind_at: DateTime<Utc>,
    pub status: ApprovalStatus,
    pub category: RecognitionCategory,
    pub lecturer: LecturerResponse,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct RecognitionLecturerQuery{
    pub name: Option<String>,
    pub lecturer_name: Option<String>,
    pub lecturer_nip: Option<String>,
    pub study_program: Option<String>,
    pub institute: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<u64>,
    pub page: Option<u64>,
}