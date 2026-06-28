use axum::{Router, body::Body, extract::{DefaultBodyLimit, Path}, routing::{get, post}};
use crate::{middleware::auth::AuthUser, state::AppState, utils::response::{ApiError, AppError, WebResponse}};

use axum::{extract::{Multipart}, response::IntoResponse};
use http::{Uri, header};
use tokio::{fs::{self, File}, io::AsyncWriteExt};
use uuid::Uuid;
use serde_json::json;
use tokio_util::io::ReaderStream;
use axum::response::Html;

async fn home() -> Html<&'static str> {
    Html(
        r#"
        <h2>APT Backend Service</h2>
        <p>Version 1.0.0</p>
        <hr>
        <small>
            Engineered with Rust & Axum (2026)
            <br>
            by Arbath Abdurrahman
            <br>
            <a href="https://github.com/Arbath" target="_blank">
                github/Arbath
            </a>
        </small>
        "#
    )
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .route("/api", get(home))
        .route("/api/uploads", post(upload_file_handler)).layer(DefaultBodyLimit::max(100 * 1024 * 1024))
        .route("/api/uploads/{file_user_id}/{filename}", get(get_secure_file_handler))
}

pub async fn upload_file_handler(uri: Uri, AuthUser(user): AuthUser, mut multipart: Multipart) -> Result<impl IntoResponse, ApiError> {
    let upload_dir = "./media/uploads";
    let mut file_url = String::new();

    while let Some(mut field) = multipart.next_field().await.map_err(|e|ApiError { error:AppError::BadRequest(format!("Failed read multipart: {}", e)), path: uri.to_string() })? {
        let field_name = field.name().unwrap_or("").to_string();
        
        if field_name == "document" || field_name == "image" || field_name == "file" {
            let original_name = field.file_name().unwrap_or("unknown.bin").to_string();
            let path = std::path::Path::new(&original_name);
            let extension = std::path::Path::new(&original_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("bin")
                .to_lowercase();
            let file_stem = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("file");

            let allowed_extensions = vec!["jpg", "jpeg", "png", "zip", "pdf", "xlsx", "xls", "doc", "docx", "docs"];
            if !allowed_extensions.contains(&extension.as_str()) {
                return Err(ApiError { error:AppError::BadRequest(format!("File extension not allowed!")), path: uri.to_string() })?
            }

            let unique_filename = format!("{}_{}.{}", file_stem, Uuid::new_v4(), extension);
            
            let user_dir = format!("{}/{}", upload_dir, user.id);
            
            fs::create_dir_all(&user_dir).await.map_err(|e|ApiError { 
                error:AppError::InternalError(format!("Failed create user directory: {}", e)), 
                path: uri.to_string() 
            })?;

            let save_path = format!("{}/{}", user_dir, unique_filename);
            let mut file = fs::File::create(&save_path).await.map_err(|e|ApiError { error:AppError::InternalError(format!("Failed create file: {}", e)), path: uri.to_string() })?;

            while let Some(chunk) = field.chunk().await.map_err(|e|ApiError { error:AppError::BadRequest(format!("Failed read chunk file: {}", e)), path: uri.to_string() })? {
                file.write_all(&chunk).await.map_err(|e|ApiError { error:AppError::InternalError(format!("Failed write file: {}", e)), path: uri.to_string() })?;
            }

            file_url = format!("/api/uploads/{}/{}", user.id, unique_filename);
            
            break; 
        }
    }

    if file_url.is_empty() {
        return Err(ApiError { error:AppError::BadRequest(format!("File is empty!")), path: uri.to_string() })?
    }

    let res = json!({
        "file_url" : file_url
    });
    Ok(WebResponse::ok(&uri, "File uploaded!".to_string(), res))
}

pub async fn get_secure_file_handler(
    uri: Uri,
    AuthUser(_): AuthUser,
    Path((file_user_id, filename)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    
    // Otorisasi Level Pengguna
    // Cek apakah user yang sedang login berhak melihat file di folder ini.
    // Jika hanya admin atau pemilik file yang boleh melihat, validasinya di sini.
    /*
    if user.id.to_string() != file_user_id && user.role != "admin" {
        return Err(ApiError {
            error: AppError::Forbidden("Anda tidak berhak mengakses file ini".to_string()),
            path: uri.to_string(),
        });
    }
    */

    let file_path = format!("./media/uploads/{}/{}", file_user_id, filename);

    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => {
            return Err(ApiError {
                error: AppError::NotFound("File not found or deleted!".to_string()),
                path: uri.to_string(),
            })
        }
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let headers = [(header::CONTENT_TYPE, "application/octet-stream")];

    Ok((headers, body))
}