use async_trait::async_trait;
use sqlx::PgPool;

use crate::{domain::repository::InstituteTrait, models::institute::{Institute, InstituteCreate, InstituteUpdate, StudyProgram}};

pub struct InstituteRepository {
    pool: PgPool
}

impl InstituteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InstituteTrait for InstituteRepository {
    async fn find_by_id(&self, institute_id: i32) -> Result<Institute, sqlx::Error> {
        sqlx::query_as::<_, Institute> (
            r#"SELECT * FROM institutes WHERE id = $1"#
        )
        .bind(institute_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_name(&self,institute_name: &str, page: i64, limit: i64) -> Result<(Vec<Institute>, u64), sqlx::Error> {
        let offset = (page - 1) * limit;

        let name = format!("%{}%", institute_name);

        let total_items: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM institutes WHERE name ILIKE $1"#
        )
        .bind(&name)
        .fetch_one(&self.pool)
        .await?;

        let data = sqlx::query_as::<_, Institute>(
            r#"
            SELECT * FROM institutes 
            WHERE name ILIKE $1 
            ORDER BY name ASC 
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(&name)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((data, total_items as u64))
    }

    async fn find_all(&self, page: i64, limit: i64) -> Result<(Vec<Institute>, u64), sqlx::Error> {
        let offset = (page - 1) * limit;

        let total_items: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM institutes"#
        )
        .fetch_one(&self.pool)
        .await?;

        let data = sqlx::query_as::<_, Institute>(
            r#"
            SELECT * FROM institutes 
            ORDER BY name ASC 
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((data, total_items as u64))
    }

    async fn find_all_study_programs(&self, institute_id: i32) -> Result<Vec<StudyProgram>, sqlx::Error> {
        let data = sqlx::query_as::<_, StudyProgram>(
            r#"
            SELECT sp.*, i.name AS institute_name 
            FROM study_programs sp
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE i.id = $1
            ORDER BY sp.name ASC
            "#
        )
        .bind(institute_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(data)
    }

    async fn create(&self, data: InstituteCreate) -> Result<Institute, sqlx::Error> {
        sqlx::query_as::<_, Institute> (
            r#"INSERT INTO institutes(name, description)
            VALUES($1, $2)
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.description)
        .fetch_one(&self.pool)
        .await
    }
    async fn update(&self, institute_id: i32, data: InstituteUpdate) -> Result<Institute, sqlx::Error> {
        sqlx::query_as::<_, Institute> (
            r#"UPDATE institutes
            SET name = COALESCE($1, name), description = COALESCE($2, description)
            WHERE id = $3
            RETURNING *"#
        )
        .bind(data.name)
        .bind(data.description)
        .bind(institute_id)
        .fetch_one(&self.pool)
        .await
    }
    async fn delete(&self, institute_id: i32) -> Result<Institute, sqlx::Error> {
        sqlx::query_as::<_, Institute> (
            r#"DELETE FROM institutes WHERE id = $1 RETURNING * "#
        )
        .bind(institute_id)
        .fetch_one(&self.pool)
        .await
    }
}