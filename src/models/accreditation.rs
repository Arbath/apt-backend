use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "accreditation_criteria", rename_all = "lowercase")]
pub enum AccreditationCriteria{
    QualityCulture,
    EducationRelevance,
    ResearchRelevance,
    ComunityServiceRelevance,
    Accountability
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "quality_target", rename_all = "lowercase")]
pub enum QualityTarget{
    INPUT,
    PROCESS,
    OUTPUT,
    IMPACT,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "calculation_type", rename_all = "lowercase")]
pub enum CalculationType{
    POINTS,
    MATHS,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "result_format", rename_all = "lowercase")]
pub enum ResultFormat{
    PERCENTAGE,
    DECIMAL
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "evalution_level", rename_all = "lowercase")]
pub enum EvalutionLevel{
    Univeristy,
    Institute,
    StudyProgram
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InputType {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "static")]
    Static,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRule {
    pub var: String,
    pub val: Decimal,
    pub r#type: InputType,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Accreditation{
    id: Uuid,
    name: String,
    description: String,
    year: i32,
    reference: String,
    created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AccreditationCreate{
    name: String,
    description: String,
    year: i32,
    reference: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AccreditationUpdate{
    name: Option<String>,
    description: Option<String>,
    year: Option<i32>,
    reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Indicator{
    pub id: Uuid,
    pub accreditation_id: Uuid,
    pub number: String,
    pub name: String,
    pub justification: String,
    pub criteria: AccreditationCriteria,
    pub quality: QualityTarget,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct IndicatorCreate{
    pub accreditation_id: Uuid,
    pub number: String,
    pub name: String,
    pub justification: String,
    pub criteria: AccreditationCriteria,
    pub quality: QualityTarget,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct IndicatorUpdate{
    pub accreditation_id: Option<Uuid>,
    pub number: Option<String>,
    pub name: Option<String>,
    pub justification: Option<String>,
    pub criteria: Option<AccreditationCriteria>,
    pub quality: Option<QualityTarget>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CalculationRule{
    pub id: Uuid,
    pub indicator_id: Uuid,
    pub assessment: String,
    pub fullfillment: String,
    pub data_source: String,
    pub r#type: CalculationType,
    pub input_rules: Value,
    pub formula: String,
    pub expectation_result: Decimal,
    pub result_format: ResultFormat,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CalculationRuleCreate{
    pub indicator_id: Uuid,
    pub assessment: String,
    pub fullfillment: String,
    pub data_source: String,
    pub r#type: CalculationType,
    pub input_rules: Vec<InputRule>,
    pub formula: String,
    pub expectation_result: Decimal,
    pub result_format: ResultFormat,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CalculationRuleUpdate{
    pub indicator_id: Option<Uuid>,
    pub assessment: Option<String>,
    pub fullfillment: Option<String>,
    pub data_source: Option<String>,
    pub r#type: Option<CalculationType>,
    pub input_rules: Option<Vec<InputRule>>,
    pub formula: Option<String>,
    pub expectation_result: Option<Decimal>,
    pub result_format: Option<ResultFormat>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Evalution{
    pub id: Uuid,
    pub rule_id: Uuid,
    pub level: EvalutionLevel,
    pub user_id: Uuid,
    pub insitute_id: i32,
    pub study_program_id: i32,
    pub input_variables: Value,
    pub calculated_result: Decimal,
    pub proof: String,
    pub proof_required: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EvalutionCreate{
    pub rule_id: Uuid,
    pub level: EvalutionLevel,
    pub user_id: Uuid,
    pub insitute_id: i32,
    pub study_program_id: i32,
    pub input_variables: Vec<InputRule>,
    pub calculated_result: Decimal,
    pub proof: String,
    pub proof_required: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EvalutionUpdate{
    pub rule_id: Option<Uuid>,
    pub level: Option<EvalutionLevel>,
    pub user_id: Option<Uuid>,
    pub insitute_id: Option<i32>,
    pub study_program_id: Option<i32>,
    pub input_variables: Option<Vec<InputRule>>,
    pub calculated_result: Option<Decimal>,
    pub proof: Option<String>,
    pub proof_required: Option<bool>,
}