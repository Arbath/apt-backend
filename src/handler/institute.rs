use axum::extract::Query;
use axum::{http::Uri, response::IntoResponse};
use crate::models::institute::{InstituteCreate, InstituteQueryParams, InstituteUpdate, StudyProgramCreate, StudyProgramResponse, StudyProgramUpdate};
use crate::repository::institute::InstituteRepository;
use crate::repository::study_program::StudyProgramRepository;
use crate::service::institute::InstituteService;
use crate::utils::request::ValidatedPath;
use crate::utils::response::PaginationMeta;
use crate::utils::{response::WebResponse, response::AppError, request::ValidatedJson};
use crate::middleware::auth::{AuthAdmin};
use crate::repository::user::{UserRepository};

type AppInstituteService = InstituteService<UserRepository, InstituteRepository, StudyProgramRepository >;

pub async fn get_one_institute_hand(
    ValidatedPath(institute_id): ValidatedPath<i32>,
    uri: Uri,
    service: AppInstituteService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.get_one_institute(institute_id).await?;
    let message= format!("Detail lembaga '{}'", response_data.name);
    
    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn search_institute_hand(
    Query(query): Query<InstituteQueryParams>,
    uri: Uri,
    service: AppInstituteService,
) -> Result<impl IntoResponse, AppError> {
    let institute_name = query.name.clone();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let (response_data, total_items) = match institute_name {
        Some(name) => service.get_one_institute_name(&name, page as i64, limit as i64).await?,
        None => service.get_all_institute(page as i64, limit as i64).await?
    };
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items,
        total_pages
    };
    let message = match &query.name {
        Some(nama) => format!("List lembaga dengan keyword '{}'", nama),
        None => "Menampilkan semua daftar lembaga".to_string(),
    };

    Ok(WebResponse::ok_paginated(&uri, message, response_data, meta))
}

pub async fn get_all_institute_study_programs_hand(
    ValidatedPath(institute_id): ValidatedPath<i32>,
    uri: Uri,
    service: AppInstituteService,
) -> Result<impl IntoResponse, AppError> {  
    let (message, response_data) = service.get_all_institute_study_programs(institute_id).await?;
    
    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn add_institute_hand(
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppInstituteService,
    ValidatedJson(data): ValidatedJson<InstituteCreate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.add_institute(data).await?;
    let message= format!("Lembaga '{}' berhasil ditambahkan.", response_data.name);
    
    Ok(WebResponse::created(&uri, message, response_data))
}

pub async fn edit_institute_hand(
    ValidatedPath(institute_id): ValidatedPath<i32>,
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppInstituteService,
    ValidatedJson(data): ValidatedJson<InstituteUpdate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.edit_institute(institute_id, data).await?;
    let message= format!("Lembaga '{}' berhasil diperbarui.", response_data.name);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn delete_institute_hand(
    ValidatedPath(institute_id): ValidatedPath<i32>,
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppInstituteService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.delete_institute(institute_id).await?;
    let message= format!("Lembaga dengan id '{}' berhasi dihapus.", response_data.name);

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn get_one_study_program_hand(
    ValidatedPath(program_id): ValidatedPath<i32>,
    uri: Uri,
    service: AppInstituteService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.get_one_study_prg(program_id).await?;
    let message= format!("Detail Program Studi '{}'", response_data.name);
        let response: StudyProgramResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}

pub async fn search_study_program_hand(
    Query(query): Query<InstituteQueryParams>,
    uri: Uri,
    service: AppInstituteService,
) -> Result<impl IntoResponse, AppError> {
    let program_name = query.name.clone();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);

    let (response_data, total_items) = match program_name {
        Some(name) => service.get_one_study_prg_name(&name, page as i64, limit as i64).await?,
        None => service.get_all_study_prg(page as i64, limit as i64).await?
    };
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items,
        total_pages
    };
    let message = match &query.name {
        Some(nama) => format!("List program studi dengan keyword '{}'", nama),
        None => "Menampilkan semua daftar program studi".to_string(),
    };
    let response: Vec<StudyProgramResponse> = response_data
        .into_iter()
        .map(Into::into)
        .collect();

    Ok(WebResponse::ok_paginated(&uri, message, response, meta))
}

pub async fn add_study_program_hand(
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppInstituteService,
    ValidatedJson(data): ValidatedJson<StudyProgramCreate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.add_study_prg(data).await?;
    let message= format!("Program Studi '{}' berhasil ditambahkan.", response_data.name);
        let response: StudyProgramResponse = response_data.into();

    Ok(WebResponse::created(&uri, message, response))
}

pub async fn edit_study_program_hand(
    ValidatedPath(program_id): ValidatedPath<i32>,
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppInstituteService,
    ValidatedJson(data): ValidatedJson<StudyProgramUpdate>
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.edit_study_prg(program_id, data).await?;
    let message= format!("Program Studi '{}' berhasil diperbarui.", response_data.name);
    let response: StudyProgramResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}

pub async fn delete_study_program_hand(
    ValidatedPath(program_id): ValidatedPath<i32>,
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppInstituteService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.delete_study_prg(program_id).await?;
    let message= format!("Program Studi dengan id '{}' berhasi dihapus.", response_data.name);
    let response: StudyProgramResponse = response_data.into();

    Ok(WebResponse::ok(&uri, message, response))
}