use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::models::{accreditation::{Accreditation, AccreditationCreate, AccreditationStatistics, AccreditationUpdate, CalculationQuery, CalculationRule, CalculationRuleCreate, CalculationRuleUpdate, Evaluation, EvaluationCreate, EvaluationQuery, EvaluationUpdate, Indicator, IndicatorCreate, IndicatorQuery, IndicatorStatistics, IndicatorUpdate, RawCalculationRule, RawEvaluation, RawIndicator}, feature::{Link, LinkCreate, LinkUpdate, LogActivity, LogActivityQuery}, institute::{Institute, InstituteCreate, InstituteUpdate, StudyProgram, StudyProgramCreate, StudyProgramUpdate}, lecturer::{ApprovalStatus, Lecturer, LecturerCreate, LecturerQuery, LecturerUpdate}, recognition::{ManyRecognitionLecturer, RecognitionCategory, RecognitionCategoryCreate, RecognitionCategoryUpdate, RecognitionLecturer, RecognitionLecturerCreate, RecognitionLecturerQuery, RecognitionLecturerUpdate}, user::{User, UserReq, UserUpdate}};

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait UserRepoTrait: Send + Sync {
    async fn find_by_id(&self, id: &Uuid) -> Result<User, sqlx::Error>;
    async fn find_by_username(&self, name: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_name_or_email(&self, identifier: &str) -> Result<Option<User>, sqlx::Error>;
    async fn get_all(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn create(&self, data: UserReq, password_hash: String) -> Result<User, sqlx::Error>;
    async fn update(&self, id: &Uuid, data: UserUpdate) -> Result<User, sqlx::Error>;
    async fn update_password(&self, id: &Uuid, password_hash: String, must_change: bool) -> Result<User, sqlx::Error>;
    async fn delete(&self, user_id: &Uuid) -> Result<User, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait TokenRepoTrait: Send + Sync {
    async fn save_token(&self, token: &str, user_id: Uuid, expires_at: DateTime<Utc>) -> Result<(), sqlx::Error>;
    async fn exists(&self, token: &str) -> Result<bool, sqlx::Error>;
    async fn revoke(&self, token: &str) -> Result<(), sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait InstituteTrait: Send + Sync {
    async fn find_by_id(&self, institute_id: i32) -> Result<Institute, sqlx::Error>;
    async fn find_by_name(&self, institute_name: &str, page: i64, limit: i64) -> Result<(Vec<Institute>, u64), sqlx::Error>;
    async fn find_all(&self, page: i64, limit: i64) -> Result<(Vec<Institute>, u64), sqlx::Error>;
    async fn find_all_study_programs(&self, institute_id: i32) -> Result<Vec<StudyProgram>, sqlx::Error>;
    async fn create(&self, data: InstituteCreate) -> Result<Institute, sqlx::Error>;
    async fn update(&self, institute_id: i32, data: InstituteUpdate) -> Result<Institute, sqlx::Error>;
    async fn delete(&self, institute_id: i32) -> Result<Institute, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait StudyProgramTrait: Send + Sync {
    async fn find_by_id(&self, program_id: i32) -> Result<StudyProgram, sqlx::Error>;
    async fn find_by_name(&self, program_name: &str, page: i64, limit: i64) -> Result<(Vec<StudyProgram>, u64), sqlx::Error>;
    async fn find_all(&self, page: i64, limit: i64) -> Result<(Vec<StudyProgram>, u64), sqlx::Error>;
    async fn create(&self, data: StudyProgramCreate) -> Result<StudyProgram, sqlx::Error>;
    async fn update(&self, program_id: i32, data: StudyProgramUpdate) -> Result<StudyProgram, sqlx::Error>;
    async fn delete(&self, program_id: i32) -> Result<StudyProgram, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait LecturerTrait: Send + Sync {
    async fn find_by_id(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error>;
    async fn find_by_nip(&self, lecturer_nip: String) -> Result<Lecturer, sqlx::Error>;
    async fn search(&self, query: LecturerQuery) -> Result<(Vec<Lecturer>, i64), sqlx::Error>;
    async fn create(&self, approval_status: ApprovalStatus, data: LecturerCreate) -> Result<Lecturer, sqlx::Error>;
    async fn update(&self, lecturer_id: Uuid, data: LecturerUpdate) -> Result<Lecturer, sqlx::Error>;
    async fn delete(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error>;
    async fn approve(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error>;
    async fn reject(&self, lecturer_id: Uuid) -> Result<Lecturer, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait RecognitionLecturerTrait: Send + Sync {
    async fn find_by_id(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error>;
    async fn search(&self, query: RecognitionLecturerQuery) -> Result<(Vec<ManyRecognitionLecturer>, i64), sqlx::Error>;
    async fn create(&self, aproval_status: ApprovalStatus, data: RecognitionLecturerCreate) -> Result<RecognitionLecturer, sqlx::Error>;
    async fn update(&self, recognition_id: Uuid, data: RecognitionLecturerUpdate) -> Result<RecognitionLecturer, sqlx::Error>;
    async fn delete(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error>;
    async fn approve(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error>;
    async fn reject(&self, recognition_id: Uuid) -> Result<RecognitionLecturer, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait RecognitionLecturerCatTrait: Send + Sync {
    async fn find_all(&self) -> Result<Vec<RecognitionCategory>, sqlx::Error>;
    async fn find_by_id(&self, category_id: i32) -> Result<RecognitionCategory, sqlx::Error>;
    async fn find_name(&self, category_name: &str) -> Result<Vec<RecognitionCategory>, sqlx::Error>;
    async fn create(&self, data: RecognitionCategoryCreate) -> Result<RecognitionCategory, sqlx::Error>;
    async fn update(&self, category_id: i32, data: RecognitionCategoryUpdate) -> Result<RecognitionCategory, sqlx::Error>;
    async fn delete(&self, category_id: i32) -> Result<RecognitionCategory, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait LinkTrait: Send + Sync {
    async fn find_by_id(&self, link_id: Uuid) -> Result<Link, sqlx::Error>;
    async fn find_by_slug(&self, slug: String) -> Result<Link, sqlx::Error>;
    async fn find_all_by_institute(&self, institute_id: i32) -> Result<Vec<Link>, sqlx::Error>;
    async fn create(&self, study_program_id: i32, data: LinkCreate) -> Result<Link, sqlx::Error>;
    async fn update(&self, link_id: Uuid, data: LinkUpdate) -> Result<Link, sqlx::Error>;
    async fn delete(&self, link_id: Uuid) -> Result<Link, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait LogActivityTrait: Send + Sync {
    async fn find_by_id(&self, log_id: Uuid) -> Result<LogActivity, sqlx::Error>;
    async fn search(&self, query: LogActivityQuery) -> Result<(Vec<LogActivity>, i64), sqlx::Error>;
    async fn create(&self, user_id: Uuid, activity: String) -> Result<LogActivity, sqlx::Error>;
    async fn delete(&self, log_id: Uuid) -> Result<LogActivity, sqlx::Error>;
    async fn delete_older_than_days(&self, days: i64) -> Result<u64, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait AccreditationTrait: Send + Sync {
    async fn find_by_id(&self, accreditation_id: Uuid) -> Result<Accreditation, sqlx::Error>;
    async fn find_all(&self) -> Result<Vec<Accreditation>, sqlx::Error>;
    async fn create(&self, data: AccreditationCreate) -> Result<Accreditation, sqlx::Error>;
    async fn update(&self, accreditation_id: Uuid, data: AccreditationUpdate) -> Result<Accreditation, sqlx::Error>;
    async fn delete(&self, accreditation_id: Uuid) -> Result<Accreditation, sqlx::Error>;
    async fn one_stats(&self, accreditation_id: Uuid) -> Result<AccreditationStatistics, sqlx::Error>;
    async fn all_stats(&self) -> Result<Vec<AccreditationStatistics>, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait IndicatorTrait: Send + Sync {
    async fn find_by_id(&self, indicator_id: Uuid) -> Result<Indicator, sqlx::Error>;
    async fn search(&self, query: IndicatorQuery) -> Result<(Vec<RawIndicator>, i64), sqlx::Error>;
    async fn create(&self, data: IndicatorCreate) -> Result<Indicator, sqlx::Error>;
    async fn update(&self, indicator_id: Uuid, data: IndicatorUpdate) -> Result<Indicator, sqlx::Error>;
    async fn delete(&self, indicator_id: Uuid) -> Result<Indicator, sqlx::Error>;
    async fn one_stats(&self, indicator_id: Uuid) -> Result<IndicatorStatistics, sqlx::Error>;
    async fn all_stats(&self, accreditation_id: Uuid) -> Result<Vec<IndicatorStatistics>, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait CalculationRuleTrait: Send + Sync {
    async fn find_by_id(&self, rule_id: Uuid) -> Result<CalculationRule, sqlx::Error>;
    async fn search(&self, query: CalculationQuery) -> Result<(Vec<RawCalculationRule>, i64), sqlx::Error>;
    async fn create(&self, data: CalculationRuleCreate) -> Result<CalculationRule, sqlx::Error>;
    async fn update(&self, rule_id: Uuid, data: CalculationRuleUpdate) -> Result<CalculationRule, sqlx::Error>;
    async fn delete(&self, rule_id: Uuid) -> Result<CalculationRule, sqlx::Error>;
}

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait]
pub trait EvaluationTrait: Send + Sync {
    async fn find_by_id(&self, evaluation_id: Uuid) -> Result<Evaluation, sqlx::Error>;
    async fn search(&self, query: EvaluationQuery) -> Result<(Vec<RawEvaluation>, i64), sqlx::Error>;
    async fn create(&self, user_id: Uuid, calculated_result: Decimal, score: Decimal, data: EvaluationCreate) -> Result<Evaluation, sqlx::Error>;
    async fn update(&self, evaluation_id: Uuid, calculated_result: Decimal, score: Decimal, data: EvaluationUpdate) -> Result<Evaluation, sqlx::Error>;
    async fn delete(&self, evaluation_id: Uuid) -> Result<Evaluation, sqlx::Error>;
}