use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::{User, UserUpdate};
use chrono::{DateTime, Utc};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &Uuid) -> Result<User, sqlx::Error> {
        
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_username(
        &self,
        name: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_name_or_email(
        &self,
        identifier: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE name = $1 OR email = $1"
        )
        .bind(identifier)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            ORDER BY id ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create(&self, data: User) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, name, password, email, institute_id, role)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(data.username)
        .bind(data.name)
        .bind(data.password)
        .bind(data.email)
        .bind(data.institute_id)
        .bind(data.role)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update(&self, id: &Uuid, data: UserUpdate) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET username = COALESCE($1, username),
                name = COALESCE($2, name),
                email = COALESCE($3, email),
                institute_id = COALESCE($4, institute_id),
                role = COALESCE($5::role_users, role)
            WHERE id = $6
            RETURNING *
            "#
        )
        .bind(data.username)
        .bind(data.name)
        .bind(data.email)
        .bind(data.institute_id)
        .bind(data.role)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn update_password(&self,id: &Uuid, password: &String, must_change: bool) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET password = $1, must_change_password = $2 WHERE id = $3 RETURNING *
            "#
        )
        .bind(password)
        .bind(must_change)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, user_id: &i32) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            DELETE FROM users
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
    }
}

pub struct TokenRepository {
    pool: PgPool,
}

impl TokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_token(
        &self,
        token: &str,
        user_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (token, user_id, expires_at)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(token)
        .bind(user_id)
        .bind(expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn exists(&self, token: &str) -> Result<bool, sqlx::Error> {
        let exists: Option<i32> = sqlx::query_scalar(
            r#"
            SELECT 1
            FROM refresh_tokens
            WHERE token = $1
            LIMIT 1
            "#
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;

        Ok(exists.is_some())
    }

    pub async fn revoke(&self, token: &str) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM refresh_tokens WHERE token = $1"
        )
        .bind(token)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}