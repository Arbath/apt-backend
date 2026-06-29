use axum::http::Uri;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::middleware::auth::{AuthAdmin, AuthUser};
use crate::models::feature::{LinkCreate, LinkUpdate, LogActivityQuery};
use crate::repository::feature::{LinkRepository, LogActivityRepository};
use crate::repository::user::UserRepository;
use crate::service::feature::{LinkService, LogActivityService};
use crate::utils::request::{ValidatedJson, ValidatedPath, ValidatedQuery};
use crate::utils::response::{ApiError, AppError, PaginationMeta, WebResponse};

type AppLinkService = LinkService<UserRepository, LinkRepository>;
type AppLogService = LogActivityService<LogActivityRepository>;

pub async fn get_link_by_id_hand(
    ValidatedPath(link_id): ValidatedPath<Uuid>,
    uri: Uri,
    service: AppLinkService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.find_link_id(link_id).await.map_err(|e|e.with_path(&uri))?;
    let message = format!("Detail link '{}'", response_data.name);
    
    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn get_link_by_slug_hand(
    ValidatedPath(slug): ValidatedPath<String>,
    uri: Uri,
    service: AppLinkService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.find_link_slug(slug).await.map_err(|e|e.with_path(&uri))?;
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
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.create_link(user, data).await.map_err(|e|e.with_path(&uri))?;
    let message = format!("Link '{}' berhasil ditambahkan.", response_data.name);
    
    Ok(WebResponse::created(&uri, message, response_data))
}

pub async fn update_link_hand(
    ValidatedPath(link_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLinkService,
    ValidatedJson(data): ValidatedJson<LinkUpdate>
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.update_link(link_id, data).await.map_err(|e|e.with_path(&uri))?;
    let message = format!("Link '{}' berhasil diperbarui.", response_data.name);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn delete_link_hand(
    ValidatedPath(link_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLinkService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.delete_link(link_id).await.map_err(|e|e.with_path(&uri))?;
    let message = format!("Link '{}' berhasil dihapus.", response_data.name);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn get_log_detail(
    ValidatedPath(log_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLogService,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_log_detail(log_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail log".to_string(), data))
}

pub async fn search_logs(
    ValidatedQuery(query): ValidatedQuery<LogActivityQuery>,
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppLogService,
) -> Result<impl IntoResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let (data, total_items) = service.search_logs(query).await.map_err(|e|e.with_path(&uri))?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items: total_items as u64,
        total_pages
    };

    Ok(WebResponse::ok_paginated(&uri, "Hasil pencarian log".to_string(), data, meta))
}

pub async fn delete_log(
    ValidatedPath(log_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLogService,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.remove_log(log_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Log berhasil dihapus!".to_string(), data))
}

pub async fn delete_logs(
    ValidatedPath(days): ValidatedPath<i64>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLogService,
) -> Result<impl IntoResponse, ApiError> {
    let message = service.remove_log_days(days).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, message , None::<()>))
}