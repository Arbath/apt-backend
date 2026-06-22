-- Add migration script here
CREATE TABLE lecturers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    nip VARCHAR(25) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    status approval_status NOT NULL DEFAULT 'pending',
    study_program_id INTEGER NOT NULL REFERENCES study_programs(id) ON DELETE SET NULL
);

CREATE TABLE lecturer_recognition_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE lecturer_recognitions(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    lecturer UUID NOT NULL REFERENCES lecturers(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES lecturer_recognition_categories(id) ON DELETE CASCADE,
    description TEXT,
    proof_links JSONB NOT NULL DEFAULT '[]'::jsonb,
    obtained_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status approval_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW()
);