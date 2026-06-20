use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts};
use chrono::{Duration, Utc};

use crate::{models::{auth::{LoginReq, LoginRes, ResetPassword}, user::User}, state::AppConfig}; 
use crate::state::AppState;
use crate::utils::{auth::*, response::AppError};

use crate::domain::repository::{UserRepoTrait, TokenRepoTrait}; 
use crate::repository::user::{UserRepository, TokenRepository};

pub struct AuthService<U: UserRepoTrait, T: TokenRepoTrait> {
    user_repo: U,
    token_repo: T,
    config: AppConfig,
}

impl<U: UserRepoTrait, T: TokenRepoTrait> AuthService<U, T> {
    
    pub fn new(user_repo: U, token_repo: T, config: AppConfig) -> Self {
        Self { user_repo, token_repo, config }
    }
    
    pub async fn login(&self, req: LoginReq) -> Result<LoginRes, AppError> {
        let refresh_ttl = 60 * 60 * self.config.access_ttl;
        let user = self.authenticate(&req.username, &req.password).await?;
        let expiration_time = Utc::now() + Duration::seconds(refresh_ttl as i64); 
        let access_token = gen_access_token(&user, &self.config).await?;
        let refresh_token = gen_refresh_token(&user, &self.config).await?;
        
        self.token_repo.save_token(&refresh_token, user.id, expiration_time).await?;

        Ok(LoginRes { access_token, refresh_token, user })
    }

    pub async fn logout(&self, refresh_token_str: String) -> Result<(), AppError> {
        let _ = verify_refresh_token(&self.config.secret, &refresh_token_str)
             .map_err(|_| AppError::AuthError("Invalid token".to_string()))?;

        self.token_repo.revoke(&refresh_token_str).await?;
        
        Ok(())
    }

    pub async fn refresh(&self, token_str: String) -> Result<LoginRes, AppError> {
        let claims = verify_refresh_token(&self.config.secret, &token_str)?;
        let exists = self.token_repo.exists(&token_str)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        if !exists {
            return Err(AppError::AuthError("Refresh token has been revoked".to_string()));
        }

        let user_id = claims.sub;

        let user = self.user_repo.find_by_id(&user_id).await?;
        let refresh_ttl = 60 * 60 * self.config.refresh_ttl;
        let expiration_time = Utc::now() + Duration::seconds(refresh_ttl as i64); 
        let access_token = gen_access_token(&user, &self.config).await?;
        let refresh_token = gen_refresh_token(&user, &self.config).await?;
        
        self.token_repo.revoke(&token_str).await?;
        self.token_repo.save_token(&refresh_token, user_id, expiration_time).await?;

        Ok(LoginRes { access_token, refresh_token, user })
    }

    pub async fn reset_pasword(&self, user: User, data: ResetPassword) -> Result<(), AppError> {
        if data.password1 != data.password2 {
            return Err(AppError::AuthError(format!("Kedua password tidak sama!")));
        }
        
        let password_hash = tokio::task::spawn_blocking(move || {
            crate::utils::hash::generate(&data.password1) 
        })
        .await
        .map_err(|e| AppError::InternalError(format!("Hash verify failed: {}", e)))??;

        self.user_repo.update_password(&user.id, password_hash, false).await?;

        Ok(())
    }

    async fn authenticate(&self, username: &str, password: &str) -> Result<User, AppError> {
        let user_opt = self.user_repo.find_by_username(username).await?;
        let user = match user_opt {
            Some(u) => u,
            None => return Err(AppError::AuthError("Invalid username or password".to_string())),
        };

        let plain_password = password.to_string();
        let hash_from_db = user.password.clone();

        let is_valid = tokio::task::spawn_blocking(move || {
            crate::utils::hash::verify(&plain_password, &hash_from_db)
        })
        .await
        .map_err(|e| AppError::InternalError(format!("Hash verify failed: {}", e)))??;
 
        if !is_valid {
            return Err(AppError::AuthError("Invalid username or password".to_string()));
        }

        Ok(user)
    }
}

impl<S> FromRequestParts<S> for AuthService<UserRepository, TokenRepository>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let user_repo = UserRepository::new(state.database.clone());
        let token_repo = TokenRepository::new(state.database.clone());
        
        Ok(AuthService::new(user_repo, token_repo, (*state.app_config).clone()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::auth::{Claims, LoginReq, ResetPassword};
    use crate::models::user::{User, RoleUsers};
    use crate::domain::repository::{MockUserRepoTrait, MockTokenRepoTrait};
    use crate::state::AppConfig;
    use mockall::predicate::*;
    use tower_http::cors::CorsLayer;
use uuid::Uuid;
    use chrono::Utc;

    // Helper untuk membuat konfigurasi palsu
    fn create_dummy_config() -> AppConfig {
        AppConfig {
            secret: "rahasia_negara_super_aman_1234567890".to_string(),
            access_ttl: 1, // 1 jam
            refresh_ttl: 24, // 24 jam
            cors: CorsLayer::new()
        }
    }

    fn create_dummy_user() -> User {
        User {
            id: Uuid::new_v4(),
            username: "dosen123".to_string(),
            email: "dosen@kampus.ac.id".to_string(),
            password: "$argon2id$v=19$m=19456,t=2,p=1$...".to_string(), 
            name: "Budi Santoso".to_string(),
            role: RoleUsers::AUDITEE,
            institute_id: Some(1),
            is_banned: false,
            must_change_password: true,
            created_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_login_invalid_username() {
        let mut mock_user_repo = MockUserRepoTrait::new();
        let mock_token_repo = MockTokenRepoTrait::new();

        mock_user_repo
            .expect_find_by_username()
            .with(eq("salah_user"))
            .times(1)
            .returning(|_| Ok(None));

        let config = create_dummy_config();
        let auth_service = AuthService::new(mock_user_repo, mock_token_repo, config);

        let req = LoginReq {
            username: "salah_user".to_string(),
            password: "any_password".to_string(),
        };
        let result = auth_service.login(req).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::AuthError(msg) => assert_eq!(msg, "Invalid username or password"),
            _ => panic!("Expected AuthError!"),
        }
    }

    #[tokio::test]
    async fn test_reset_password_mismatch() {
        let mut mock_user_repo = MockUserRepoTrait::new();
        let mock_token_repo = MockTokenRepoTrait::new();

        mock_user_repo.expect_update_password().times(0);

        let config = create_dummy_config();
        let auth_service = AuthService::new(mock_user_repo, mock_token_repo, config);

        let dummy_user = create_dummy_user();
        let req = ResetPassword {
            password1: "baru123".to_string(),
            password2: "beda123".to_string(), 
        };

        let result = auth_service.reset_pasword(dummy_user, req).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::AuthError(msg) => assert_eq!(msg, "Kedua password tidak sama!"),
            _ => panic!("Expected AuthError for mismatched passwords!"),
        }
    }

    #[tokio::test]
    async fn test_logout_success() {
        let mock_user_repo = MockUserRepoTrait::new();
        let mut mock_token_repo = MockTokenRepoTrait::new();
        let config = create_dummy_config();

        let dummy_user = create_dummy_user();
        let secret = config.secret.as_bytes();
        use jsonwebtoken::{encode, Header, EncodingKey};
        let claims = Claims { 
            email: dummy_user.email,
            token_type: "refresh".to_string(),
            sub: dummy_user.id, 
            iat: (Utc::now() + Duration::hours(1)).timestamp() as usize,
            exp: (Utc::now() + Duration::hours(1)).timestamp() as usize 
        };
        let real_jwt = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap();

        mock_token_repo
            .expect_revoke()
            .with(eq(real_jwt.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let auth_service = AuthService::new(mock_user_repo, mock_token_repo, config);

        let result = auth_service.logout(real_jwt).await;
        assert!(result.is_ok());
    }
}