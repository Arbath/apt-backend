use axum::{http::Uri, response::IntoResponse};
use uuid::Uuid;
use crate::models::user::{UserReq, UserUpdate};
use crate::utils::request::ValidatedPath;
use crate::utils::{response::WebResponse, response::AppError, request::ValidatedJson};
use crate::middleware::auth::AuthAdmin;
use crate::service::user::UserService;
use crate::repository::user::{UserRepository};

type AppUserService = UserService<UserRepository>;

pub async fn detail_user_hand(
    ValidatedPath(user_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppUserService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.get_one_user(&user_id).await?;

    Ok(WebResponse::ok(&uri, "Success".to_string(), response_data))
}

pub async fn all_user_hand(
    uri: Uri,
    AuthAdmin(_): AuthAdmin,
    service: AppUserService,
) -> Result<impl IntoResponse, AppError> {  
    let response_data = service.get_all_users().await?;

    Ok(WebResponse::ok(&uri, "List semua users".to_string(), response_data))
}

pub async fn add_user_hand(
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppUserService,
    ValidatedJson(data): ValidatedJson<UserReq>
) -> Result<impl IntoResponse, AppError> {  
    let (message, response_data) = service.add_user(user, data).await?;
    
    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn edit_user_hand(
    ValidatedPath(user_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppUserService,
    ValidatedJson(data): ValidatedJson<UserUpdate>
) -> Result<impl IntoResponse, AppError> {  
    let (message, response_data) = service.edit_user(user, &user_id, data).await?;

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn delete_user_hand(
    ValidatedPath(user_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppUserService,
) -> Result<impl IntoResponse, AppError> {  
    let (message, response_data) = service.delete_user(user, &user_id).await?;

    Ok(WebResponse::ok(&uri, message, response_data))
}

pub async fn reset_password_user_hand(
    ValidatedPath(user_id): ValidatedPath<Uuid>,
    uri: Uri,
    AuthAdmin(user): AuthAdmin,
    service: AppUserService,
) -> Result<impl IntoResponse, AppError> {  
    let (message, response_data) = service.reset_password_user(user, &user_id,).await?;

    Ok(WebResponse::ok(&uri, message, response_data))
}