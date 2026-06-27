use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use uuid::Uuid;

use crate::{domain::repository::{InstituteTrait, LecturerTrait, StudyProgramTrait, UserRepoTrait}, models::{lecturer::{ApprovalStatus, Lecturer, LecturerCreate, LecturerQuery, LecturerUpdate}, user::{RoleUsers, User}}, repository::{institute::InstituteRepository, lecturer::LecturerRepository, study_program::StudyProgramRepository, user::UserRepository}, state::{AppConfig, AppState}, utils::response::AppError};

#[allow(dead_code)]
pub struct LecturerService<U: UserRepoTrait, L:LecturerTrait, I: InstituteTrait, SP:StudyProgramTrait> {
    user_repo: U,
    lecturer_repo: L,
    institute_repo: I, 
    study_prg_repo: SP,
    config: AppConfig,
}

impl <U: UserRepoTrait, L:LecturerTrait, I: InstituteTrait, SP:StudyProgramTrait> LecturerService<U, L, I, SP>{
    pub fn new(user_repo: U, lecturer_repo: L, institute_repo: I, study_prg_repo: SP, config: AppConfig)-> Self {
        Self { user_repo, lecturer_repo, institute_repo, study_prg_repo, config }
    }

    pub async fn get_lecturer_detail(&self, lecturer_id: Uuid) -> Result<Lecturer, AppError> {
        let q = self.lecturer_repo.find_by_id(lecturer_id)
            .await.map_err(|_| AppError::NotFound(format!("Dosen dengan id '{}' tidak ditemukan!", lecturer_id)))?;

        Ok(q)
    }

    pub async fn get_lecturer_by_nip(&self, nip: String) -> Result<Lecturer, AppError> {
        let q = self.lecturer_repo.find_by_nip(nip.clone())
            .await.map_err(|_| AppError::NotFound(format!("Dosen dengan NIP '{}' tidak ditemukan!", nip)))?;

        Ok(q)
    }

    pub async fn search_lecturer(&self, query: LecturerQuery) -> Result<(Vec<Lecturer>, i64), AppError> {
        let q = self.lecturer_repo.search(query)
            .await.map_err(|_| AppError::NotFound(format!("Dosen tidak ditemukan!")))?;

        Ok(q)
    }

    pub async fn add_lecturer(&self, user: Option<User>, data: LecturerCreate) -> Result<Lecturer, AppError> {
        let lecturer_nip = data.nip.clone();
        let approval_status = match user {
            Some(u) if u.role != RoleUsers::ASESOR && u.role != RoleUsers::AUDITOR => {
                ApprovalStatus::APPROVED
            },
            _ => ApprovalStatus::PENDING,
        };
        let lecturer = match self.lecturer_repo.create(approval_status, data).await {
            Ok(lecturer) => lecturer,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                return Err(AppError::BadRequest(format!("Dosen dengan NIP '{}' sudah terdaftar!", lecturer_nip)));
                            }

                            if code == "23503" {
                                return Err(AppError::BadRequest(format!("Program Studi tidak ditemukan!")));
                            }
                        }
                        return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
                    }
                    other_error => {
                        return Err(AppError::DatabaseError(other_error));
                    }
                }
            }
        };

        Ok(lecturer)
    }

    pub async fn edit_lecturer(&self, lecturer_id: Uuid, data: LecturerUpdate) -> Result<Lecturer, AppError> {
        let lecturer_nip = data.nip.clone();
        let lecturer = match self.lecturer_repo.update(lecturer_id, data).await {
            Ok(lecturer) => lecturer,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                return Err(AppError::BadRequest(format!("Dosen dengan NIP '{}' sudah terdaftar!", lecturer_nip.unwrap_or("0".to_string()))));
                            }
                            
                            if code == "23503" {
                                return Err(AppError::BadRequest(format!("Program Studi tidak ditemukan!")));
                            }
                        }
                        return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
                    }
                    other_error => {
                        return Err(AppError::DatabaseError(other_error));
                    }
                }
            }
        };

        Ok(lecturer)
    }

    pub async fn remove_lecturer(&self, lecturer_id: Uuid) -> Result<Lecturer, AppError> {
        let q = self.lecturer_repo.delete(lecturer_id)
            .await.map_err(|_| AppError::NotFound(format!("Dosen dengan id '{}' tidak ditemukan!", lecturer_id)))?;

        Ok(q)
    }
    
    pub async fn approve_lecturer(&self, lecturer_id: Uuid) -> Result<Lecturer, AppError> {
        let q = self.lecturer_repo.approve(lecturer_id)
            .await.map_err(|_| AppError::NotFound(format!("Dosen dengan id '{}' tidak ditemukan!", lecturer_id)))?;

        Ok(q)
    }
    
    pub async fn reject_lecturer(&self, lecturer_id: Uuid) -> Result<Lecturer, AppError> {
        let q = self.lecturer_repo.reject(lecturer_id)
            .await.map_err(|_| AppError::NotFound(format!("Dosen dengan id '{}' tidak ditemukan!", lecturer_id)))?;

        Ok(q)
    }
}

impl<S> FromRequestParts<S> for LecturerService<UserRepository, LecturerRepository, InstituteRepository, StudyProgramRepository>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let user_repo = UserRepository::new(state.database.clone());
        let lecturer_repo = LecturerRepository::new(state.database.clone());
        let institute_repo = InstituteRepository::new(state.database.clone());
        let study_prg_repo = StudyProgramRepository::new(state.database.clone());
        
        Ok(LecturerService::new(user_repo, lecturer_repo, institute_repo, study_prg_repo, (*state.app_config).clone()))
    }
}