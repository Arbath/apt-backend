use async_trait::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::{domain::repository::IndicatorTrait, models::accreditation::{IndicatorQuery, Indicator, IndicatorCreate, IndicatorUpdate, RawIndicator, SortType}};

pub struct IndicatorRepository{
    pool: PgPool,
}

impl IndicatorRepository {
    pub fn new(pool: PgPool)-> Self {
        Self { pool }
    }
}

#[async_trait]
impl IndicatorTrait for IndicatorRepository{
    async fn find_by_id(&self, indicator_id: Uuid)-> Result<Indicator, sqlx::Error> {
        sqlx::query_as::<_,Indicator>(
            r#"
            SELECT * FROM accreditation_indicators WHERE id = $1
            "#
        )
        .bind(indicator_id)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn search(&self, query: IndicatorQuery)-> Result<(Vec<RawIndicator>, i64), sqlx::Error> {
        let limit = query.limit.unwrap_or(10) as i64;
        let page = query.page.unwrap_or(1) as i64;
        let offset = (page - 1) * limit;
        let base_query = r#"
            FROM accreditation_indicators ai 
            INNER JOIN accreditations a ON ai.accreditation_id = a.id
            WHERE 1=1
        "#;

        let mut count_qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT COUNT(ai.id) ");
        count_qb.push(base_query);

        let mut data_qb: QueryBuilder<Postgres> = QueryBuilder::new(r#"SELECT ai.*, a.name AS accreditation_name "#);
        data_qb.push(base_query);
        let apply_filters = |qb: &mut QueryBuilder<'_, Postgres>| {
            if let Some(accreditation_id) = &query.accreditation_id {
                qb.push(" AND ai.accreditation_id = ");
                qb.push_bind(accreditation_id.clone());
            }
            if let Some(criteria) = &query.criteria {
                qb.push(" AND ai.criteria = ");
                qb.push_bind(criteria.clone());
            }
            if let Some(target) = &query.target {
                qb.push(" AND ai.target = ");
                qb.push_bind(target.clone());
            }
        };

        apply_filters(&mut count_qb);
        apply_filters(&mut data_qb);

        let total_items: i64 = count_qb.build_query_scalar().fetch_one(&self.pool).await?;

        match query.sort {
            Some(SortType::Oldest) => data_qb.push(" ORDER BY ai.created_at ASC"),
            Some(SortType::Newest) | _ => data_qb.push(" ORDER BY ai.created_at DESC"),
        };

        data_qb.push(" LIMIT ");
        data_qb.push_bind(limit);
        data_qb.push(" OFFSET ");
        data_qb.push_bind(offset);

        let data = data_qb.build_query_as::<RawIndicator>().fetch_all(&self.pool).await?;

        Ok((data, total_items))
    }
    
    async fn create(&self, data: IndicatorCreate)-> Result<Indicator, sqlx::Error> {
        sqlx::query_as::<_,Indicator>(
            r#"
            INSERT INTO accreditation_indicators(accreditation_id, number, name, justification, criteria, target) 
            VALUES($1, $2, $3, $4, $5, $6) RETURNING *
            "#
        )
        .bind(data.accreditation_id)
        .bind(data.number)
        .bind(data.name)
        .bind(data.justification)
        .bind(data.criteria)
        .bind(data.target)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn update(&self, indicator_id: Uuid, data: IndicatorUpdate) -> Result<Indicator, sqlx::Error> {
        sqlx::query_as::<_, Indicator>(
            r#"
            UPDATE accreditation_indicators 
            SET
                accreditation_id = COALESCE($1, accreditation_id),
                number = COALESCE($2, number),
                name = COALESCE($3, name),
                justification = COALESCE($4, justification),
                criteria = COALESCE($5, criteria),
                target = COALESCE($6, target)
            WHERE id = $7
            RETURNING *
            "#
        )
        .bind(data.accreditation_id)
        .bind(data.number)
        .bind(data.name)
        .bind(data.justification)
        .bind(data.criteria)
        .bind(data.target)
        .bind(indicator_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, indicator_id: Uuid)-> Result<Indicator, sqlx::Error> {
        sqlx::query_as::<_,Indicator>(
            r#"
            DELETE FROM accreditation_indicators WHERE id = $1
            "#
        )
        .bind(indicator_id)
        .fetch_one(&self.pool)
        .await
    }
}