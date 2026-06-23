use async_trait::async_trait;
use sqlx::{PgPool};

use crate::{domain::repository::RecognitionLecturerCatTrait, models::recognition::{RecognitionCategory, RecognitionCategoryCreate, RecognitionCategoryUpdate}};

pub struct RecognitionCategoryRepository {
    pool: PgPool,
}

impl RecognitionCategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecognitionLecturerCatTrait for RecognitionCategoryRepository {
    async fn find_by_id(&self, category_id: i32) -> Result<RecognitionCategory, sqlx::Error> {
        sqlx::query_as::<_, RecognitionCategory>(
            "SELECT * FROM lecturer_recognition_categories WHERE id = $1"
        )
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_name(&self, category_name: &str) -> Result<Vec<RecognitionCategory>, sqlx::Error> {
        let name = format!("%{}%", category_name);
        sqlx::query_as::<_, RecognitionCategory>(
            "SELECT * FROM lecturer_recognition_categories WHERE name ILIKE $1 ORDER BY name ASC"
        )
        .bind(name)
        .fetch_all(&self.pool)
        .await
    }
    
    async fn find_all(&self) -> Result<Vec<RecognitionCategory>, sqlx::Error> {
        sqlx::query_as::<_, RecognitionCategory>(
            "SELECT * FROM lecturer_recognition_categories ORDER BY name ASC"
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn create(&self, data: RecognitionCategoryCreate) -> Result<RecognitionCategory, sqlx::Error> {
        sqlx::query_as::<_, RecognitionCategory>(
            r#"
            INSERT INTO lecturer_recognition_categories (name, description)
            VALUES ($1, $2)
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.description)
        .fetch_one(&self.pool)
        .await
    }

    // Perbaikan: category_id diubah ke i32 sesuai struct model
    async fn update(&self, category_id: i32, data: RecognitionCategoryUpdate) -> Result<RecognitionCategory, sqlx::Error> {
        sqlx::query_as::<_, RecognitionCategory>(
            r#"
            UPDATE lecturer_recognition_categories
            SET 
                name = COALESCE($1, name),
                description = COALESCE($2, description)
            WHERE id = $3
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.description)
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, category_id: i32) -> Result<RecognitionCategory, sqlx::Error> {
        sqlx::query_as::<_, RecognitionCategory>(
            "DELETE FROM lecturer_recognition_categories WHERE id = $1 RETURNING *"
        )
        .bind(category_id)
        .fetch_one(&self.pool)
        .await
    }
}