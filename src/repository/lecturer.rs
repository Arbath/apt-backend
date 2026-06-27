use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::repository::LecturerTrait, models::lecturer::{ApprovalStatus, Lecturer, LecturerCreate, LecturerQuery, LecturerUpdate}};

pub struct LecturerRepository {
    pool: PgPool,
}

impl LecturerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LecturerTrait for LecturerRepository {
    async fn find_by_id(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error> {
        sqlx::query_as::<_, Lecturer>(
            r#"
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM lecturers l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE l.id = $1
            "#
        )
        .bind(lecturer_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_nip(&self, lecturer_nip: String) -> Result<Lecturer, sqlx::Error> {
        sqlx::query_as::<_, Lecturer>(
            r#"
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM lecturers l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE l.nip = $1
            "#
        )
        .bind(lecturer_nip)
        .fetch_one(&self.pool)
        .await
    }

    async fn search(&self, query: LecturerQuery) -> Result<(Vec<Lecturer>, i64), sqlx::Error> {
        let limit = query.limit.unwrap_or(10) as i64;
        let page = query.page.unwrap_or(1) as i64;
        let offset = (page - 1) * limit;

        let name_param = query.name.map(|n| format!("%{}%", n));
        let sp_param = query.study_program.map(|sp| format!("%{}%", sp));
        let inst_param = query.institute.map(|inst| format!("%{}%", inst));
        let status_param = query.status;

        let total_items: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(l.id) 
            FROM lecturers l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE ($1::text IS NULL OR l.name ILIKE $1)
              AND ($2::text IS NULL OR sp.name ILIKE $2)
              AND ($3::text IS NULL OR i.name ILIKE $3)
              AND ($4::text IS NULL OR l.status = $4::approval_status)
            "#
        )
        .bind(&name_param)
        .bind(&sp_param)
        .bind(&inst_param)
        .bind(&status_param)
        .fetch_one(&self.pool)
        .await?;

        let data = sqlx::query_as::<_, Lecturer>(
            r#"
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM lecturers l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE ($1::text IS NULL OR l.name ILIKE $1)
              AND ($2::text IS NULL OR sp.name ILIKE $2)
              AND ($3::text IS NULL OR i.name ILIKE $3)
              AND ($4::text IS NULL OR l.status = $4::approval_status)
            ORDER BY l.name ASC
            LIMIT $5 OFFSET $6
            "#
        )
        .bind(&name_param)
        .bind(&sp_param)
        .bind(&inst_param)
        .bind(&status_param)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok((data, total_items))
    }

    async fn create(&self, approval_status: ApprovalStatus, data: LecturerCreate) -> Result<Lecturer, sqlx::Error> {
        sqlx::query_as::<_, Lecturer>(
            r#"
            WITH new_lecturer AS (
                INSERT INTO lecturers (name, nip, email, study_program_id, status)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            )
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM new_lecturer l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(data.name)
        .bind(data.nip)
        .bind(data.email)
        .bind(data.study_program_id)
        .bind(approval_status)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, lecturer_id: Uuid, data: LecturerUpdate) -> Result<Lecturer, sqlx::Error> {
        sqlx::query_as::<_, Lecturer>(
            r#"
            WITH updated_lecturer AS (
                UPDATE lecturers
                SET 
                    name = COALESCE($1, name),
                    nip = COALESCE($2, nip),
                    email = COALESCE($3, email),
                    study_program_id = COALESCE($4, study_program_id)
                WHERE id = $5
                RETURNING *
            )
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM updated_lecturer l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(data.name)
        .bind(data.nip)
        .bind(data.email)
        .bind(data.study_program_id)
        .bind(lecturer_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error> {
        sqlx::query_as::<_, Lecturer>(
            r#"
            WITH deleted_lecturer AS (
                DELETE FROM lecturers WHERE id = $1 RETURNING *
            )
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM deleted_lecturer l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(lecturer_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn approve(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error> {
        sqlx::query_as::<_, Lecturer>(
            r#"
            WITH updated_lecturer AS (
                UPDATE lecturers SET status = $1 WHERE id = $2 RETURNING *
            )
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM updated_lecturer l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(ApprovalStatus::APPROVED)
        .bind(lecturer_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn reject(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error> {
        sqlx::query_as::<_, Lecturer>(
            r#"
            WITH updated_lecturer AS (
                UPDATE lecturers SET status = $1 WHERE id = $2 RETURNING *
            )
            SELECT 
                l.id, l.name, l.nip, l.email,
                l.status, l.study_program_id,
                sp.name AS study_program_name,
                sp.institute_id,
                i.name AS institute_name
            FROM updated_lecturer l
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            "#
        )
        .bind(ApprovalStatus::REJECTED)
        .bind(lecturer_id)
        .fetch_one(&self.pool)
        .await
    }
}