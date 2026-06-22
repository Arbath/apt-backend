use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::repository::{LinkTrait, LogActivityTrait}, models::feature::{Link, LinkCreate, LinkUpdate, LogActivity}};

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
            SELECT l.*, u.username AS owner_username 
            FROM links l
            INNER JOIN users u ON l.owner_id = u.id
            WHERE l.id = $1
            "#
        )
        .bind(link_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_by_slug(&self, slug: String) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            SELECT l.*, u.username AS owner_username 
            FROM links l
            INNER JOIN users u ON l.owner_id = u.id
            WHERE l.slug = $1
            "#
        )
        .bind(slug)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_all_user(&self, user_id: Uuid) -> Result<Vec<Link>, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            SELECT l.*, u.username AS owner_username 
            FROM links l
            INNER JOIN users u ON l.owner_id = u.id
            WHERE l.owner_id = $1
            ORDER BY l.created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn create(&self,owner_id: Uuid, data: LinkCreate) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            WITH new_link AS (
                INSERT INTO links (name, slug, description, is_active, owner_id, started_at, ended_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            )
            SELECT nl.*, u.username AS owner_username 
            FROM new_link nl
            INNER JOIN users u ON nl.owner_id = u.id
            "#
        )
        .bind(data.name)
        .bind(data.slug)
        .bind(data.description)
        .bind(data.is_active)
        .bind(owner_id)
        .bind(data.started_at)
        .bind(data.ended_at)
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, link_id: Uuid, data: LinkUpdate) -> Result<Link, sqlx::Error> {
        sqlx::query_as::<_, Link>(
            r#"
            WITH updated_link AS (
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
            )
            SELECT ul.*, u.username AS owner_username 
            FROM updated_link ul
            INNER JOIN users u ON ul.owner_id = u.id
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
            WITH deleted_link AS (
                DELETE FROM links WHERE id = $1 RETURNING *
            )
            SELECT dl.*, u.username AS owner_username 
            FROM deleted_link dl
            INNER JOIN users u ON dl.owner_id = u.id
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
    async fn create(&self, user_id: Uuid, activity: String) -> Result<LogActivity, sqlx::Error> {
        sqlx::query_as::<_, LogActivity>(
            r#"
            WITH new_log AS (
                INSERT INTO log_activities (user_id, activity)
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
                DELETE FROM log_activities WHERE id = $1 RETURNING *
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
}