use axum::extract::{FromRef, FromRequestParts};
use chrono::Utc;
use http::request::Parts;
use uuid::Uuid;

use crate::{
    domain::repository::{LinkTrait, UserRepoTrait}, models::user::User, repository::{feature::LinkRepository, user::UserRepository}, state::{AppConfig, AppState}, utils::response::AppError
};
use crate::models::feature::*;

#[allow(dead_code)]
pub struct LinkService<U: UserRepoTrait, L: LinkTrait> {
    user_repo: U,
    link_repo: L,
    config: AppConfig,
}

impl <U: UserRepoTrait, L: LinkTrait> LinkService<U, L> {
    pub fn new(user_repo: U, link_repo: L, config: AppConfig) -> Self {
        Self { user_repo, link_repo, config }
    }

    pub async fn find_link_id(&self, link_id: Uuid) -> Result<Link, AppError> {
        self.link_repo.find_by_id(link_id).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Link dengan ID '{}' tidak ditemukan!", link_id)),
            _ => AppError::DatabaseError(e),
        })
    }
    
    pub async fn find_link_slug(&self, slug: String) -> Result<Link, AppError> {
        let link = self.link_repo.find_by_slug(slug.clone()).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound(format!("Link dengan slug '{}' tidak ditemukan!", slug)),
            _ => AppError::DatabaseError(e),
        })?;

        let now = Utc::now();
        if now < link.started_at || now > link.ended_at {
            return Err(AppError::Forbidden(
                "Masa pengisian untuk form ini belum dimulai atau sudah berakhir.".to_string()
            ));
        }

        if !link.is_active {
            return Err(AppError::Forbidden(
                "Form ini sedang dinonaktifkan oleh Admin.".to_string()
            ));
        }

        Ok(link)
    }

    pub async fn find_links_by_institute(&self, institute_id: i32) -> Result<Vec<Link>, AppError> {
        let links = self.link_repo.find_all_by_institute(institute_id).await?;
        Ok(links)
    }

    pub async fn create_link(&self, user: User, data: LinkCreate) -> Result<Link, AppError> {
        let user = self.user_repo.find_by_id(&user.id).await.map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("User tidak ditemukan".to_string()),
            _ => AppError::DatabaseError(e)
        })?;

        let institute_id = user.institute_id.ok_or_else(|| {
            AppError::Forbidden("User tidak memiliki Institut, tidak bisa membuat link!".to_string())
        })?;

        let link_name = data.name.clone();

        match self.link_repo.create(institute_id, data).await {
            Ok(link) => Ok(link),
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                Err(AppError::BadRequest(format!("Link bernama atau slug '{}' sudah terdaftar!", link_name)))
            }
            Err(other_error) => Err(AppError::DatabaseError(other_error)),
        }
    }

    pub async fn update_link(&self, link_id: Uuid, data: LinkUpdate) -> Result<Link, AppError> {
        let slug_name = data.slug.clone().unwrap_or_else(|| "yang Anda masukkan".to_string());

        match self.link_repo.update(link_id, data).await {
            Ok(link) => Ok(link),
            Err(sqlx::Error::RowNotFound) => {
                Err(AppError::NotFound(format!("Link dengan id '{}' tidak ditemukan!", link_id)))
            }
            Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                Err(AppError::BadRequest(format!("Slug {} sudah dipakai oleh link lain!", slug_name)))
            }
            Err(other_error) => Err(AppError::DatabaseError(other_error)),
        }
    }

    pub async fn delete_link(&self, link_id: Uuid) -> Result<Link, AppError> {
        match self.link_repo.delete(link_id).await {
            Ok(link) => Ok(link),
            Err(sqlx::Error::RowNotFound) => {
                Err(AppError::NotFound(format!("Link dengan ID '{}' tidak ditemukan!", link_id)))
            }
            Err(other_error) => Err(AppError::DatabaseError(other_error)),
        }
    }
}

impl<S> FromRequestParts<S> for LinkService<UserRepository, LinkRepository>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let user_repo = UserRepository::new(state.database.clone());
        let link_repo = LinkRepository::new(state.database.clone());
        
        Ok(LinkService { 
            user_repo, 
            link_repo, 
            config: (*state.app_config).clone() 
        })
    }
}