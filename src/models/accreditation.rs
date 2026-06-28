use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::{institute::{InstituteNested, StudyProgramNested}, user::UserNested};

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
#[sqlx(type_name = "Evaluation_level", rename_all = "lowercase")]
pub enum EvaluationLevel{
    Univeristy,
    Institute,
    StudyProgram
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "static")]
    Static,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SortType {
    #[serde(rename = "newest")]
    Newest,
    #[serde(rename = "oldest")]
    Oldest,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct InputRule {
    pub var: String,
    pub val: Decimal,
    pub r#type: InputType,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Accreditation{
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub year: i32,
    pub reference: String,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccreditationNested{
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AccreditationCreate{
    pub name: String,
    pub description: String,
    pub year: i32,
    pub reference: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AccreditationUpdate{
    pub name: Option<String>,
    pub description: Option<String>,
    pub year: Option<i32>,
    pub reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Indicator{
    pub id: Uuid,
    pub accreditation_id: Uuid,
    pub number: String,
    pub name: String,
    pub justification: String,
    pub criteria: AccreditationCriteria,
    pub target: QualityTarget,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndicatorNested{
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RawIndicator{
    pub id: Uuid,
    pub accreditation_id: Uuid,
    pub accreditation_name: String,
    pub number: String,
    pub name: String,
    pub justification: String,
    pub criteria: AccreditationCriteria,
    pub target: QualityTarget,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndicatorResponse{
    pub id: Uuid,
    pub accreditation: AccreditationNested,
    pub number: String,
    pub name: String,
    pub justification: String,
    pub criteria: AccreditationCriteria,
    pub target: QualityTarget,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}
impl From<RawIndicator> for IndicatorResponse{
    fn from(row: RawIndicator)-> Self {
        Self{
            accreditation: AccreditationNested{
                id: row.accreditation_id,
                name: row.accreditation_name,
            },
            id: row.id,
            number: row.number,
            name: row.name,
            justification: row.justification,
            criteria: row.criteria,
            target: row.target,
            updated_at: row.updated_at,
            created_at: row.created_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct IndicatorCreate{
    pub accreditation_id: Uuid,
    pub number: String,
    pub name: String,
    pub justification: String,
    pub criteria: AccreditationCriteria,
    pub target: QualityTarget,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct IndicatorUpdate{
    pub accreditation_id: Option<Uuid>,
    pub number: Option<String>,
    pub name: Option<String>,
    pub justification: Option<String>,
    pub criteria: Option<AccreditationCriteria>,
    pub target: Option<QualityTarget>,
}

#[derive(Serialize, Deserialize)]
pub struct IndicatorQuery{
    pub accreditation_id: Option<Uuid>,
    pub criteria: Option<String>,
    pub target: Option<String>,
    pub sort: Option<SortType>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
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
pub struct CalculationNested{
    pub id: Uuid,
    pub formula: String,
    pub expectation_result: Decimal,
    pub result_format: ResultFormat,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RawCalculationRule{
    pub id: Uuid,
    pub indicator_id: Uuid,
    pub indicator_name: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculationResponse{
    pub id: Uuid,
    pub indicator: IndicatorNested,
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

impl From<RawCalculationRule> for CalculationResponse {
    fn from(row: RawCalculationRule)-> Self {
        Self{
            indicator: IndicatorNested { 
                id: row.indicator_id, 
                name: row.indicator_name
            },
            id: row.id,
            assessment: row.assessment,
            fullfillment: row.fullfillment,
            data_source: row.data_source,
            r#type: row.r#type,
            input_rules: row.input_rules,
            formula: row.formula,
            expectation_result: row.expectation_result,
            result_format: row.result_format,
            updated_at: row.updated_at,
            created_at: row.created_at,
        }
    }
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

#[derive(Serialize, Deserialize)]
pub struct CalculationQuery{
    pub indicator_id: Option<Uuid>,
    pub sort: Option<SortType>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Evaluation{
    pub id: Uuid,
    pub rule_id: Uuid,
    pub level: EvaluationLevel,
    pub user_id: Uuid,
    pub institute_id: Option<i32>,
    pub study_program_id: Option<i32>,
    pub input_variables: Value,
    pub calculated_result: Decimal,
    pub proof: String,
    pub proof_required: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RawEvaluation{
    pub id: Uuid,
    pub rule_id: Uuid,
    pub formula: String,
    pub expectation_result: Decimal,
    pub result_format: ResultFormat,
    pub level: EvaluationLevel,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub institute_id: Option<i32>,
    pub institute_name: Option<String>,
    pub study_program_id: Option<i32>,
    pub study_program_name: Option<String>,
    pub input_variables: Value,
    pub calculated_result: Decimal,
    pub proof: String,
    pub proof_required: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluationResponse{
    pub id: Uuid,
    pub level: EvaluationLevel,
    pub user: UserNested,
    pub institute: Option<InstituteNested>,
    pub study_program: Option<StudyProgramNested>,
    pub calculation_rule: CalculationNested,
    pub input_variables: Value,
    pub calculated_result: Decimal,
    pub proof: String,
    pub proof_required: bool,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>
}

impl From<RawEvaluation> for EvaluationResponse {
    fn from(row: RawEvaluation)-> Self {
        Self{
            user: UserNested{
                id: row.user_id,
                name: row.user_name,
                email: row.user_email
            },
            institute: row.institute_id
                .zip(row.institute_name)
                .map(|(id, name)| InstituteNested { id, name }),

            study_program: row.study_program_id
                .zip(row.study_program_name)
                .map(|(id, name)| StudyProgramNested { id, name }),
            calculation_rule: CalculationNested{
                id: row.rule_id,
                formula: row.formula,
                expectation_result: row.expectation_result,
                result_format: row.result_format,
            },
            id: row.id,
            level: row.level,
            input_variables: row.input_variables,
            calculated_result: row.calculated_result,
            proof: row.proof,
            proof_required: row.proof_required,
            updated_at: row.updated_at,
            created_at: row.created_at,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EvaluationCreate{
    pub rule_id: Uuid,
    pub level: EvaluationLevel,
    pub institute_id: Option<i32>,
    pub study_program_id: Option<i32>,
    pub input_variables: Vec<InputRule>,
    pub proof: String,
    pub proof_required: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EvaluationUpdate{
    pub rule_id: Option<Uuid>,
    pub level: Option<EvaluationLevel>,
    pub institute_id: Option<i32>,
    pub study_program_id: Option<i32>,
    pub input_variables: Option<Vec<InputRule>>,
    pub proof: Option<String>,
    pub proof_required: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct EvaluationQuery{
    pub accreditation_id: Option<Uuid>,
    pub indicator_id: Option<Uuid>,
    pub rule_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub institute_id: Option<i32>,
    pub study_program_id: Option<i32>,
    pub sort: Option<SortType>,
    pub page: Option<u64>,
    pub limit: Option<u64>,
}