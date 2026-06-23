use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use uuid::Uuid;

use crate::{domain::repository::{ RecognitionLecturerCatTrait, RecognitionLecturerTrait, UserRepoTrait}, models::{lecturer::ApprovalStatus, recognition::{ManyRecognitionLecturer, RecognitionCategory, RecognitionCategoryCreate, RecognitionCategoryUpdate, RecognitionLecturerCreate, RecognitionLecturerQuery, RecognitionLecturerResponse, RecognitionLecturerUpdate}, user::{RoleUsers, User}}, repository::{institute::InstituteRepository, lecturer::LecturerRepository, recognition::RecognitionLecturerRepository, recognition_category::RecognitionCategoryRepository, study_program::StudyProgramRepository, user::UserRepository}, service::lecturer::LecturerService, state::{AppConfig, AppState}, utils::response::AppError};

type AppLecturerService = LecturerService<UserRepository, LecturerRepository, InstituteRepository, StudyProgramRepository>;

#[allow(dead_code)]
pub struct LecturerRecognitionService<U: UserRepoTrait, R:RecognitionLecturerTrait> {
    user_repo: U,
    recognition_repo: R,
    lecturer_service: AppLecturerService,
    config: AppConfig,
}
#[allow(dead_code)]
pub struct RecognitionCatService<U: UserRepoTrait, RC: RecognitionLecturerCatTrait> {
    user_repo: U,
    recognition_cat_repo: RC,
    config: AppConfig,
}

impl <U: UserRepoTrait,R:RecognitionLecturerTrait, > LecturerRecognitionService<U, R> {
    pub fn new(user_repo: U, recognition_repo:R, config: AppConfig, lecturer_service: AppLecturerService)-> Self {
        Self { user_repo, recognition_repo, lecturer_service, config }
    }

    pub async fn get_recongnition_detail(&self, recognition_id: Uuid) -> Result<RecognitionLecturerResponse, AppError> {
        let recognition = self.recognition_repo.find_by_id(recognition_id).await?;
        let lecturer = self.lecturer_service.get_lecturer_detail(recognition.lecturer_id).await?;
        let response: RecognitionLecturerResponse = recognition.into_model(lecturer.into());

        Ok(response)
    }

    pub async fn search_recongnition(&self,link_id: Uuid, query: RecognitionLecturerQuery) -> Result<(Vec<ManyRecognitionLecturer>, i64), AppError> {
        let recognition = self.recognition_repo.search(link_id, query)
            .await.map_err(|_|AppError::BadRequest("Terjadi kesalahan pada keyword pencarian!".to_string()))?;

        Ok(recognition)
    }

    pub async fn create_recognition(&self, data: RecognitionLecturerCreate) -> Result<RecognitionLecturerResponse, AppError> {
        let lecturer = self.lecturer_service.get_lecturer_detail(data.lecturer_id.clone()).await?;
        if lecturer.status != ApprovalStatus::APPROVED {
            return Err(AppError::Forbidden(format!("Dosen dengan NIP {} belum diperbolehkan mengisi forms!", lecturer.nip)));
        }
        let recognition = match self.recognition_repo.create(data).await {
            Ok(recognition) => recognition,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "42703" {
                                return Err(AppError::BadRequest(format!("Relasi tidak ditemukan! {}",db_err)));
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
        let lecturer = self.lecturer_service.get_lecturer_detail(recognition.lecturer_id).await?;
        let response: RecognitionLecturerResponse = recognition.into_model(lecturer.into());
        Ok(response)
    }
    
    pub async fn update_recognition(&self, _: User, recognition_id: Uuid, data: RecognitionLecturerUpdate) -> Result<RecognitionLecturerResponse, AppError> {
        let recognition = self.recognition_repo.update(recognition_id, data).await?;
        let lecturer = self.lecturer_service.get_lecturer_detail(recognition.lecturer_id).await?;
        let response: RecognitionLecturerResponse = recognition.into_model(lecturer.into());
        Ok(response)
    }

    pub async fn delete_recongnition(&self, _: User, recognition_id: Uuid) -> Result<RecognitionLecturerResponse, AppError> {
        let recognition = self.recognition_repo.delete(recognition_id).await?;
        let lecturer = self.lecturer_service.get_lecturer_detail(recognition.lecturer_id).await?;
        let response: RecognitionLecturerResponse = recognition.into_model(lecturer.into());

        Ok(response)
    }

    pub async fn approve_recongnition(&self, user: User, recognition_id: Uuid) -> Result<RecognitionLecturerResponse, AppError> {
        if user.role == RoleUsers::ASESOR {
            return Err(AppError::Forbidden("Assesor tidak diperbolehkan untuk melakukan aksi ini.".to_string()))
        }
        let recognition = self.recognition_repo.approve(recognition_id).await?;
        let lecturer = self.lecturer_service.get_lecturer_detail(recognition.lecturer_id).await?;
        let response: RecognitionLecturerResponse = recognition.into_model(lecturer.into());

        Ok(response)
    }

    pub async fn reject_recongnition(&self, user: User, recognition_id: Uuid) -> Result<RecognitionLecturerResponse, AppError> {
        if user.role == RoleUsers::ASESOR {
            return Err(AppError::Forbidden("Assesor tidak diperbolehkan untuk melakukan aksi ini.".to_string()))
        }
        let recognition = self.recognition_repo.reject(recognition_id).await?;
        let lecturer = self.lecturer_service.get_lecturer_detail(recognition.lecturer_id).await?;
        let response: RecognitionLecturerResponse = recognition.into_model(lecturer.into());

        Ok(response)
    }
}

impl <U: UserRepoTrait,RC:RecognitionLecturerCatTrait, > RecognitionCatService<U, RC> {
    pub fn new(user_repo: U, recognition_cat_repo:RC, config: AppConfig)-> Self {
        Self { user_repo, recognition_cat_repo, config }
    }

    pub async fn find_category_id(&self, category_id: i32) -> Result<RecognitionCategory, AppError> {
        let category = self.recognition_cat_repo.find_by_id(category_id).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Link dengan ID '{}' tidak ditemukan!", category_id)),
            _ => AppError::DatabaseError(e),
        })?;

        Ok(category)
    }

    pub async fn find_category_name(&self, category_name: &str) -> Result<Vec<RecognitionCategory>, AppError> {
        let category = self.recognition_cat_repo.find_name(category_name).await?;

        Ok(category)
    }
    
    pub async fn find_all_category(&self) -> Result<Vec<RecognitionCategory>, AppError> {
        let category = self.recognition_cat_repo.find_all().await?;

        Ok(category)
    }

    pub async fn create_category(&self, data: RecognitionCategoryCreate) -> Result<RecognitionCategory, AppError> {
        let category_name = data.name.clone();
        let category = match self.recognition_cat_repo.create(data).await {
            Ok(category) => category,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                return Err(AppError::BadRequest(format!("Kategori bernama '{}' sudah terdaftar!", category_name)));
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

        Ok(category)
    }

    pub async fn update_category(&self,category_id: i32, data: RecognitionCategoryUpdate) -> Result<RecognitionCategory, AppError> {
        let category_name = data.name.clone();
        let category = match self.recognition_cat_repo.update(category_id, data).await {
            Ok(category) => category,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                return Err(AppError::BadRequest(format!("Kategori bernama '{}' sudah terdaftar!", category_name.unwrap_or("NULL".to_string()))));
                            }
                        }
                        return Err(AppError::NotFound(format!("Kategori dengan id '{}' tidak ditemukan!", category_id)));
                    }
                    other_error => {
                        return Err(AppError::DatabaseError(other_error));
                    }
                }
            }
        };

        Ok(category)
    }

    pub async fn delete_category(&self, category_id: i32) -> Result<RecognitionCategory, AppError> {
        let category = self.recognition_cat_repo.delete(category_id).await?;

        Ok(category)
    }
}

impl<S> FromRequestParts<S> for LecturerRecognitionService<UserRepository, RecognitionLecturerRepository>
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
        let recognition_repo = RecognitionLecturerRepository::new(state.database.clone());
        let lecturer_service = AppLecturerService::new(user_repo.clone(), lecturer_repo, institute_repo, study_prg_repo, (*state.app_config).clone());
        
        Ok(LecturerRecognitionService { user_repo, recognition_repo, lecturer_service, config:(*state.app_config).clone() })
    }
}

impl<S> FromRequestParts<S> for RecognitionCatService<UserRepository, RecognitionCategoryRepository>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let user_repo = UserRepository::new(state.database.clone());
        let recognition_cat_repo = RecognitionCategoryRepository::new(state.database.clone());
        
        Ok(RecognitionCatService { user_repo, recognition_cat_repo, config:(*state.app_config).clone() })
    }
}