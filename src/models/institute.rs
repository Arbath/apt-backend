use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Institute {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct InstituteCreate {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct InstituteUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InstituteQueryParams {
    pub name: Option<String>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudyProgram {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub institute_name: String,
    pub institute_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudyProgramCreate {
    pub name: String,
    pub description: Option<String>,
    pub institute_id: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StudyProgramUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub institute_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstituteNested {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudyProgramNested {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct StudyProgramResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub institute: InstituteNested,
}

impl From<StudyProgram> for StudyProgramResponse {
    fn from(detail: StudyProgram) -> Self {
        Self {
            id: detail.id,
            name: detail.name,
            description: detail.description,
            institute: InstituteNested {
                id: detail.institute_id,
                name: detail.institute_name,
            },
        }
    }
}