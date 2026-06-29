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

CREATE TABLE accreditations(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    year INTEGER NOT NULL,
    reference TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE accreditation_indicators (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    accreditation_id UUID NOT NULL REFERENCES accreditations(id) ON DELETE CASCADE,
    number VARCHAR(50) NOT NULL,
    name TEXT NOT NULL,
    criteria accreditation_criteria NOT NULL,
    target quality_target NOT NULL,
    justification TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE accreditation_calculation_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    indicator_id UUID NOT NULL REFERENCES accreditation_indicators(id) ON DELETE CASCADE,
    assessment TEXT NOT NULL,
    fulfillment TEXT NOT NULL,
    data_source VARCHAR(255) NOT NULL,
    type calculation_type NOT NULL,
    input_rules JSONB NOT NULL DEFAULT '[]'::jsonb, 
    formula TEXT, 
    expectation_result NUMERIC(10,2) DEFAULT 0.00,
    result_format result_format NOT NULL,
    proof_required BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE accreditation_evaluations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    rule_id UUID NOT NULL REFERENCES accreditation_calculation_rules(id) ON DELETE CASCADE,
    level evaluation_level NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    institute_id INTEGER REFERENCES institutes(id) ON DELETE CASCADE,
    study_program_id INTEGER REFERENCES study_programs(id) ON DELETE CASCADE,
    input_variables JSONB NOT NULL DEFAULT '[]'::jsonb, 
    calculated_result NUMERIC(10,2) DEFAULT 0.00, 
    proof TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

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
CREATE INDEX idx_accreditation_target ON accreditation_indicators(target);
CREATE INDEX idx_accreditation_number ON accreditation_indicators(number);

-- Function untuk mengubah updated_at secara otomatis
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger untuk tabel accreditation_indicators
CREATE TRIGGER trg_accreditation_indicators_updated_at
BEFORE UPDATE ON accreditation_indicators
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Trigger untuk tabel accreditation_calculation_rules
CREATE TRIGGER trg_accreditation_calculation_rules_updated_at
BEFORE UPDATE ON accreditation_calculation_rules
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Trigger untuk tabel accreditation_evaluations
CREATE TRIGGER trg_accreditation_evaluations_updated_at
BEFORE UPDATE ON accreditation_evaluations
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();