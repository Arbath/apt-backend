use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;

use crate::{domain::repository::{InstituteTrait, StudyProgramTrait, UserRepoTrait}, models::institute::{Institute, InstituteCreate, InstituteUpdate, StudyProgram, StudyProgramCreate, StudyProgramUpdate}, repository::{institute::InstituteRepository, study_program::StudyProgramRepository, user::UserRepository}, state::{AppConfig, AppState}, utils::response::AppError};

#[allow(dead_code)]
pub struct InstituteService<U: UserRepoTrait, I: InstituteTrait, SP: StudyProgramTrait> {
    user_repo: U,
    institute_repo: I,
    study_prg_repo: SP,
    config: AppConfig,
}
impl <U: UserRepoTrait, I: InstituteTrait, SP: StudyProgramTrait> InstituteService<U, I, SP> {
    pub fn new(user_repo:U, institute_repo: I, study_prg_repo: SP, config: AppConfig) -> Self {
        Self { user_repo, institute_repo, study_prg_repo, config }
    }

    pub async fn get_one_institute(&self, institute_id: i32) -> Result<(String, Institute), AppError> {
        let q = self.institute_repo.find_by_id(institute_id)
            .await.map_err(|_|AppError::NotFound(format!("Lembaga dengan id '{}' tidak ditemukan!", institute_id)))?;
        let msg= format!("Detail lembaga '{}'", q.name);

        Ok((msg, q))
    }
    
    pub async fn get_one_institute_name(&self, institute_name: &str, page: i64, limit: i64) -> Result<(String, (Vec<Institute>, u64)), AppError> {
        let q = self.institute_repo.find_by_name(institute_name, page, limit)
            .await.map_err(|_|AppError::NotFound(format!("Lembaga dengan keyword '{}' tidak ditemukan!", institute_name)))?;
        let msg= format!("List lembaga dengan keyword '{}'", institute_name);

        Ok((msg, q))
    }
    
    pub async fn get_all_institute(&self, page: i64, limit: i64) -> Result<(String, (Vec<Institute>, u64)), AppError> {
        let q = self.institute_repo.find_all(page, limit)
            .await.map_err(|_|AppError::NotFound(format!("Lembaga tidak ditemukan!")))?;
        let msg= format!("Detail semua lembaga");

        Ok((msg, q))
    }
    
    pub async fn get_all_institute_study_programs(&self, institute_id: i32) -> Result<(String, Vec<StudyProgram>), AppError> {
        let institute = self.institute_repo.find_by_id(institute_id).await?;
        let q = self.institute_repo.find_all_study_programs(institute_id)
            .await.map_err(|_|AppError::NotFound(format!("Lembaga tidak ditemukan!")))?;
        let msg= format!("Detail semua program studi {}", institute.name);

        Ok((msg, q))
    }

    pub async fn add_institute(&self, data: InstituteCreate) -> Result<(String, Institute), AppError> {
        let institute_name = data.name.clone();
        let q = match self.institute_repo.create(data).await {
            Ok(institute) => institute,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                return Err(AppError::BadRequest(format!("Lembaga bernama '{}' sudah terdaftar!", institute_name)));
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
        let msg= format!("Lembaga '{}' berhasil ditambahkan.", q.name);

        Ok((msg, q))
    }
    
    pub async fn edit_institute(&self, institute_id: i32, data: InstituteUpdate) -> Result<(String, Institute), AppError> {
        let institute_name = data.name.clone();
        let q = match self.institute_repo.update(institute_id, data).await {
            Ok(institute) => institute,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                let err_msg = match institute_name {
                                    Some(nama) => format!("Lembaga bernama '{}' sudah terdaftar!", nama),
                                    None => "Data Lembaga yang Anda masukkan sudah terdaftar!".to_string(),
                                };

                                return Err(AppError::BadRequest(err_msg));
                            }
                        }
                        return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
                    }
                    sqlx::Error::RowNotFound => {
                        return Err(AppError::NotFound(format!("Lembaga dengan id '{}' tidak ditemukan!", institute_id)));
                    }
                    other_error => {
                        return Err(AppError::DatabaseError(other_error));
                    }
                }
            }
        };
        let msg= format!("Lembaga '{}' berhasil diperbarui.", q.name);

        Ok((msg, q))
    }

    pub async fn delete_institute(&self, institute_id: i32) -> Result<(String, Institute), AppError> {
        let q = self.institute_repo.delete(institute_id)
            .await.map_err(|_|AppError::NotFound(format!("Lembaga dengan id '{}' tidak ditemukan!", institute_id)))?;
        let msg= format!("Lembaga dengan id '{}' berhasi dihapus.", q.name);

        Ok((msg, q))
    }

    pub async fn get_one_study_prg(&self, study_prg_id: i32) -> Result<(String, StudyProgram), AppError> {
        let q = self.study_prg_repo.find_by_id(study_prg_id)
            .await.map_err(|_|AppError::NotFound(format!("Program Studi dengan id '{}' tidak ditemukan!", study_prg_id)))?;
        let msg= format!("Detail Program Studi '{}'", q.name);

        Ok((msg, q))
    }
    
    pub async fn get_one_study_prg_name(&self, study_prg_name: &str, page: i64, limit: i64) -> Result<(String, (Vec<StudyProgram>, u64)), AppError> {
        let q = self.study_prg_repo.find_by_name(study_prg_name, page, limit)
            .await.map_err(|_|AppError::NotFound(format!("Program Studi dengan keyword '{}' tidak ditemukan!", study_prg_name)))?;
        let msg= format!("Detail Program Studi dengan keyword '{}'", study_prg_name);

        Ok((msg, q))
    }
    
    pub async fn get_all_study_prg(&self, page: i64, limit: i64) -> Result<(String, (Vec<StudyProgram>, u64)), AppError> {
        let q = self.study_prg_repo.find_all(page, limit)
            .await.map_err(|_|AppError::NotFound(format!("Program Studi tidak ditemukan!")))?;
        let msg= format!("Detail semua Program Studi");

        Ok((msg, q))
    }

    pub async fn add_study_prg(&self, data: StudyProgramCreate) -> Result<(String, StudyProgram), AppError> {
        let study_prg_name = data.name.clone();
        let q = match self.study_prg_repo.create(data).await {
            Ok(institute) => institute,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                return Err(AppError::BadRequest(format!("Program Studi bernama '{}' sudah terdaftar!", study_prg_name)));
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
        let msg= format!("Program Studi '{}' berhasil ditambahkan.", q.name);

        Ok((msg, q))
    }
    
    pub async fn edit_study_prg(&self, study_prg_id: i32, data: StudyProgramUpdate) -> Result<(String, StudyProgram), AppError> {
        let study_prg_name = data.name.clone();
        let q = match self.study_prg_repo.update(study_prg_id, data).await {
            Ok(institute) => institute,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                let err_msg = match study_prg_name {
                                    Some(nama) => format!("Program Studi bernama '{}' sudah terdaftar!", nama),
                                    None => "Data Program Studi yang Anda masukkan sudah terdaftar!".to_string(),
                                };

                                return Err(AppError::BadRequest(err_msg));
                            }
                        }
                        return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
                    }
                    sqlx::Error::RowNotFound => {
                        return Err(AppError::NotFound(format!("Program Studi dengan id '{}' tidak ditemukan!", study_prg_id)));
                    }
                    other_error => {
                        return Err(AppError::DatabaseError(other_error));
                    }
                }
            }
        };
        let msg= format!("Program Studi '{}' berhasil diperbarui.", q.name);

        Ok((msg, q))
    }

    pub async fn delete_study_prg(&self, study_prg_id: i32) -> Result<(String, StudyProgram), AppError> {
        let q = self.study_prg_repo.delete(study_prg_id)
            .await.map_err(|_|AppError::NotFound(format!("Program Studi dengan id '{}' tidak ditemukan!", study_prg_id)))?;
        let msg= format!("Program Studi dengan id '{}' berhasi dihapus.", q.name);

        Ok((msg, q))
    }
}

impl<S> FromRequestParts<S> for InstituteService<UserRepository, InstituteRepository, StudyProgramRepository>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let user_repo = UserRepository::new(state.database.clone());
        let institute_repo = InstituteRepository::new(state.database.clone());
        let study_prg_repo = StudyProgramRepository::new(state.database.clone());
        
        Ok(InstituteService::new(user_repo, institute_repo, study_prg_repo, (*state.app_config).clone()))
    }
}
