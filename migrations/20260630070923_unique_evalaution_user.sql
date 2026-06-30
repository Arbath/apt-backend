-- Add migration script here
-- Satu user hanya boleh mengisi satu evaluasi universitas
CREATE UNIQUE INDEX uq_eval_university
ON accreditation_evaluations(user_id, rule_id)
WHERE level = 'university';

-- Satu user hanya boleh mengisi satu evaluasi fakultas
CREATE UNIQUE INDEX uq_eval_institute
ON accreditation_evaluations(
    user_id,
    rule_id,
    institute_id
)
WHERE level = 'institute';

-- Satu user hanya boleh mengisi satu evaluasi prodi
CREATE UNIQUE INDEX uq_eval_study_program
ON accreditation_evaluations(
    user_id,
    rule_id,
    institute_id,
    study_program_id
)
WHERE level = 'study_program';