use axum::{http::Uri, response::IntoResponse};
use crate::utils::{response::WebResponse, response::ApiError, request::ValidatedJson};
use crate::middleware::auth::AuthUser;
use crate::models::auth::{LoginReq, RefreshTokenReq, ResetPassword};
use crate::service::auth::AuthService;
use crate::repository::user::{UserRepository, TokenRepository};

type AppAuthService = AuthService<UserRepository, TokenRepository>;

pub async fn login_hand(
    uri: Uri,
    auth_service: AppAuthService,
    ValidatedJson(data): ValidatedJson<LoginReq>
) -> Result<impl IntoResponse, ApiError> {  
    let response_data = auth_service.login(data).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Login successfully!".to_string(), response_data))
}

pub async fn refresh_hand(
    uri: Uri,
    auth_service: AppAuthService,
    ValidatedJson(data): ValidatedJson<RefreshTokenReq>
) -> Result<impl IntoResponse, ApiError> {
    let response_data = auth_service.refresh(data.refresh_token).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok(&uri, "Refresh successfully!".to_string(), response_data))
}

pub async fn logout_hand(
    uri: Uri,
    AuthUser(_): AuthUser,
    auth_service: AppAuthService,
    ValidatedJson(data): ValidatedJson<RefreshTokenReq>
) -> Result<impl IntoResponse, ApiError> {
    auth_service.logout(data.refresh_token).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok_empty(&uri, "Logout successfully!".to_string()))
}

pub async fn reset_password_hand(
    uri: Uri,
    AuthUser(user): AuthUser,
    auth_service: AppAuthService,
    ValidatedJson(data): ValidatedJson<ResetPassword>
) -> Result<impl IntoResponse, ApiError> {
    // Catatan: Pastikan nama method di AuthService Anda adalah reset_pasword atau reset_password
    auth_service.reset_pasword(user, data).await.map_err(|e|e.with_path(&uri))?;

    Ok(WebResponse::ok_empty(&uri, "Password berhasil direset, silakan login kembali.".to_string()))
}