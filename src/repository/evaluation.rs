use async_trait::async_trait;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, QueryBuilder, types::Json};
use uuid::Uuid;

use crate::{domain::repository::EvaluationTrait, models::accreditation::{EvaluationQuery, Evaluation, EvaluationCreate, EvaluationUpdate, RawEvaluation, SortType}};

pub struct EvaluationRepository{
    pool: PgPool,
}

impl EvaluationRepository {
    pub fn new(pool: PgPool)-> Self {
        Self { pool }
    }
}

#[async_trait]
impl EvaluationTrait for EvaluationRepository{
    async fn find_by_id(&self, evaluation_id: Uuid)-> Result<Evaluation, sqlx::Error> {
        sqlx::query_as::<_,Evaluation>(
            r#"
            SELECT * FROM accreditation_evaluations WHERE id = $1
            "#
        )
        .bind(evaluation_id)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn search(&self, query: EvaluationQuery)-> Result<(Vec<RawEvaluation>, i64), sqlx::Error> {
        let limit = query.limit.unwrap_or(10) as i64;
        let page = query.page.unwrap_or(1) as i64;
        let offset = (page - 1) * limit;
        let base_query = r#"
            FROM accreditation_evaluations ae 
            INNER JOIN users u ON ae.user_id = u.id
            INNER JOIN accreditation_calculation_rules acr ON ae.rule_id = acr.id
            INNER JOIN accreditation_indicators i ON acr.indicator_id = i.id
            INNER JOIN accreditations a ON i.accreditation_id = a.id
            LEFT JOIN institutes inst ON ae.institute_id = inst.id
            LEFT JOIN study_programs sp ON ae.study_program_id = sp.id
            WHERE 1=1
        "#;

        let mut count_qb: QueryBuilder<Postgres> = QueryBuilder::new("SELECT COUNT(ae.id) ");
        count_qb.push(base_query);

        let mut data_qb: QueryBuilder<Postgres> = QueryBuilder::new(r#"
            SELECT 
                ae.*, 
                acr.formula AS formula,
                acr.expectation_result AS expectation_result,
                acr.result_format AS result_format,
                u.name AS user_name,
                u.email AS user_email,
                ae.institute_id AS institute_id, 
                COALESCE(inst.name, '') AS institute_name,
                COALESCE(sp.name, '') AS study_program_name
        "#);

        data_qb.push(base_query);
        let apply_filters = |qb: &mut QueryBuilder<'_, Postgres>| {
            if let Some(accreditation_id) = &query.accreditation_id {
                qb.push(" AND i.accreditation_id = ");
                qb.push_bind(accreditation_id.clone());
            }
            if let Some(indicator_id) = &query.indicator_id {
                qb.push(" AND acr.indicator_id = ");
                qb.push_bind(indicator_id.clone());
            }
            if let Some(rule_id) = &query.rule_id {
                qb.push(" AND ae.rule_id = ");
                qb.push_bind(rule_id.clone());
            }
            if let Some(user_id) = &query.user_id {
                qb.push(" AND ae.user_id = ");
                qb.push_bind(user_id.clone());
            }
            if let Some(institute_id) = &query.institute_id {
                qb.push(" AND ae.institute_id = ");
                qb.push_bind(institute_id.clone());
            }
            if let Some(study_program_id) = &query.study_program_id {
                qb.push(" AND ae.study_program_id = ");
                qb.push_bind(study_program_id.clone());
            }
        };

        apply_filters(&mut count_qb);
        apply_filters(&mut data_qb);

        let total_items: i64 = count_qb.build_query_scalar().fetch_one(&self.pool).await?;

        match query.sort {
            Some(SortType::Oldest) => data_qb.push(" ORDER BY ae.created_at ASC"),
            Some(SortType::Newest) | _ => data_qb.push(" ORDER BY ae.created_at DESC"),
        };

        data_qb.push(" LIMIT ");
        data_qb.push_bind(limit);
        data_qb.push(" OFFSET ");
        data_qb.push_bind(offset);

        let data = data_qb.build_query_as::<RawEvaluation>().fetch_all(&self.pool).await?;

        Ok((data, total_items))
    }
    
    async fn create(&self, user_id: Uuid, calculated_result: Decimal, score: Decimal, data: EvaluationCreate)-> Result<Evaluation, sqlx::Error> {
        sqlx::query_as::<_,Evaluation>(
            r#"
            INSERT INTO accreditation_evaluations(rule_id, user_id, level, institute_id, study_program_id, input_variables, proof, calculated_result, score) 
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *
            "#
        )
        .bind(data.rule_id)
        .bind(user_id)
        .bind(data.level)
        .bind(data.institute_id)
        .bind(data.study_program_id)
        .bind(Json(data.input_variables))
        .bind(data.proof)
        .bind(calculated_result)
        .bind(score)
        .fetch_one(&self.pool)
        .await
    }
    
    async fn update(&self, evaluation_id: Uuid, calculated_result: Decimal, score: Decimal, data: EvaluationUpdate) -> Result<Evaluation, sqlx::Error> {
        sqlx::query_as::<_, Evaluation>(
            r#"
            UPDATE accreditation_evaluations 
            SET
                rule_id = COALESCE($1, rule_id),
                level = COALESCE($2, level),
                institute_id = COALESCE($3, institute_id),
                study_program_id = COALESCE($4, study_program_id),
                input_variables = COALESCE($5, input_variables),
                proof = COALESCE($6, proof),
                calculated_result = COALESCE($7, calculated_result),
                score = COALESCE($8, score)
            WHERE id = $9
            RETURNING *
            "#
        )
        .bind(data.rule_id)
        .bind(data.level)
        .bind(data.institute_id)
        .bind(data.study_program_id)
        .bind(data.input_variables.map(Json))
        .bind(data.proof)
        .bind(calculated_result)
        .bind(score)
        .bind(evaluation_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, evaluation_id: Uuid)-> Result<Evaluation, sqlx::Error> {
        sqlx::query_as::<_,Evaluation>(
            r#"
            DELETE FROM accreditation_evaluations WHERE id = $1 RETURNING *
            "#
        )
        .bind(evaluation_id)
        .fetch_one(&self.pool)
        .await
    }
}