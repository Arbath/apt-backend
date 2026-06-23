use async_trait::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{domain::repository::RecognitionLecturerTrait, models::recognition::{ManyRecognitionLecturer, RecognitionLecturer, RecognitionLecturerCreate, RecognitionLecturerQuery, RecognitionLecturerUpdate}};

pub struct RecognitionLecturerRepository {
    pool: PgPool,
}

impl RecognitionLecturerRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RecognitionLecturerTrait for RecognitionLecturerRepository {
    async fn find_by_id(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error> {
        sqlx::query_as::<_, RecognitionLecturer>(
            r#"
            SELECT lr.*, lrc.name AS category_name
            FROM lecturer_recognitions lr
            INNER JOIN lecturer_recognition_categories lrc ON lr.category_id = lrc.id
            WHERE lr.id = $1
            "#
        )
        .bind(recognition_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn search(&self,link_id: Uuid, query: RecognitionLecturerQuery) -> Result<(Vec<ManyRecognitionLecturer>, i64), sqlx::Error> {
        let limit = query.limit.unwrap_or(10) as i64;
        let page = query.page.unwrap_or(1) as i64;
        let offset = (page - 1) * limit;

        // menyatukan JOIN ke tabel lecturers, institutes, sp agar bisa di-filter
        let base_query = r#"
            FROM lecturer_recognitions lr
            INNER JOIN lecturer_recognition_categories lrc ON lr.category_id = lrc.id
            INNER JOIN lecturers l ON lr.lecturer_id = l.id
            INNER JOIN study_programs sp ON l.study_program_id = sp.id
            INNER JOIN institutes i ON sp.institute_id = i.id
            WHERE 1=1
        "#;

        let mut count_qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT COUNT(lr.id) ");
        count_qb.push(base_query);

        let mut data_qb: QueryBuilder<Postgres> = QueryBuilder::new(r#"SELECT lr.*, 
        l.name AS lecturer_name,
        l.nip AS lecturer_nip,
        l.email AS lecturer_email,
        l.status AS lecturer_status,
        sp.id  AS study_prg_id,
        sp.name  AS study_prg_name,
        i.id  AS institute_id,
        i.name  AS institute_name,
        lrc.name AS category_name "#);
        data_qb.push(base_query);

        // Closure pembantu untuk mem-push filter ke kedua QueryBuilder sekaligus
        let apply_filters = |qb: &mut QueryBuilder<'_, Postgres>| {
            qb.push(" AND lr.link_id = ");
            qb.push_bind(link_id);
            if let Some(name) = &query.name {
                qb.push(" AND lr.description ILIKE ");
                qb.push_bind(format!("%{}%", name));
            }
            if let Some(l_name) = &query.lecturer_name {
                qb.push(" AND l.name ILIKE ");
                qb.push_bind(format!("%{}%", l_name));
            }
            if let Some(nip) = &query.lecturer_nip {
                qb.push(" AND l.nip = "); // NIP biasanya exact match, tapi jika ingin ILIKE tinggal diubah
                qb.push_bind(nip.clone());
            }
            if let Some(sp) = &query.study_program {
                qb.push(" AND sp.name ILIKE ");
                qb.push_bind(format!("%{}%", sp));
            }
            if let Some(inst) = &query.institute {
                qb.push(" AND i.name ILIKE ");
                qb.push_bind(format!("%{}%", inst));
            }
            if let Some(cat) = &query.category {
                qb.push(" AND lrc.name ILIKE ");
                qb.push_bind(format!("%{}%", cat));
            }
            if let Some(status) = &query.status {
                qb.push(" AND lr.status = ");
                // Mengkonversi string ke format enum database dengan casting
                qb.push_bind(status.to_lowercase()); 
                qb.push("::approval_status");
            }
        };

        // Aplikasikan filter ke query COUNT dan DATA
        apply_filters(&mut count_qb);
        apply_filters(&mut data_qb);

        let total_items: i64 = count_qb.build_query_scalar().fetch_one(&self.pool).await?;

        match query.sort.as_deref() {
            Some("oldest") => data_qb.push(" ORDER BY lr.created_at ASC"),
            Some("newest") | _ => data_qb.push(" ORDER BY lr.created_at DESC"),
        };

        data_qb.push(" LIMIT ");
        data_qb.push_bind(limit);
        data_qb.push(" OFFSET ");
        data_qb.push_bind(offset);

        let data = data_qb.build_query_as::<ManyRecognitionLecturer>().fetch_all(&self.pool).await?;

        Ok((data, total_items))
    }

    async fn create(&self, data: RecognitionLecturerCreate) -> Result<RecognitionLecturer, sqlx::Error> {
        sqlx::query_as::<_, RecognitionLecturer>(
            r#"
            WITH new_rec AS (
                INSERT INTO lecturer_recognitions (description, proof_links, obtained_at, lecturer_id, category_id, link_id, status)
                VALUES ($1, $2, $3, $4, $5, $6, 'pending'::approval_status)
                RETURNING *
            )
            SELECT nr.*, lrc.name AS category_name
            FROM new_rec nr
            INNER JOIN lecturer_recognition_categories lrc ON nr.category_id = lrc.id
            "#
        )
        .bind(data.description)
        .bind(data.proof_links)
        .bind(data.obtained_at)
        .bind(data.lecturer_id)
        .bind(data.category_id)
        .bind(data.link_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, recognition_id: Uuid, data: RecognitionLecturerUpdate) -> Result<RecognitionLecturer, sqlx::Error> {
        sqlx::query_as::<_, RecognitionLecturer>(
            r#"
            WITH updated_rec AS (
                UPDATE lecturer_recognitions
                SET 
                    description = COALESCE($1, description),
                    proof_links = COALESCE($2, proof_links),
                    obtained_at = COALESCE($3, obtained_at),
                    category_id = COALESCE($4, category_id)
                WHERE id = $5
                RETURNING *
            )
            SELECT ur.*, lrc.name AS category_name
            FROM updated_rec ur
            INNER JOIN lecturer_recognition_categories lrc ON ur.category_id = lrc.id
            "#
        )
        .bind(data.description)
        .bind(data.proof_links)
        .bind(data.obtained_at)
        .bind(data.category_id)
        .bind(recognition_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error> {
        sqlx::query_as::<_, RecognitionLecturer>(
            r#"
            WITH deleted_rec AS (
                DELETE FROM lecturer_recognitions WHERE id = $1 RETURNING *
            )
            SELECT dr.*, lrc.name AS category_name
            FROM deleted_rec dr
            INNER JOIN lecturer_recognition_categories lrc ON dr.category_id = lrc.id
            "#
        )
        .bind(recognition_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn approve(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error> {
        sqlx::query_as::<_, RecognitionLecturer>(
            r#"
            WITH approved_rec AS (
                UPDATE lecturer_recognitions SET status = 'approved'::approval_status WHERE id = $1 RETURNING *
            )
            SELECT ar.*, lrc.name AS category_name
            FROM approved_rec ar
            INNER JOIN lecturer_recognition_categories lrc ON ar.category_id = lrc.id
            "#
        )
        .bind(recognition_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn reject(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error> {
        sqlx::query_as::<_, RecognitionLecturer>(
            r#"
            WITH rejected_rec AS (
                UPDATE lecturer_recognitions SET status = 'rejected'::approval_status WHERE id = $1 RETURNING *
            )
            SELECT rr.*, lrc.name AS category_name
            FROM rejected_rec rr
            INNER JOIN lecturer_recognition_categories lrc ON rr.category_id = lrc.id
            "#
        )
        .bind(recognition_id)
        .fetch_one(&self.pool)
        .await
    }
}