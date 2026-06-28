use axum::extract::{FromRef, FromRequestParts};
use http::request::Parts;
use uuid::Uuid;

use crate::{domain::repository::{AccreditationTrait, CalculationRuleTrait, EvaluationTrait, IndicatorTrait}, models::accreditation::{Accreditation, AccreditationCreate, AccreditationUpdate, CalculationQuery, CalculationResponse, CalculationRule, CalculationRuleCreate, CalculationRuleUpdate, Evaluation, EvaluationCreate, EvaluationQuery, EvaluationResponse, EvaluationUpdate, Indicator, IndicatorCreate, IndicatorQuery, IndicatorResponse, IndicatorUpdate, InputRule}, repository::{accreditation::AccreditationRepository, calculation::CalculationRuleRepository, evaluation::EvaluationRepository, indicator::IndicatorRepository}, state::AppState, utils::{math::calculate_formula, response::AppError}};

pub struct AccreditationService<A: AccreditationTrait, I: IndicatorTrait, C: CalculationRuleTrait, E: EvaluationTrait> {
    accreditation_repo: A,
    indicator_repo: I,
    calculation_repo: C,
    evaluation_repo: E,
}

impl <A: AccreditationTrait, I: IndicatorTrait, C: CalculationRuleTrait, E: EvaluationTrait> AccreditationService<A, I, C, E> {
    pub fn new(accreditation_repo: A, indicator_repo: I, calculation_repo: C, evaluation_repo: E) -> Self {
        Self { accreditation_repo, indicator_repo, calculation_repo, evaluation_repo }
    }

    // ACCREDITATION
    pub async fn get_accr_detail(&self, accreditation_id: Uuid)-> Result<Accreditation, AppError> {
        let q = self.accreditation_repo.find_by_id(accreditation_id)
            .await.map_err(|_| AppError::NotFound(format!("Akreditasi tidak ditemukan!")))?;
        Ok(q)
    }

    pub async fn get_accr_all(&self)-> Result<Vec<Accreditation>, AppError>{
        let q = self.accreditation_repo.find_all().await?;
        Ok(q)
    }

    pub async fn add_accr(&self, data: AccreditationCreate)-> Result<Accreditation, AppError> {
        let q = self.accreditation_repo.create(data)
            .await.map_err(|_| AppError::NotFound(format!("Akreditasi tidak ditemukan!")))?;
        Ok(q)
    }
    
    pub async fn edit_accr(&self, accreditation_id: Uuid, data: AccreditationUpdate)-> Result<Accreditation, AppError> {
        let q = self.accreditation_repo.update(accreditation_id, data)
            .await.map_err(|_| AppError::NotFound(format!("Akreditasi tidak ditemukan!")))?;
        Ok(q)
    }

    pub async fn remove_accr(&self, accreditation_id: Uuid)-> Result<Accreditation, AppError> {
        let q = self.accreditation_repo.delete(accreditation_id)
            .await.map_err(|_| AppError::NotFound(format!("Akreditasi tidak ditemukan!")))?;
        Ok(q)
    }

    // INDICATOR
    pub async fn get_indicator_detail(&self, indicator_id: Uuid)-> Result<Indicator, AppError>{
        let q = self.indicator_repo.find_by_id(indicator_id)
            .await.map_err(|_| AppError::NotFound(format!("Indikator akreditasi tidak ditemukan!")))?;
        Ok(q)
    }
    
    pub async fn search_indicator(&self, query: IndicatorQuery)-> Result<(Vec<IndicatorResponse>, i64), AppError>{
        let (data, items) = self.indicator_repo.search(query).await?;
        let response: Vec<IndicatorResponse> = data
            .into_iter()
            .map(Into::into)
            .collect();
        Ok((response, items))
    }

    pub async fn add_indicator(&self, data: IndicatorCreate)-> Result<Indicator, AppError>{
        let q = self.indicator_repo.create(data)
            .await.map_err(|_| AppError::NotFound(format!("Indikator akreditasi tidak ditemukan!")))?;
        Ok(q)
    }

    pub async fn edit_indicator(&self, indicator_id: Uuid, data: IndicatorUpdate)-> Result<Indicator, AppError>{
        let q = self.indicator_repo.update(indicator_id, data)
            .await.map_err(|_| AppError::NotFound(format!("Indikator akreditasi tidak ditemukan!")))?;
        Ok(q)
    }

    pub async fn remove_indicator(&self, indicator_id: Uuid)-> Result<Indicator, AppError>{
        let q = self.indicator_repo.delete(indicator_id)
            .await.map_err(|_| AppError::NotFound(format!("Indikator akreditasi tidak ditemukan!")))?;
        Ok(q)
    }

    // CALCULATION
    pub async fn get_calculation_detail(&self, rule_id: Uuid)-> Result<CalculationRule, AppError>{
        let q = self.calculation_repo.find_by_id(rule_id)
            .await.map_err(|_| AppError::NotFound(format!("Kalkulasi indikator tidak ditemukan!")))?;
        Ok(q)
    }

    pub async fn search_calculation(&self, query: CalculationQuery)-> Result<(Vec<CalculationResponse>, i64), AppError>{
        let (data, items) = self.calculation_repo.search(query).await?;
        let response: Vec<CalculationResponse> = data
            .into_iter()
            .map(Into::into)
            .collect();
        Ok((response, items))
    }

    pub async fn add_calculation(&self, data: CalculationRuleCreate)-> Result<CalculationRule, AppError>{
        let q = self.calculation_repo.create(data)
            .await.map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("Kalkulasi indikator tidak ditemukan!")),
                _ => AppError::BadRequest(e.to_string())
            })?;
        Ok(q)
    }

    pub async fn edit_calculation(&self, rule_id: Uuid, data: CalculationRuleUpdate)-> Result<CalculationRule, AppError>{
        let q = self.calculation_repo.update(rule_id, data)
            .await.map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("Kalkulasi indikator tidak ditemukan!")),
                _ => AppError::BadRequest(e.to_string())
            })?;
        Ok(q)
    }

    pub async fn remove_calculation(&self, rule_id: Uuid)-> Result<CalculationRule, AppError>{
        let q = self.calculation_repo.delete(rule_id)
            .await.map_err(|_| AppError::NotFound(format!("Kalkulasi indikator tidak ditemukan!")))?;
        Ok(q)
    }

    // Evaluation
    pub async fn get_evaluation_detail(&self, evaluation_id: Uuid)-> Result<Evaluation, AppError>{
        let q = self.evaluation_repo.find_by_id(evaluation_id)
            .await.map_err(|_| AppError::NotFound(format!("Evaluasi indikator tidak ditemukan!")))?;
        Ok(q)
    }

    pub async fn search_evaluation(&self, query: EvaluationQuery)-> Result<(Vec<EvaluationResponse>, i64), AppError>{
        let (data, items) = self.evaluation_repo.search(query).await?;
        let response: Vec<EvaluationResponse> = data
            .into_iter()
            .map(Into::into)
            .collect();
        Ok((response, items))
    }

    pub async fn add_evaluation(&self, data: EvaluationCreate) -> Result<Evaluation, AppError> {
        let calculation_rule = self.calculation_repo.find_by_id(data.rule_id.clone()).await?;
        let formula = calculation_rule.formula;
        let calculated_result = calculate_formula(&formula, &data.input_variables).await?;

        let q = self.evaluation_repo.create(calculated_result, data)
            .await.map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound("Evaluasi indikator tidak ditemukan!".to_string()),
                _ => AppError::BadRequest(e.to_string())
            })?;
            
        Ok(q)
    }

    pub async fn edit_evaluation(&self, evaluation_id: Uuid, data: EvaluationUpdate) -> Result<Evaluation, AppError> {
        let existing_eval = self.evaluation_repo.find_by_id(evaluation_id).await
            .map_err(|_| AppError::NotFound("Data evaluasi sebelumnya tidak ditemukan".to_string()))?;
        let mut final_calculated_result = existing_eval.calculated_result;
        if data.input_variables.is_some() || data.rule_id.is_some() {
            let final_rule_id = data.rule_id.clone().unwrap_or(existing_eval.rule_id);
            
            let final_input_vars = match &data.input_variables {
                Some(new_vars) => new_vars.clone(),
                None => {
                    serde_json::from_value::<Vec<InputRule>>(existing_eval.input_variables)
                        .map_err(|e| AppError::BadRequest(format!("Gagal membaca variabel lama dari database: {}", e)))?
                }
            };

            let calculation_rule = self.calculation_repo.find_by_id(final_rule_id).await?;
            final_calculated_result = calculate_formula(&calculation_rule.formula, &final_input_vars).await?;
        }
        let q = self.evaluation_repo.update(evaluation_id, final_calculated_result, data)
            .await.map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound("Evaluasi indikator tidak ditemukan!".to_string()),
                _ => AppError::BadRequest(e.to_string())
            })?;
            
        Ok(q)
    }

    pub async fn remove_evaluation(&self, evaluation_id: Uuid)-> Result<Evaluation, AppError>{
        let q = self.evaluation_repo.delete(evaluation_id)
            .await.map_err(|_| AppError::NotFound(format!("Evaluasi indikator tidak ditemukan!")))?;
        Ok(q)
    }
}

impl<S> FromRequestParts<S> for AccreditationService<AccreditationRepository, IndicatorRepository, CalculationRuleRepository, EvaluationRepository>
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        
        let accreditation_repo = AccreditationRepository::new(state.database.clone());
        let indicator_repo = IndicatorRepository::new(state.database.clone());
        let calculation_repo = CalculationRuleRepository::new(state.database.clone());
        let evaluation_repo = EvaluationRepository::new(state.database);
        
        Ok(AccreditationService { accreditation_repo, indicator_repo, calculation_repo, evaluation_repo})
    }
}