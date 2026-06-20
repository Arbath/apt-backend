use std::sync::Arc;
use apt_backend::models::auth::{Claims, LoginReq, ResetPassword};
use apt_backend::models::user::{User, RoleUsers};
use apt_backend::domain::repository::{MockTokenRepoTrait, MockUserRepoTrait};
use apt_backend::service::auth::AuthService;
use apt_backend::state::AppConfig;
use apt_backend::utils::response::AppError;
use mockall::predicate::*;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header, EncodingKey};

// ---------------------------------------------------------
// HELPER FUNCTIONS
// ---------------------------------------------------------

// Pastikan menggunakan Arc<AppConfig>
fn create_dummy_config() -> Arc<AppConfig> {
    Arc::new(AppConfig {
        secret: "rahasia_negara_super_aman_1234567890".to_string(),
        access_ttl: 1, 
        refresh_ttl: 24, 
        cors: CorsLayer::new(),
    })
}

fn create_dummy_user() -> User {
    User {
        id: Uuid::new_v4(),
        username: "dosen123".to_string(),
        email: "dosen@kampus.ac.id".to_string(),
        password: "hashed_password_placeholder".to_string(), 
        name: "Budi Santoso".to_string(),
        role: RoleUsers::AUDITEE,
        institute_id: Some(1),
        is_banned: false,
        must_change_password: false,
        created_at: Utc::now(),
    }
}

// Fungsi pembantu untuk membuat JWT valid agar bisa mem-bypass validasi awal di service
fn generate_valid_token(secret: &str) -> String {
    let dummy_user = create_dummy_user();
    let claims = Claims { 
        email: dummy_user.email,
        token_type: "refresh".to_string(),
        sub: dummy_user.id,
        iat: (Utc::now() + Duration::hours(1)).timestamp() as usize,
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize 
    };
    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes())
    ).unwrap()
}

// ---------------------------------------------------------
// TEST CASES
// ---------------------------------------------------------

#[tokio::test]
async fn test_login_invalid_username_from_outside() {
    let mut mock_user_repo = MockUserRepoTrait::new();
    let mock_token_repo = MockTokenRepoTrait::new();

    mock_user_repo
        .expect_find_by_username()
        .with(eq("salah_user"))
        .times(1)
        .returning(|_| Ok(None));

    let config = create_dummy_config();
    let auth_service = AuthService::new(mock_user_repo, mock_token_repo, (*config).clone());

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
async fn test_logout_success() {
    let mock_user_repo = MockUserRepoTrait::new();
    let mut mock_token_repo = MockTokenRepoTrait::new();

    let config = create_dummy_config();
    
    // 1. Buat token JWT asli yang valid
    let valid_jwt = generate_valid_token(&config.secret);

    // 2. Ekspektasi: revoke dipanggil dengan token asli tersebut
    mock_token_repo
        .expect_revoke()
        .with(eq(valid_jwt.clone()))
        .times(1)
        .returning(|_| Ok(()));

    let auth_service = AuthService::new(mock_user_repo, mock_token_repo, (*config).clone());

    // 3. Eksekusi
    let result = auth_service.logout(valid_jwt).await;

    // 4. Validasi
    assert!(result.is_ok(), "Logout seharusnya berhasil");
}

#[tokio::test]
async fn test_refresh_token_revoked() {
    let mock_user_repo = MockUserRepoTrait::new();
    let mut mock_token_repo = MockTokenRepoTrait::new();

    let config = create_dummy_config();
    let valid_jwt = generate_valid_token(&config.secret);

    // Ekspektasi: token valid secara format, tetapi ditolak database karena sudah di-revoke (exists = false)
    mock_token_repo
        .expect_exists()
        .with(eq(valid_jwt.clone()))
        .times(1)
        .returning(|_| Ok(false));

    let auth_service = AuthService::new(mock_user_repo, mock_token_repo, (*config).clone());

    let result = auth_service.refresh(valid_jwt).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::AuthError(msg) => assert_eq!(msg, "Refresh token has been revoked"),
        _ => panic!("Expected AuthError for revoked token!"),
    }
}

#[tokio::test]
async fn test_reset_password_success() {
    let mut mock_user_repo = MockUserRepoTrait::new();
    let mock_token_repo = MockTokenRepoTrait::new();
    
    let dummy_user = create_dummy_user();
    let cloned_user = dummy_user.clone(); // Untuk dicocokkan di dalam closure

    // Ekspektasi: Fungsi update_password berhasil dieksekusi (dipanggil 1 kali)
    // Parameter pertama (id) harus sesuai dengan id user, must_change = false.
    mock_user_repo
        .expect_update_password()
        .withf(move |id, _hash, must_change| {
            id == &cloned_user.id && *must_change == false
        })
        .times(1)
        .returning(|_, _, _| Ok(create_dummy_user())); // Return bebas karena Ok(()) sudah cukup jika structnya tidak mengembalikan data spesifik. Jika repository mengembalikan User, biarkan seperti ini.

    let config = create_dummy_config();
    let auth_service = AuthService::new(mock_user_repo, mock_token_repo, (*config).clone());

    let req = ResetPassword {
        password1: "passwordBaru123!".to_string(),
        password2: "passwordBaru123!".to_string(), // Harus sama persis
    };

    let result = auth_service.reset_pasword(dummy_user, req).await;

    assert!(result.is_ok(), "Reset password seharusnya berhasil jika input valid");
}

#[tokio::test]
async fn test_reset_password_mismatch() {
    let mut mock_user_repo = MockUserRepoTrait::new();
    let mock_token_repo = MockTokenRepoTrait::new();

    // Pastikan update database TIDAK PERNAH terjadi jika validasi gagal
    mock_user_repo.expect_update_password().times(0);

    let config = create_dummy_config();
    let auth_service = AuthService::new(mock_user_repo, mock_token_repo, (*config).clone());

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