use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::institute::{InstituteNested, StudyProgramNested};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "approval_status", rename_all = "lowercase")]
pub enum ApprovalStatus{
    APPROVED,
    REJECTED,
    PENDING,
}

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct Lecturer{
    pub id: Uuid,
    pub name: String,
    pub nip: String,
    pub email: String,
    pub status: ApprovalStatus,
    pub study_program_name: String,
    pub study_program_id: i32,
    pub institute_name: String,
    pub institute_id: i32
}

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct LecturerCreate{
    pub name: String,
    pub nip: String,
    pub email: String,
    pub study_program_id: i32,
}

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct LecturerUpdate{
    pub name: Option<String>,
    pub nip: Option<String>,
    pub email: Option<String>,
    pub study_program_id: Option<i32>,
}

#[derive(Debug,Serialize, Deserialize, FromRow)]
pub struct LecturerResponse{
    pub id: Uuid,
    pub name: String,
    pub nip: String,
    pub email: String,
    pub status: ApprovalStatus,
    pub study_program: StudyProgramNested,
    pub institute: InstituteNested,
}
impl From<Lecturer> for LecturerResponse {
    fn from(detail: Lecturer) -> Self {
        Self {
            id: detail.id,
            name: detail.name,
            nip: detail.nip,
            email: detail.email,
            status: detail.status,
            study_program: StudyProgramNested { 
                id: detail.study_program_id,
                name: detail.study_program_name
            },
            institute: InstituteNested {
                id: detail.institute_id,
                name: detail.institute_name,
            },
        }
    }
}

#[derive(Debug,Serialize, Deserialize)]
pub struct LecturerQuery{
    pub name: Option<String>,
    pub study_program: Option<String>,
    pub institute: Option<String>,
    pub limit: Option<u64>,
    pub page: Option<u64>,
}