use axum::{response::IntoResponse};
use http::Uri;
use uuid::Uuid;

use crate::{middleware::auth::{AuthAdmin, AuthAdminOrAuditor, AuthUser}, models::accreditation::{AccreditationCreate, AccreditationUpdate, CalculationQuery, CalculationRuleCreate, CalculationRuleUpdate, EvaluationCreate, EvaluationQuery, EvaluationUpdate, IndicatorCreate, IndicatorQuery, IndicatorUpdate}, repository::{accreditation::AccreditationRepository, calculation::CalculationRuleRepository, evaluation::EvaluationRepository, feature::LogActivityRepository, indicator::IndicatorRepository}, service::{accreditation::AccreditationService, feature::LogActivityService}, utils::{request::{ValidatedJson, ValidatedPath, ValidatedQuery}, response::{ApiError, PaginationMeta, WebResponse}}};

type AppAccreditation = AccreditationService<AccreditationRepository, IndicatorRepository, CalculationRuleRepository, EvaluationRepository>;
type AppLogService = LogActivityService<LogActivityRepository>;

// ACCREDITATION
pub async fn get_accreditation_detail(
    ValidatedPath(accreditation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_accr_detail(accreditation_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail akreditasi".to_string(), data))
}

pub async fn get_all_accreditation(
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_accr_all().await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "List semua Akreditasi".to_string(), data))
}

pub async fn create_accreditation(
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<AccreditationCreate>,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User membuat akreditasi {}", data.name);
    let data = service.add_accr(data).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::created(&uri, "Akreditasi berhasil dibuat!".to_string(), data))
}

pub async fn update_accreditation(
    ValidatedPath(accreditation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<AccreditationUpdate>,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User mengubah akreditasi dengan id {}", accreditation_id);
    let data = service.edit_accr(accreditation_id, data).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;

    Ok(WebResponse::ok(&uri, "Akreditasi berhasil diperbarui!".to_string(), data))
}

pub async fn delete_accreditation(
    ValidatedPath(accreditation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User menghapus akreditasi dengan id {}", accreditation_id);
    let data = service.remove_accr(accreditation_id).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;

    Ok(WebResponse::ok(&uri, "Akreditasi berhasil dihapus!".to_string(), data))
}

pub async fn get_one_accreditation_stats(
    ValidatedPath(accreditation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_one_accr_stats(accreditation_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail statistik akreditasi".to_string(), data))
}

pub async fn get_all_accreditation_stats(
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_all_accr_stats().await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail semua statistik akreditasi".to_string(), data))
}

// INDICATOR
pub async fn get_indicator_detail(
    ValidatedPath(indicator_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_indicator_detail(indicator_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail indikator akreditasi".to_string(), data))
}

pub async fn search_indicator(
    ValidatedQuery(query): ValidatedQuery<IndicatorQuery>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let (data, total_items) = service.search_indicator(query).await.map_err(|e|e.with_path(&uri))?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items: total_items as u64,
        total_pages
    };

    Ok(WebResponse::ok_paginated(&uri, "Hasil pencarian indikator akreditasi".to_string(), data, meta))
}

pub async fn create_indicator(
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<IndicatorCreate>,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.add_indicator(data).await.map_err(|e|e.with_path(&uri))?;
    let activity = format!("User membuat indikator dengan id {}", data.id);
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::created(&uri, "Indikator akreditasi berhasil dibuat!".to_string(), data))
}

pub async fn update_indicator(
    ValidatedPath(indicator_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<IndicatorUpdate>,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User mengubah indikator dengan id {}", indicator_id);
    let data = service.edit_indicator(indicator_id, data).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::ok(&uri, "Indikator akreditasi berhasil diperbarui!".to_string(), data))
}

pub async fn delete_indicator(
    ValidatedPath(indicator_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User menghapus indikator dengan id {}", indicator_id);
    let data = service.remove_indicator(indicator_id).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::ok(&uri, "Indikator akreditasi berhasil dihapus!".to_string(), data))
}

pub async fn get_one_indicator_stats(
    ValidatedPath(indicator_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_one_indicator_stats(indicator_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail akreditasi".to_string(), data))
}

pub async fn get_all_indicator_stats(
    ValidatedPath(accreditation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_all_indicator_stats(accreditation_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail akreditasi".to_string(), data))
}

// CALCULATION
pub async fn get_calculation_detail(
    ValidatedPath(rule_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_calculation_detail(rule_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail kalkulasi indikator akreditasi".to_string(), data))
}

pub async fn search_calculation(
    ValidatedQuery(query): ValidatedQuery<CalculationQuery>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let (data, total_items) = service.search_calculation(query).await.map_err(|e|e.with_path(&uri))?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items: total_items as u64,
        total_pages
    };

    Ok(WebResponse::ok_paginated(&uri, "Hasil pencarian kalkulasi indikator akreditasi".to_string(), data, meta))
}

pub async fn create_calculation(
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<CalculationRuleCreate>,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.add_calculation(data).await.map_err(|e|e.with_path(&uri))?;
    let activity = format!("User membuat kalkulasi dengan id {}", data.id);
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::created(&uri, "Kalkulasi indikator akreditasi berhasil dibuat!".to_string(), data))
}

pub async fn update_calculation(
    ValidatedPath(rule_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<CalculationRuleUpdate>,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User mengubah kalkulasi dengan id {}", rule_id);
    let data = service.edit_calculation(rule_id, data).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::ok(&uri, "Kalkulasi indikator akreditasi berhasil diperbarui!".to_string(), data))
}

pub async fn delete_calculation(
    ValidatedPath(rule_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    log_service: AppLogService,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User menghapus kalkulasi dengan id {}", rule_id);
    let data = service.remove_calculation(rule_id).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::ok(&uri, "Kalkulasi indikator akreditasi berhasil dihapus!".to_string(), data))
}

// EVALUATION
pub async fn get_evaluation_detail(
    ValidatedPath(evaluation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.get_evaluation_detail(evaluation_id).await.map_err(|e|e.with_path(&uri))?;
    
    Ok(WebResponse::ok(&uri, "Detail evaluasi indikator akreditasi".to_string(), data))
}

pub async fn search_evaluation(
    ValidatedQuery(query): ValidatedQuery<EvaluationQuery>,
    uri: Uri,
    AuthUser(_): AuthUser,
    service: AppAccreditation,
) -> Result<impl IntoResponse, ApiError> {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let (data, total_items) = service.search_evaluation(query).await.map_err(|e|e.with_path(&uri))?;
    let total_pages = (total_items as f64 / limit as f64).ceil() as u64;
    let meta = PaginationMeta {
        current_page: page,
        limit_page: limit,
        total_items: total_items as u64,
        total_pages
    };

    Ok(WebResponse::ok_paginated(&uri, "Hasil pencarian evaluasi indikator akreditasi".to_string(), data, meta))
}

pub async fn create_evaluation(
    uri: Uri,
    AuthAdminOrAuditor(user): AuthAdminOrAuditor,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<EvaluationCreate>,
) -> Result<impl IntoResponse, ApiError> {
    let data = service.add_evaluation(user.clone(), data).await.map_err(|e|e.with_path(&uri))?;
    let activity = format!("User menambahkan evaluasi dengan id {}", data.id);
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::created(&uri, "Evaluasi indikator akreditasi berhasil dibuat!".to_string(), data))
}

pub async fn update_evaluation(
    ValidatedPath(evaluation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdminOrAuditor(user): AuthAdminOrAuditor,
    service: AppAccreditation,
    log_service: AppLogService,
    ValidatedJson(data): ValidatedJson<EvaluationUpdate>,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User mengubah evaluasi dengan id {}", evaluation_id);
    let data = service.edit_evaluation(evaluation_id, data).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::ok(&uri, "Evaluasi indikator akreditasi berhasil diperbarui!".to_string(), data))
}

pub async fn delete_evaluation(
    ValidatedPath(evaluation_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppAccreditation,
    log_service: AppLogService,
) -> Result<impl IntoResponse, ApiError> {
    let activity = format!("User menghapus evaluasi dengan id {}", evaluation_id);
    let data = service.remove_evaluation(evaluation_id).await.map_err(|e|e.with_path(&uri))?;
    let _= log_service.add_log(user.id, activity).await;
    
    Ok(WebResponse::ok(&uri, "Evaluasi indikator akreditasi berhasil dihapus!".to_string(), data))
}