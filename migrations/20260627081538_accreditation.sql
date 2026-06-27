-- Add migration script here
-- ENUM untuk 5 Pilar Kriteria Akreditasi
CREATE TYPE accreditation_criteria AS ENUM (
    'quality_culture',               -- Budaya Mutu
    'education_relevance',           -- Relevansi Pendidikan
    'research_relevance',            -- Relevansi Penelitian
    'community_service_relevance',   -- Relevansi Pengabdian
    'accountability'                 -- Akuntabilitas
);

-- ENUM untuk 4 Sasaran Mutu
CREATE TYPE quality_target AS ENUM (
    'input',    -- Masukan
    'process',  -- Proses
    'output',   -- Luaran
    'impact'    -- Dampak
);

-- ENUM untuk 2 tipe kalkulasi
CREATE TYPE calculation_type AS ENUM (
    'points',
    'maths'
);

CREATE TYPE result_format AS ENUM (
    'percentage',
    'decimal'
);

CREATE TYPE evaluation_level AS ENUM (
    'university',     -- Level 1: Akreditasi Institusi (APT)
    'institute',      -- Level 2: UPPS / Fakultas / LPM
    'study_program'   -- Level 3: Akreditasi Program Studi (APS)
);

CREATE TABLE accreditation_indicators (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    number VARCHAR(50) NOT NULL,
    name TEXT NOT NULL,
    criteria accreditation_criteria NOT NULL,
    quality quality_target NOT NULL,
    justification TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE accreditation_calculation_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    indicator_id UUID NOT NULL REFERENCES accreditation_indicators(id) ON DELETE CASCADE,
    assessment TEXT NOT NULL,
    fulfillment TEXT NOT NULL,
    data_source VARCHAR(255) NOT NULL,
    type calculation_type NOT NULL,
    input_rules JSONB NOT NULL DEFAULT '{}'::jsonb, 
    formula TEXT, 
    expectation_result NUMERIC(10,2) DEFAULT 0.00,
    result_format result_format NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE accreditation_evaluations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    rule_id UUID NOT NULL REFERENCES accreditation_calculation_rules(id) ON DELETE RESTRICT,
    level evaluation_level NOT NULL,
    institute_id UUID REFERENCES institutes(id) ON DELETE CASCADE,
    study_program_id UUID REFERENCES study_programs(id) ON DELETE CASCADE,

    evaluation_year INTEGER NOT NULL,
    input_variables JSONB NOT NULL DEFAULT '{}'::jsonb, 
    calculated_result NUMERIC(10,2) DEFAULT 0.00, 
    proof_links JSONB NOT NULL DEFAULT '[]'::jsonb, 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT chk_evaluation_hierarchy CHECK (
        -- Jika level Universitas: Borang milik rektorat, tidak terikat Fakultas/Prodi
        (level = 'university' AND institute_id IS NULL AND study_program_id IS NULL) OR
        -- Jika level Fakultas (UPPS): Terikat pada Fakultas (institutes), tapi tidak terikat Prodi
        (level = 'institute' AND institute_id IS NOT NULL AND study_program_id IS NULL) OR
        -- Jika level Prodi: Wajib terikat pada Prodi DAN Fakultas induknya
        (level = 'study_program' AND institute_id IS NOT NULL AND study_program_id IS NOT NULL)
    )
);

-- berdasarkan kriteria, sasaran mutu, atau nomor indikator
CREATE INDEX idx_accreditation_criteria ON accreditation_indicators(criteria);
CREATE INDEX idx_accreditation_quality_target ON accreditation_indicators(quality_target);
CREATE INDEX idx_accreditation_indicator_num ON accreditation_indicators(indicator_number);

CREATE INDEX idx_evals_university ON accreditation_evaluations(level, evaluation_year) WHERE level = 'university';
CREATE INDEX idx_evals_institute ON accreditation_evaluations(institute_id, evaluation_year);
CREATE INDEX idx_evals_study_prog ON accreditation_evaluations(study_program_id, evaluation_year);