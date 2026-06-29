use axum::{response::IntoResponse};
use http::Uri;
use uuid::Uuid;

use crate::{middleware::auth::{AuthAdminOrAuditee, AuthUser, OptionalUser}, models::lecturer::{LecturerCreate, LecturerQuery, LecturerResponse, LecturerUpdate}, repository::{institute::InstituteRepository, lecturer::LecturerRepository, study_program::StudyProgramRepository, user::UserRepository}, service::lecturer::LecturerService, utils::{request::{ValidatedJson, ValidatedPath, ValidatedQuery}, response::{ApiError, PaginationMeta, WebResponse}}};

type AppLecturerService = LecturerService<UserRepository, LecturerRepository, InstituteRepository, StudyProgramRepository>;

pub async fn get_lecturer_detail_hand(
    ValidatedPath(lecturer_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLecturerService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.get_lecturer_detail(lecturer_id).await.map_err(|e|e.with_path(&uri))?;
    let message= format!("Detail Dosen '{}'", response_data.name);
    let response: LecturerResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}

pub async fn get_lecturer_nip_hand(
    ValidatedPath(lecturer_nip): ValidatedPath<String>,
    uri: Uri,
    service: AppLecturerService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.get_lecturer_by_nip(lecturer_nip).await.map_err(|e|e.with_path(&uri))?;
    let message= format!("Detail Dosen '{}'", response_data.name);
    let response: LecturerResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}

pub async fn search_lecturer_hand(
    ValidatedQuery(query): ValidatedQuery<LecturerQuery>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppLecturerService,
) -> Result<impl IntoResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let (response_data, total_items) = service.search_lecturer(query).await.map_err(|e|e.with_path(&uri))?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items: total_items as u64,
        total_pages
    };
    let message= format!("Hasil pencarian dosen.");
    let response: Vec<LecturerResponse> = response_data
        .into_iter()
        .map(Into::into)
        .collect();
    
    Ok(WebResponse::ok_paginated(&uri, message, response, meta))
}

pub async fn add_lecturer_hand(
    uri: Uri,
    OptionalUser(user): OptionalUser,
    service: AppLecturerService,
    ValidatedJson(data): ValidatedJson<LecturerCreate>
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.add_lecturer(user, data).await.map_err(|e|e.with_path(&uri))?;
    let message= format!("Dosen bernama '{}' berhasi diajukan!", response_data.name);
    let response: LecturerResponse = response_data.into();

    Ok(WebResponse::created(&uri, message, response))
}

pub async fn edit_lecturer_hand(
    ValidatedPath(lecturer_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdminOrAuditee(_): AuthAdminOrAuditee,
    service: AppLecturerService,
    ValidatedJson(data): ValidatedJson<LecturerUpdate>
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.edit_lecturer(lecturer_id, data).await.map_err(|e|e.with_path(&uri))?;
    let message= format!("Detail Dosen '{}'", response_data.name);
    let response: LecturerResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}

pub async fn delete_lecturer_hand(
    ValidatedPath(lecturer_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdminOrAuditee(_): AuthAdminOrAuditee,
    service: AppLecturerService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.remove_lecturer(lecturer_id).await.map_err(|e|e.with_path(&uri))?;
    let message= format!("Dosen bernama '{}' berhasi dihapus!", response_data.name);
    let response: LecturerResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}

pub async fn approve_lecturer_hand(
    ValidatedPath(lecturer_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdminOrAuditee(_): AuthAdminOrAuditee,
    service: AppLecturerService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.approve_lecturer(lecturer_id).await.map_err(|e|e.with_path(&uri))?;
    let message= format!("Dosen bernama '{}' telah disetujui.", response_data.name);
    let response: LecturerResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}

pub async fn reject_lecturer_hand(
    ValidatedPath(lecturer_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdminOrAuditee(_): AuthAdminOrAuditee,
    service: AppLecturerService,
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = service.reject_lecturer(lecturer_id).await.map_err(|e|e.with_path(&uri))?;
    let message= format!("Dosen bernama '{}' telah ditolak.", response_data.name);
    let response: LecturerResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}