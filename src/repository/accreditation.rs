use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::repository::AccreditationTrait, models::accreditation::{Accreditation, AccreditationCreate, AccreditationStatistics, AccreditationUpdate}};

pub struct AccreditationRepository {
    pool: PgPool,
}

impl AccreditationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccreditationTrait for AccreditationRepository {
    async fn find_by_id(&self, accreditation_id: Uuid)-> Result<Accreditation, sqlx::Error> {
        sqlx::query_as::<_,Accreditation>(
            r#"
            SELECT * FROM accreditations WHERE id = $1
            "#
        )
        .bind(accreditation_id)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn find_all(&self)-> Result<Vec<Accreditation>, sqlx::Error> {
        sqlx::query_as::<_,Accreditation>(
            r#"
            SELECT * FROM accreditations ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
    }
    
    async fn create(&self, data: AccreditationCreate)-> Result<Accreditation, sqlx::Error> {
        sqlx::query_as::<_,Accreditation>(
            r#"
            INSERT INTO accreditations(name, description, year, reference) VALUES($1, $2, $3, $4) RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.description)
        .bind(data.year)
        .bind(data.reference)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn update(&self, accreditation_id: Uuid, data: AccreditationUpdate)-> Result<Accreditation, sqlx::Error> {
        sqlx::query_as::<_,Accreditation>(
            r#"
            UPDATE accreditations 
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                year = COALESCE($3, year),
                reference = COALESCE($4, reference)
            WHERE id = $5 RETURNING *
            "#
        )
        .bind(data.name)
        .bind(data.description)
        .bind(data.year)
        .bind(data.reference)
        .bind(accreditation_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, accreditation_id: Uuid)-> Result<Accreditation, sqlx::Error> {
        sqlx::query_as::<_,Accreditation>(
            r#"
            DELETE FROM accreditations WHERE id = $1 RETURNING *
            "#
        )
        .bind(accreditation_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn one_stats(&self, accreditation_id: Uuid)-> Result<AccreditationStatistics, sqlx::Error> {
        sqlx::query_as::<_,AccreditationStatistics>(
            r#"
            SELECT * FROM accreditation_statistics WHERE accreditation_id = $1
            "#
        )
        .bind(accreditation_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn all_stats(&self)-> Result<Vec<AccreditationStatistics>, sqlx::Error> {
        sqlx::query_as::<_,AccreditationStatistics>(
            r#"
            SELECT * FROM accreditation_statistics
            "#
        )
        .fetch_all(&self.pool)
        .await
    }
}