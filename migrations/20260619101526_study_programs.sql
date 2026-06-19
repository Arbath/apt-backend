-- Add migration script here
CREATE TABLE study_programs (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    institute_id INTEGER REFERENCES institutes(id) ON DELETE SET NULL
);