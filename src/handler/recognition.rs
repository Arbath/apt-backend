use axum::{extract::Query, response::IntoResponse};
use http::Uri;
use uuid::Uuid;

use crate::{middleware::auth::{AuthAdminOrAuditor, AuthUser, OptionalUser}, models::recognition::{RecognitionCategoryCreate, RecognitionCategoryUpdate, RecognitionLecturerCreate, RecognitionLecturerQuery, RecognitionLecturerResponse, RecognitionLecturerUpdate}, repository::{recognition::RecognitionLecturerRepository, recognition_category::RecognitionCategoryRepository, user::UserRepository}, service::recognition::{LecturerRecognitionService, RecognitionCatService}, utils::{request::{ValidatedJson, ValidatedPath}, response::{AppError, PaginationMeta, WebResponse}}};

type AppRecongnitionService = LecturerRecognitionService<UserRepository, RecognitionLecturerRepository>;
type AppRecongnitionCatService = RecognitionCatService<UserRepository, RecognitionCategoryRepository>;

pub async fn get_recognition_detail_hand(
    ValidatedPath(recognition_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppRecongnitionService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.get_recongnition_detail(recognition_id).await?;
    let message= format!("Detail Rekognisi '{}'", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn search_recognition_hand(
    Query(query): Query<RecognitionLecturerQuery>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppRecongnitionService,
) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let (response_data, total_items) = service.search_recongnition(query).await?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items: total_items as u64,
        total_pages
    };
    let message= format!("Hasil pencarian Rekognisi.");
    let response: Vec<RecognitionLecturerResponse> = response_data
        .into_iter()
        .map(Into::into)
        .collect();
    
    Ok(WebResponse::ok_paginated(&uri, message, response, meta))
}

pub async fn create_recognition_hand(
    uri: Uri,
    OptionalUser(user): OptionalUser,
    service: AppRecongnitionService,
    ValidatedJson(data):ValidatedJson<RecognitionLecturerCreate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.create_recognition(user, data).await?;
    let message= format!("Rekognisi '{}' berhasil diajukan", response_data.id);

    Ok(WebResponse::created(&uri, message, response_data))
}

pub async fn update_recognition_hand(
    ValidatedPath(recognition_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: AppRecongnitionService,
    ValidatedJson(data):ValidatedJson<RecognitionLecturerUpdate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.update_recognition(user, recognition_id, data).await?;
    let message= format!("Rekognisi '{}' berhasil diupdate", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn delete_recognition_hand(
    ValidatedPath(recognition_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: AppRecongnitionService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.delete_recongnition(user, recognition_id).await?;
    let message= format!("Rekognisi '{}' berhasil dihapus", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn approve_recognition_hand(
    ValidatedPath(recognition_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: AppRecongnitionService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.approve_recongnition(user, recognition_id).await?;
    let message= format!("Rekognisi '{}' berhasil disetujui", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn reject_recognition_hand(
    ValidatedPath(recognition_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(user): AuthUser,
    service: AppRecongnitionService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.reject_recongnition(user, recognition_id).await?;
    let message= format!("Rekognisi '{}' berhasil ditolak", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}

// RECOGNITION CATEGORY
// ====================
pub async fn get_all_recognition_cat_hand(
    uri: Uri,
    service: AppRecongnitionCatService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.find_all_category().await?;
    let message= format!("Semua Kategori");

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn get_recognition_cat_detail_hand(
    ValidatedPath(category_id): ValidatedPath<i32>,
    uri: Uri,
    service: AppRecongnitionCatService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.find_category_id(category_id).await?;
    let message= format!("Detail Kategori '{}'", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn get_recognition_cat_name_hand(
    ValidatedPath(category_name): ValidatedPath<String>,
    uri: Uri,
    service: AppRecongnitionCatService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.find_category_name(&category_name).await?;
    let message= format!("List Semua Kategori");

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn create_recognition_cat_hand(
    uri: Uri,
    AuthAdminOrAuditor(_): AuthAdminOrAuditor,
    service: AppRecongnitionCatService,
    ValidatedJson(data):ValidatedJson<RecognitionCategoryCreate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.create_category(data).await?;
    let message= format!("Kategori '{}' berhasil dibuat!", response_data.id);

    Ok(WebResponse::created(&uri, message, response_data))
}

pub async fn update_recognition_cat_hand(
    ValidatedPath(category_id): ValidatedPath<i32>,
    uri: Uri,
    AuthAdminOrAuditor(_): AuthAdminOrAuditor,
    service: AppRecongnitionCatService,
    ValidatedJson(data):ValidatedJson<RecognitionCategoryUpdate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.update_category(category_id, data).await?;
    let message= format!("Kategori '{}' berhasil diupdate!", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn delete_recognition_cat_hand(
    ValidatedPath(category_id): ValidatedPath<i32>,
    uri: Uri,
    AuthAdminOrAuditor(_): AuthAdminOrAuditor,
    service: AppRecongnitionCatService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.delete_category(category_id).await?;
    let message= format!("Kategori '{}' berhasil dihapus!", response_data.id);

    Ok(WebResponse::ok(&uri, message, response_data))
}