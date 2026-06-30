use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use uuid::Uuid;

use crate::{domain::repository::UserRepoTrait, models::user::{RoleUsers, User, UserReq, UserUpdate}, repository::user::UserRepository, state::{AppConfig, AppState}, utils::{hash::generate_async, response::AppError}};

#[allow(dead_code)]
pub struct UserService<U: UserRepoTrait> {
    user_repo: U,
    config: AppConfig,
}

impl<U: UserRepoTrait> UserService<U>{
    pub fn new(user_repo: U, config: AppConfig) -> Self {
        Self { user_repo, config }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError>{
        let users = self.user_repo.get_all().await?;

        Ok(users)
    }
    
    pub async fn get_one_user(&self, user_id: &Uuid) -> Result<User, AppError>{
        let users = self.user_repo.find_by_id(&user_id).await?;

        Ok(users)
    }

    pub async fn add_user(&self, user:User, data: UserReq)-> Result<(String, User), AppError>{
        if user.role != RoleUsers::ADMIN {
            return Err(AppError::Forbidden(format!("Hanya admin yang dapat menambahkan user!")));
        }
        let final_data = data.normalize();
        let password_hash = generate_async(final_data.username.clone()).await?;
        let q = match self.user_repo.create(final_data, password_hash).await {
            Ok(user) => user,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" { // 23505 = Unique Constraint Violation
                                return Err(AppError::BadRequest(
                                    "Gagal menambahkan user. Username sudah terdaftar!".to_string()
                                ));
                            }
                            if code == "23503" { // 23503 = Foreign Key Violation
                                return Err(AppError::BadRequest(
                                    "Institute ID yang Anda masukkan tidak valid/tidak ditemukan!".to_string()
                                ));
                            }
                        }
                        
                        // Jika error database lainnya (misal tipe data salah), 
                        // lempar sebagai DatabaseError agar dilog ke terminal tapi disembunyikan dari user
                        return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
                    }
                    // Tangkap RowNotFound jika memakai fetch_one tapi data tidak ada
                    sqlx::Error::RowNotFound => {
                        return Err(AppError::NotFound("Data tidak ditemukan.".to_string()));
                    }
                    // Error teknis lainnya seperti koneksi ke PostgreSQL terputus
                    other_error => {
                        return Err(AppError::DatabaseError(other_error));
                    }
                }
            }
        };
        let message = format!("User '{}' dengan password '{}' berhasil ditambahkan!", q.username, q.username);

        Ok((message, q))
    }
    
    pub async fn edit_user(&self, user:User,user_id: &Uuid, data: UserUpdate)-> Result<(String, User), AppError>{
        if user.role != RoleUsers::ADMIN {
            return Err(AppError::Forbidden(format!("Hanya admin yang dapat mengedit user!")));
        };

        let q = match self.user_repo.update(user_id, data.normalize()).await {
            Ok(user) => user,
            Err(e) => {
                match e {
                    sqlx::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" { // 23505 = Unique Constraint Violation
                                return Err(AppError::BadRequest(
                                    "Gagal menambahkan user. Username sudah terdaftar!".to_string()
                                ));
                            }
                            if code == "23503" { // 23503 = Foreign Key Violation
                                return Err(AppError::BadRequest(
                                    "Institute ID yang Anda masukkan tidak valid/tidak ditemukan!".to_string()
                                ));
                            }
                        }
                        
                        // Jika error database lainnya (misal tipe data salah), 
                        // lempar sebagai DatabaseError agar dilog ke terminal tapi disembunyikan dari user
                        return Err(AppError::DatabaseError(sqlx::Error::Database(db_err)));
                    }
                    // Tangkap RowNotFound jika memakai fetch_one tapi data tidak ada
                    sqlx::Error::RowNotFound => {
                        return Err(AppError::NotFound("Data tidak ditemukan.".to_string()));
                    }
                    // Error teknis lainnya seperti koneksi ke PostgreSQL terputus
                    other_error => {
                        return Err(AppError::DatabaseError(other_error));
                    }
                }
            }
        };
        let message = format!("User '{}' berhasil diperbarui!", q.username);
        Ok((message, q))
    }
   
    pub async fn delete_user(&self, user:User,user_id: &Uuid)-> Result<(String, User), AppError>{
        if user.role != RoleUsers::ADMIN {
            return Err(AppError::Forbidden(format!("Hanya admin yang dapat menghapus user!")));
        }
        let q =  self.user_repo.delete(user_id).await?;
        let message = format!("User '{}' berhasil dihapus!", q.username);

        Ok((message, q))
    }
    
    pub async fn reset_password_user(&self, user:User,user_id: &Uuid)-> Result<(String, User), AppError>{
        if user.role != RoleUsers::ADMIN {
            return Err(AppError::Forbidden(format!("Hanya admin yang dapat mereset password user!")));
        }
        let user_obj = self.user_repo.find_by_id(user_id).await?;
        let password_hash = generate_async(user_obj.username).await?;
        let q =  self.user_repo.update_password(user_id, password_hash, true).await?;
        let message = format!("Password '{}' berhasil diperbarui dengan password '{}'", q.username, q.username);

        Ok((message, q))
    }

}

impl<S> FromRequestParts<S> for UserService<UserRepository>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let user_repo = UserRepository::new(state.database.clone());
        
        Ok(UserService::new(user_repo, (*state.app_config).clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockUserRepoTrait; // Sesuaikan dengan lokasi mock Anda
    use crate::models::user::{RoleUsers, User, UserReq, UserUpdate};
    use chrono::Utc;
    use mockall::predicate::*;
    use uuid::Uuid;
    use tower_http::cors::CorsLayer;

    // ==========================================
    // HELPER FUNCTIONS
    // ==========================================
    fn create_dummy_config() -> AppConfig {
        AppConfig {
            secret: "rahasia_123".to_string(),
            access_ttl: 1,
            refresh_ttl: 24,
            cors: CorsLayer::new(),
        }
    }

    fn create_dummy_admin() -> User {
        User {
            id: Uuid::new_v4(),
            username: "admin_super".to_string(),
            email: "admin@kampus.ac.id".to_string(),
            password: "hashed_password".to_string(),
            name: "Admin Utama".to_string(),
            role: RoleUsers::ADMIN,
            institute_id: Some(1),
            is_banned: false,
            must_change_password: false,
            created_at: Utc::now(),
        }
    }

    fn create_dummy_auditee() -> User {
        let mut user = create_dummy_admin();
        user.username = "dosen_biasa".to_string();
        user.role = RoleUsers::AUDITEE; // Role selain ADMIN
        user
    }

    // ==========================================
    // TEST CASES: ADD USER
    // ==========================================
    #[tokio::test]
    async fn test_add_user_success_by_admin() {
        let mut mock_repo = MockUserRepoTrait::new();
        let admin = create_dummy_admin();
        let mut new_user_mock = admin.clone();
        new_user_mock.username = "mahasiswa1".to_string();

        let req_data = UserReq {
            username: "mahasiswa1".to_string(),
            email: "mahasiswa@teknohole.com".to_string(),
            name: "mahasiswa satu".to_string(),
            institute_id: None,
            role: RoleUsers::AUDITEE
        };

        // Ekspektasi: repo.create dipanggil 1 kali
        mock_repo
            .expect_create()
            .times(1)
            .returning(move |_,_| Ok(new_user_mock.clone()));

        let service = UserService::new(mock_repo, create_dummy_config());
        let result = service.add_user(admin, req_data).await;

        assert!(result.is_ok());
        let (msg, user) = result.unwrap();
        assert!(msg.contains("berhasil ditambahkan"));
        assert_eq!(user.username, "mahasiswa1");
    }

    #[tokio::test]
    async fn test_add_user_forbidden_non_admin() {
        let mut mock_repo = MockUserRepoTrait::new();
        let auditee = create_dummy_auditee();
        let req_data = UserReq { 
            username: "mahasiswa1".to_string(),
            email: "mahasiswa@teknohole.com".to_string(),
            name: "mahasiswa satu".to_string(),
            institute_id: None,
            role: RoleUsers::AUDITEE
        };

        // Pastikan database TIDAK PERNAH disentuh
        mock_repo.expect_create().times(0);

        let service = UserService::new(mock_repo, create_dummy_config());
        let result = service.add_user(auditee, req_data).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Forbidden(msg) => assert_eq!(msg, "Hanya admin yang dapat menambahkan user!"),
            _ => panic!("Expected Forbidden error!"),
        }
    }

    // ==========================================
    // TEST CASES: EDIT USER
    // ==========================================
    #[tokio::test]
    async fn test_edit_user_success_by_admin() {
        let mut mock_repo = MockUserRepoTrait::new();
        let admin = create_dummy_admin();
        let target_id = Uuid::new_v4();
        
        let mut updated_user = admin.clone();
        updated_user.id = target_id;
        updated_user.username = "username_baru".to_string();

        let req_data = UserUpdate {
            username: Some("username_baru".to_string()),
            email: None,
            name: None,
            institute_id: None,
            role: None,
            is_banned: None,
        };

        // Ekspektasi: update dipanggil 1 kali.
        // eq() untuk ID, dan always() untuk data UserUpdate agar terhindar 
        // dari error kompilasi jika UserUpdate tidak memiliki #[derive(PartialEq)]
        mock_repo
            .expect_update()
            .with(eq(target_id), always()) 
            .times(1)
            .returning(move |_, _| Ok(updated_user.clone()));

        let service = UserService::new(mock_repo, create_dummy_config());
        let result = service.edit_user(admin, &target_id, req_data).await;

        assert!(result.is_ok());
        let (msg, user) = result.unwrap();
        assert!(msg.contains("berhasil diperbarui"));
        assert_eq!(user.username, "username_baru");
    }

    #[tokio::test]
    async fn test_edit_user_forbidden_non_admin() {
        let mut mock_repo = MockUserRepoTrait::new();
        let auditee = create_dummy_auditee();
        let target_id = Uuid::new_v4();

        let req_data = UserUpdate {
            username: Some("hacker_coba_ubah".to_string()),
            email: None,
            name: None,
            institute_id: None,
            role: None,
            is_banned: None,
        };

        // Pastikan database TIDAK PERNAH disentuh
        mock_repo.expect_update().times(0);

        let service = UserService::new(mock_repo, create_dummy_config());
        let result = service.edit_user(auditee, &target_id, req_data).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Forbidden(msg) => assert_eq!(msg, "Hanya admin yang dapat mengedit user!"),
            _ => panic!("Expected Forbidden error!"),
        }
    }

    // ==========================================
    // TEST CASES: DELETE USER
    // ==========================================
    #[tokio::test]
    async fn test_delete_user_success_by_admin() {
        let mut mock_repo = MockUserRepoTrait::new();
        let admin = create_dummy_admin();
        let target_id = Uuid::new_v4();
        
        let mut deleted_user = admin.clone();
        deleted_user.username = "user_dihapus".to_string();

        mock_repo
            .expect_delete()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(deleted_user.clone()));

        let service = UserService::new(mock_repo, create_dummy_config());
        let result = service.delete_user(admin, &target_id).await;

        assert!(result.is_ok());
        let (msg, user) = result.unwrap();
        assert!(msg.contains("berhasil dihapus"));
        assert_eq!(user.username, "user_dihapus");
    }

    #[tokio::test]
    async fn test_delete_user_forbidden_non_admin() {
        let mut mock_repo = MockUserRepoTrait::new();
        let auditee = create_dummy_auditee();
        let target_id = Uuid::new_v4();

        mock_repo.expect_delete().times(0);

        let service = UserService::new(mock_repo, create_dummy_config());
        let result = service.delete_user(auditee, &target_id).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Forbidden(_) => (),
            _ => panic!("Expected Forbidden error!"),
        }
    }

    // ==========================================
    // TEST CASES: RESET PASSWORD
    // ==========================================
    #[tokio::test]
    async fn test_reset_password_success_by_admin() {
        let mut mock_repo = MockUserRepoTrait::new();
        let admin = create_dummy_admin();
        let target_id = Uuid::new_v4();
        
        let mut target_user = create_dummy_admin();
        target_user.id = target_id;
        target_user.username = "dosen_lupa_pass".to_string();

        let returned_user = target_user.clone();

        // 1. Ekspektasi: Cari user berdasarkan ID
        mock_repo
            .expect_find_by_id()
            .with(eq(target_id))
            .times(1)
            .returning(move |_| Ok(target_user.clone()));

        // 2. Ekspektasi: Update password menggunakan username sebagai password default (must_change = true)
        mock_repo
            .expect_update_password()
            .withf(move |id, pass, must_change| {
                id == &target_id && pass == "dosen_lupa_pass" && *must_change == true
            })
            .times(1)
            .returning(move |_, _, _| Ok(returned_user.clone()));

        let service = UserService::new(mock_repo, create_dummy_config());
        let result = service.reset_password_user(admin, &target_id).await;

        assert!(result.is_ok());
        let (msg, _) = result.unwrap();
        assert!(msg.contains("berhasil diperbarui dengan password"));
    }
}