use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::{institute::{InstituteNested, StudyProgramNested}, lecturer::{ApprovalStatus, LecturerResponse}};

// Table lecturer_recognition_categories
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionCategory {
    pub id: i32,
    pub name: String,
    pub description: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecognitionCategoryNested {
    pub id: i32,
    pub name: String,
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
    pub obtained_at: DateTime<Utc>,
    pub status: ApprovalStatus,
    pub lecturer_id: Uuid,
    pub category_name: String,
    pub category_id: i32,
    pub created_at: DateTime<Utc>,
}

impl RecognitionLecturer {
    pub fn into_model(self, lecturer_detail: LecturerResponse) -> RecognitionLecturerResponse {
        RecognitionLecturerResponse {
            id: self.id,
            description: self.description,
            proof_links: self.proof_links,
            obtained_at: self.obtained_at,
            status: self.status,
            category: RecognitionCategoryNested {
                id: self.category_id,
                name: self.category_name,
            },
            lecturer: lecturer_detail,
            created_at: self.created_at,
        }
    }
}

// Khusus untuk return banyak data
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ManyRecognitionLecturer {
    pub id: Uuid,
    pub description: String,
    pub proof_links: Value,
    pub obtained_at: DateTime<Utc>,
    pub status: ApprovalStatus,
    pub lecturer_name: String,
    pub lecturer_nip: String,
    pub lecturer_email: String,
    pub lecturer_status: ApprovalStatus,
    pub lecturer_id: Uuid,
    pub category_name: String,
    pub category_id: i32,
    pub study_prg_name: String,
    pub study_prg_id: i32,
    pub institute_name: String,
    pub institute_id: i32,
    pub created_at: DateTime<Utc>,
}

impl From<ManyRecognitionLecturer> for RecognitionLecturerResponse {
    fn from(row: ManyRecognitionLecturer) -> Self {
        Self {
            id: row.id,
            description: row.description,
            proof_links: row.proof_links,
            obtained_at: row.obtained_at,
            status: row.status,
            created_at: row.created_at,
            
            category: RecognitionCategoryNested {
                id: row.category_id,
                name: row.category_name,
            },
            
            lecturer: LecturerResponse {
                id: row.lecturer_id,
                name: row.lecturer_name,
                nip: row.lecturer_nip,
                email: row.lecturer_email,
                status: row.lecturer_status,
                study_program: StudyProgramNested {
                    id: row.study_prg_id,
                    name: row.study_prg_name,
                },
                institute: InstituteNested {
                    id: row.institute_id,
                    name: row.institute_name,
                },
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionLecturerCreate {
    pub description: String,
    pub proof_links: Value,
    pub obtained_at: DateTime<Utc>,
    pub lecturer_id: Uuid,
    pub category_id: i32,
    pub link_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionLecturerUpdate {
    pub description: Option<String>,
    pub proof_links: Option<Value>,
    pub obtained_at: Option<DateTime<Utc>>,
    pub category_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecognitionLecturerResponse {
    pub id: Uuid,
    pub description: String,
    pub proof_links: Value,
    pub obtained_at: DateTime<Utc>,
    pub status: ApprovalStatus,
    pub category: RecognitionCategoryNested,
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
    pub link_id: Option<Uuid>,
    pub sort: Option<String>,
    pub limit: Option<u64>,
    pub page: Option<u64>,
}