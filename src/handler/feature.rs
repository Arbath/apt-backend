use axum::http::Uri;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::models::feature::{LinkCreate, LinkUpdate};
use crate::repository::feature::LinkRepository;
use crate::repository::user::UserRepository;
use crate::service::feature::LinkService;
use crate::utils::request::{ValidatedJson, ValidatedPath};
use crate::utils::response::{AppError, WebResponse};

type AppLinkService = LinkService<UserRepository, LinkRepository>;

pub async fn get_link_by_id_hand(
    ValidatedPath(link_id): ValidatedPath<Uuid>,
    uri: Uri,
    service: AppLinkService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.find_link_id(link_id).await?;
    let message = format!("Detail link '{}'", response_data.name);
    
    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn get_link_by_slug_hand(
    ValidatedPath(slug): ValidatedPath<String>,
    uri: Uri,
    service: AppLinkService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.find_link_slug(slug).await?;
    let message = format!("Detail link '{}'", response_data.name);
    
    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn get_links_by_institute_hand(
    uri: Uri,
    AuthUser(user): AuthUser,
    service: AppLinkService,
) -> Result<impl IntoResponse, AppError> {  
    let institute_id = user.institute_id.ok_or_else(|| {
        AppError::Forbidden("Akses ditolak: User tidak memiliki data Institut!".to_string())
    })?;
    let response_data = service.find_links_by_institute(institute_id).await?;
    let message = format!("Menampilkan semua link untuk institut ID {}", institute_id);
    
    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn create_link_hand(
    uri: Uri,
    AuthUser(user): AuthUser,
    service: AppLinkService,
    ValidatedJson(data): ValidatedJson<LinkCreate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.create_link(user, data).await?;
    let message = format!("Link '{}' berhasil ditambahkan.", response_data.name);
    
    Ok(WebResponse::created(&uri, message, response_data))
}

pub async fn update_link_hand(
    ValidatedPath(link_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLinkService,
    ValidatedJson(data): ValidatedJson<LinkUpdate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.update_link(link_id, data).await?;
    let message = format!("Link '{}' berhasil diperbarui.", response_data.name);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn delete_link_hand(
    ValidatedPath(link_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLinkService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.delete_link(link_id).await?;
    let message = format!("Link '{}' berhasil dihapus.", response_data.name);

    Ok(WebResponse::ok(&uri, message, response_data))
}