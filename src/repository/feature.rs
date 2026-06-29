use async_trait::async_trait;
use chrono::{Duration, Utc};
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{domain::repository::{LinkTrait, LogActivityTrait}, models::feature::{Link, LinkCreate, LinkUpdate, LogActivity, LogActivityQuery}};

// ==========================================
// LINK REPOSITORY
// ==========================================

pub struct LinkRepository {
    pool: PgPool,
}

impl LinkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LinkTrait for LinkRepository {
    async fn find_by_id(&self, link_id: Uuid) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            SELECT * FROM links 
            WHERE id = $1
            "#
        )
        .bind(link_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_slug(&self, slug: String) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            SELECT * FROM links 
            WHERE slug = $1
            "#
        )
        .bind(slug)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_all_by_institute(&self, institute_id: i32) -> Result<Vec<Link>, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            SELECT *
            FROM links 
            WHERE institute_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(institute_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn create(&self, institute_id: i32, data: LinkCreate) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            INSERT INTO links (name, slug, description, is_active, institute_id, started_at, ended_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.slug)
        .bind(data.description)
        .bind(data.is_active)
        .bind(institute_id)
        .bind(data.started_at)
        .bind(data.ended_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, link_id: Uuid, data: LinkUpdate) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            UPDATE links
            SET 
                name = $1,
                slug = $2,
                description = $3,
                is_active = $4,
                started_at = $5,
                ended_at = $6
            WHERE id = $7
            RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.slug)
        .bind(data.description)
        .bind(data.is_active)
        .bind(data.started_at)
        .bind(data.ended_at)
        .bind(link_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, link_id: Uuid) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            DELETE FROM links 
            WHERE id = $1 
            RETURNING *
            "#
        )
        .bind(link_id)
        .fetch_one(&self.pool)
        .await
    }
}

// ==========================================
// LOG ACTIVITY REPOSITORY
// ==========================================

pub struct LogActivityRepository {
    pool: PgPool,
}

impl LogActivityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LogActivityTrait for LogActivityRepository {
    async fn find_by_id(&self, log_id: Uuid) -> Result<LogActivity, sqlx::Error> {
        sqlx::query_as::<_, LogActivity>(
            r#"
            SELECT l.*, u.username AS user_username 
            FROM logs l
            INNER JOIN users u ON l.user_id = u.id
            WHERE l.id = $1
            "#
        )
        .bind(log_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn search(&self, query: LogActivityQuery) -> Result<(Vec<LogActivity>, i64), sqlx::Error> {
        let limit = query.limit.unwrap_or(10) as i64;
        let page = query.page.unwrap_or(1) as i64;
        let offset = (page - 1) * limit;

        let base_query = r#"
            FROM logs l
            INNER JOIN users u ON l.user_id = u.id
            WHERE 1=1
        "#;

        let mut count_qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT COUNT(l.id) ");
        count_qb.push(base_query);

        let mut data_qb: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT l.*, u.username AS user_username "
        );
        data_qb.push(base_query);

        let apply_filters = |qb: &mut QueryBuilder<'_, Postgres>| {
            if let Some(activity) = &query.activity {
                qb.push(" AND l.activity ILIKE ");
                qb.push_bind(format!("%{}%", activity)); 
            }
            if let Some(user_id) = &query.user_id {
                qb.push(" AND l.user_id = ");
                qb.push_bind(user_id.clone());
            }
            if let Some(username) = &query.username {
                qb.push(" AND u.username ILIKE ");
                qb.push_bind(format!("%{}%", username)); 
            }
            if let Some(created_at) = &query.created_at {
                qb.push(" AND l.created_at >= ");
                qb.push_bind(created_at.clone());
            }
        };

        apply_filters(&mut count_qb);
        apply_filters(&mut data_qb);

        let total_items: i64 = count_qb.build_query_scalar().fetch_one(&self.pool).await?;

        data_qb.push(" ORDER BY l.created_at DESC");
        data_qb.push(" LIMIT ");
        data_qb.push_bind(limit);
        data_qb.push(" OFFSET ");
        data_qb.push_bind(offset);

        let data = data_qb.build_query_as::<LogActivity>().fetch_all(&self.pool).await?;

        Ok((data, total_items))
    }

    async fn create(&self, user_id: Uuid, activity: String) -> Result<LogActivity, sqlx::Error> {
        sqlx::query_as::<_, LogActivity>(
            r#"
            WITH new_log AS (
                INSERT INTO logs (user_id, activity)
                VALUES ($1, $2)
                RETURNING *
            )
            SELECT l.*, u.username AS user_username 
            FROM new_log l
            INNER JOIN users u ON l.user_id = u.id
            "#
        )
        .bind(user_id)
        .bind(activity)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, log_id: Uuid) -> Result<LogActivity, sqlx::Error> {
        sqlx::query_as::<_, LogActivity>(
            r#"
            WITH deleted_log AS (
                DELETE FROM logs WHERE id = $1 RETURNING *
            )
            SELECT dl.*, u.username AS user_username 
            FROM deleted_log dl
            INNER JOIN users u ON dl.user_id = u.id
            "#
        )
        .bind(log_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete_older_than_days(&self, days: i64) -> Result<u64, sqlx::Error> {
        let threshold_date = Utc::now() - Duration::try_days(days).unwrap_or_default();

        let result = sqlx::query(
            r#"
            DELETE FROM logs 
            WHERE created_at < $1
            "#
        )
        .bind(threshold_date)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }
}