use async_trait::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder, types::Json};
use uuid::Uuid;

use crate::{domain::repository::CalculationRuleTrait, models::accreditation::{CalculationQuery, CalculationRule, CalculationRuleCreate, CalculationRuleUpdate, RawCalculationRule, SortType}};

pub struct CalculationRuleRepository{
    pool: PgPool,
}

impl CalculationRuleRepository {
    pub fn new(pool: PgPool)-> Self {
        Self { pool }
    }
}

#[async_trait]
impl CalculationRuleTrait for CalculationRuleRepository{
    async fn find_by_id(&self, rule_id: Uuid)-> Result<CalculationRule, sqlx::Error> {
        sqlx::query_as::<_,CalculationRule>(
            r#"
            SELECT * FROM accreditation_calculation_rules WHERE id = $1
            "#
        )
        .bind(rule_id)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn search(&self, query: CalculationQuery)-> Result<(Vec<RawCalculationRule>, i64), sqlx::Error> {
        let limit = query.limit.unwrap_or(10) as i64;
        let page = query.page.unwrap_or(1) as i64;
        let offset = (page - 1) * limit;
        let base_query = r#"
            FROM accreditation_calculation_rules acr 
            INNER JOIN accreditation_indicators ai ON acr.indicator_id = ai.id
            WHERE 1=1
        "#;

        let mut count_qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT COUNT(acr.id) ");
        count_qb.push(base_query);

        let mut data_qb: QueryBuilder<Postgres> = QueryBuilder::new(r#"SELECT acr.*, ai.name AS indicator_name "#);
        data_qb.push(base_query);
        let apply_filters = |qb: &mut QueryBuilder<'_, Postgres>| {
            if let Some(indicator_id) = &query.indicator_id {
                qb.push(" AND acr.indicator_id = ");
                qb.push_bind(indicator_id.clone());
            }
        };

        apply_filters(&mut count_qb);
        apply_filters(&mut data_qb);

        let total_items: i64 = count_qb.build_query_scalar().fetch_one(&self.pool).await?;

        match query.sort {
            Some(SortType::Oldest) => data_qb.push(" ORDER BY acr.created_at ASC"),
            Some(SortType::Newest) | _ => data_qb.push(" ORDER BY acr.created_at DESC"),
        };

        data_qb.push(" LIMIT ");
        data_qb.push_bind(limit);
        data_qb.push(" OFFSET ");
        data_qb.push_bind(offset);

        let data = data_qb.build_query_as::<RawCalculationRule>().fetch_all(&self.pool).await?;

        Ok((data, total_items))
    }
    
    async fn create(&self, data: CalculationRuleCreate)-> Result<CalculationRule, sqlx::Error> {
        sqlx::query_as::<_,CalculationRule>(
            r#"
            INSERT INTO accreditation_calculation_rules(indicator_id, assessment, fullfillment, data_source, type, input_rules, formula, expectation_result, result_format) 
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *
            "#
        )
        .bind(data.indicator_id)
        .bind(data.assessment)
        .bind(data.fullfillment)
        .bind(data.data_source)
        .bind(data.r#type)
        .bind(Json(data.input_rules))
        .bind(data.formula)
        .bind(data.expectation_result)
        .bind(data.result_format)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn update(&self, rule_id: Uuid, data: CalculationRuleUpdate) -> Result<CalculationRule, sqlx::Error> {
        sqlx::query_as::<_, CalculationRule>(
            r#"
            UPDATE accreditation_calculation_rules 
            SET
                indicator_id = COALESCE($1, indicator_id),
                assessment = COALESCE($2, assessment),
                fullfillment = COALESCE($3, fullfillment),
                data_source = COALESCE($4, data_source),
                type = COALESCE($5, type),
                input_rules = COALESCE($6, input_rules),
                formula = COALESCE($7, formula),
                expectation_result = COALESCE($8, expectation_result),
                result_format = COALESCE($9, result_format)
            WHERE id = $10 
            RETURNING *
            "#
        )
        .bind(data.indicator_id)
        .bind(data.assessment)
        .bind(data.fullfillment)
        .bind(data.data_source)
        .bind(data.r#type)
        // .map(Json) jika data.input_rules adalah Option<Vec<InputRule>>
        .bind(data.input_rules.map(Json)) 
        .bind(data.formula)
        .bind(data.expectation_result)
        .bind(data.result_format)
        .bind(rule_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, rule_id: Uuid)-> Result<CalculationRule, sqlx::Error> {
        sqlx::query_as::<_,CalculationRule>(
            r#"
            DELETE FROM accreditation_calculation_rules WHERE id = $1
            "#
        )
        .bind(rule_id)
        .fetch_one(&self.pool)
        .await
    }
}