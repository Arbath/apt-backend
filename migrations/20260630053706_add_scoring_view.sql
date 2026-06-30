-- Add migration script here
ALTER TABLE accreditation_evaluations
ADD COLUMN score NUMERIC(3,2) NOT NULL DEFAULT 0.00 CHECK (score >= 0 AND score <= 3.00);

-- Indicator Scores per-akreditasi
CREATE OR REPLACE VIEW indicator_scores AS
SELECT
    ai.accreditation_id,
    ai.id AS indicator_id,
    ai.number,
    ai.name,
    ai.criteria,
    ai.target,
    acr.assessment,
    acr.fulfillment,
    ROUND(AVG(ae.score), 2) AS score
FROM accreditation_indicators ai
JOIN accreditation_calculation_rules acr ON acr.indicator_id = ai.id
JOIN accreditation_evaluations ae ON ae.rule_id = acr.id
GROUP BY
    ai.accreditation_id,
    ai.id,
    ai.number,
    ai.name,
    ai.criteria,
    ai.target,
    acr.assessment,
    acr.fulfillment;

-- Criteria Scores per-akreditasi
CREATE OR REPLACE VIEW criteria_scores AS
SELECT
    ai.accreditation_id,
    ai.criteria,
    ROUND(AVG(ae.score), 2) AS score
FROM accreditation_indicators ai
JOIN accreditation_calculation_rules acr ON acr.indicator_id = ai.id
JOIN accreditation_evaluations ae ON ae.rule_id = acr.id
GROUP BY 
    ai.accreditation_id, 
    ai.criteria;

-- Target Scores per-akreditasi
CREATE OR REPLACE VIEW target_scores AS
SELECT
    ai.accreditation_id,
    ai.target,
    ROUND(AVG(ae.score), 2) AS score
FROM accreditation_indicators ai
JOIN accreditation_calculation_rules acr
    ON acr.indicator_id = ai.id
JOIN accreditation_evaluations ae
    ON ae.rule_id = acr.id
GROUP BY 
    ai.accreditation_id, 
    ai.target;

-- Accreditation Scores
CREATE OR REPLACE VIEW accreditation_scores AS
SELECT
    a.id,
    a.name,
    ROUND(AVG(ae.score), 2) AS score
FROM accreditations a
JOIN accreditation_indicators ai ON ai.accreditation_id = a.id
JOIN accreditation_calculation_rules acr ON acr.indicator_id = ai.id
JOIN accreditation_evaluations ae ON ae.rule_id = acr.id
GROUP BY 
    a.id, 
    a.name;

CREATE OR REPLACE VIEW accreditation_statistics AS
SELECT 
    a.id AS accreditation_id,
    a.name AS accreditation_name,
    
    -- Quality Culture (Budaya Mutu)
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'quality_culture' AND ai.target = 'input'), 2), 0.00) AS quality_culture_input,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'quality_culture' AND ai.target = 'process'), 2), 0.00) AS quality_culture_process,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'quality_culture' AND ai.target = 'output'), 2), 0.00) AS quality_culture_output,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'quality_culture' AND ai.target = 'impact'), 2), 0.00) AS quality_culture_impact,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'quality_culture'), 2), 0.00) AS quality_culture_total,

    -- Education Relevance (Relevansi Pendidikan)
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'education_relevance' AND ai.target = 'input'), 2), 0.00) AS education_relevance_input,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'education_relevance' AND ai.target = 'process'), 2), 0.00) AS education_relevance_process,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'education_relevance' AND ai.target = 'output'), 2), 0.00) AS education_relevance_output,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'education_relevance' AND ai.target = 'impact'), 2), 0.00) AS education_relevance_impact,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'education_relevance'), 2), 0.00) AS education_relevance_total,

    -- Research Relevance (Relevansi Penelitian)
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'research_relevance' AND ai.target = 'input'), 2), 0.00) AS research_relevance_input,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'research_relevance' AND ai.target = 'process'), 2), 0.00) AS research_relevance_process,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'research_relevance' AND ai.target = 'output'), 2), 0.00) AS research_relevance_output,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'research_relevance' AND ai.target = 'impact'), 2), 0.00) AS research_relevance_impact,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'research_relevance'), 2), 0.00) AS research_relevance_total,

    -- Community Service Relevance (Relevansi Pengabdian)
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'community_service_relevance' AND ai.target = 'input'), 2), 0.00) AS community_service_relevance_input,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'community_service_relevance' AND ai.target = 'process'), 2), 0.00) AS community_service_relevance_process,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'community_service_relevance' AND ai.target = 'output'), 2), 0.00) AS community_service_relevance_output,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'community_service_relevance' AND ai.target = 'impact'), 2), 0.00) AS community_service_relevance_impact,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'community_service_relevance'), 2), 0.00) AS community_service_relevance_total,

    -- Accountability (Akuntabilitas)
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'accountability' AND ai.target = 'input'), 2), 0.00) AS accountability_input,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'accountability' AND ai.target = 'process'), 2), 0.00) AS accountability_process,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'accountability' AND ai.target = 'output'), 2), 0.00) AS accountability_output,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'accountability' AND ai.target = 'impact'), 2), 0.00) AS accountability_impact,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'accountability'), 2), 0.00) AS accountability_total,

    -- Total Akreditasi
    COALESCE(ROUND(AVG(ae.score), 2), 0.00) AS accreditation_total

FROM accreditations a
JOIN accreditation_indicators ai 
    ON ai.accreditation_id = a.id
JOIN accreditation_calculation_rules acr 
    ON acr.indicator_id = ai.id
JOIN accreditation_evaluations ae 
    ON ae.rule_id = acr.id

GROUP BY 
    a.id, 
    a.name;