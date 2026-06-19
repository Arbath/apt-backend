use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Duration, Utc};
use apt_backend::models::auth::Claims;
use apt_backend::models::user::User;
use apt_backend::utils::response::AppError;
use uuid::Uuid;

#[test]
fn test_gen_jwt () {
    let user = User { 
        id: Uuid::now_v7(), 
        username: "Arbath".to_string(),
        name: "Arbath".to_string(), 
        email: "arbath@teknohole.com".to_string(), 
        password: "secret".to_string(), 
        role: apt_backend::models::user::RoleUsers::ADMIN, 
        institute: None,
        is_banned: false,
        must_change_password: false,
        created_at: Utc::now(),};
    let token = gen_access_token(&user);
    println!("{:?}", token);
}

fn gen_access_token(user: &User) -> Result<String, AppError> {
    let now = Utc::now();
    let access_ttl = 60 * 60;
    let access_duration = Duration::seconds(access_ttl as i64);
    let access_expires_at = now + access_duration;
    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp: access_expires_at.timestamp() as usize, 
        iat: now.timestamp() as usize,
        token_type: "access".to_string(),
    };
    let secret = "secret";

    let access_token = encode(
        &Header::default(),&claims,
        &EncodingKey::from_secret(secret.as_bytes())
    ).map_err(|e| AppError::InternalError(e.to_string()))?;

    Ok(access_token)
}
