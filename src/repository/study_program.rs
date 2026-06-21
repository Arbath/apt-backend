use async_trait::async_trait;
use sqlx::PgPool;

use crate::{domain::repository::StudyProgramTrait, models::institute::{StudyProgram, StudyProgramCreate, StudyProgramUpdate}};

pub struct StudyProgramRepository {
    pool: PgPool
}

impl StudyProgramRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StudyProgramTrait for StudyProgramRepository {
    
    async fn find_by_id(&self, program_id: i32) -> Result<StudyProgram, sqlx::Error> {
        sqlx::query_as::<_, StudyProgram>(
            r#"
            SELECT sp.*, i.name AS institute_name 
            FROM study_programs sp
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE sp.id = $1
            "#
        )
        .bind(program_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_name(&self, program_name: &str, page: i64, limit: i64) -> Result<(Vec<StudyProgram>, u64), sqlx::Error> {
        let offset = (page - 1) * limit;
        let name = format!("%{}%", program_name);

        // Pada COUNT, cukup hitung dari tabel aslinya saja agar lebih ringan
        let total_items: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM study_programs WHERE name ILIKE $1"#
        )
        .bind(&name)
        .fetch_one(&self.pool)
        .await?;

        let data = sqlx::query_as::<_, StudyProgram>(
            r#"
            SELECT sp.*, i.name AS institute_name 
            FROM study_programs sp
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE sp.name ILIKE $1 
            ORDER BY sp.name ASC 
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

    async fn find_all(&self, page: i64, limit: i64) -> Result<(Vec<StudyProgram>, u64), sqlx::Error> {
        let offset = (page - 1) * limit;

        let total_items: i64 = sqlx::query_scalar(
            r#"SELECT COUNT(*) FROM study_programs"#
        )
        .fetch_one(&self.pool)
        .await?;

        let data = sqlx::query_as::<_, StudyProgram>(
            r#"
            SELECT sp.*, i.name AS institute_name 
            FROM study_programs sp
            INNER JOIN institutes i ON sp.institute_id = i.id
            ORDER BY sp.name ASC 
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((data, total_items as u64))
    }

    async fn create(&self, data: StudyProgramCreate) -> Result<StudyProgram, sqlx::Error> {
        sqlx::query_as::<_, StudyProgram>(
            r#"
            WITH new_program AS (
                INSERT INTO study_programs(name, description, institute_id)
                VALUES($1, $2, $3)
                RETURNING *
            )
            SELECT sp.*, i.name AS institute_name 
            FROM new_program sp
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(data.name)
        .bind(data.description)
        .bind(data.institute_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, program_id: i32, data: StudyProgramUpdate) -> Result<StudyProgram, sqlx::Error> {
        sqlx::query_as::<_, StudyProgram>(
            r#"
            WITH updated_program AS (
                UPDATE study_programs
                SET 
                    name = COALESCE($1, name), 
                    description = COALESCE($2, description), 
                    institute_id = COALESCE($3, institute_id)
                WHERE id = $4
                RETURNING *
            )
            SELECT sp.*, i.name AS institute_name 
            FROM updated_program sp
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(data.name)
        .bind(data.description)
        .bind(data.institute_id)
        .bind(program_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, program_id: i32) -> Result<StudyProgram, sqlx::Error> {
        sqlx::query_as::<_, StudyProgram>(
            r#"
            WITH deleted_program AS (
                DELETE FROM study_programs WHERE id = $1 RETURNING *
            )
            SELECT sp.*, i.name AS institute_name 
            FROM deleted_program sp
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(program_id)
        .fetch_one(&self.pool)
        .await
    }
}