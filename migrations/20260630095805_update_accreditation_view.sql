-- Add migration script here
DROP VIEW IF EXISTS accreditation_statistics;
CREATE VIEW accreditation_statistics AS
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

    -- Mission Differentiation (Diferensiasi Misi)
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'mission_differentiation' AND ai.target = 'input'), 2), 0.00) AS mission_differentiation_input,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'mission_differentiation' AND ai.target = 'process'), 2), 0.00) AS mission_differentiation_process,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'mission_differentiation' AND ai.target = 'output'), 2), 0.00) AS mission_differentiation_output,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'mission_differentiation' AND ai.target = 'impact'), 2), 0.00) AS mission_differentiation_impact,
    COALESCE(ROUND(AVG(ae.score) FILTER (WHERE ai.criteria = 'mission_differentiation'), 2), 0.00) AS mission_differentiation_total,

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