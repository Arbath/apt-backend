-- Add migration script here
CREATE TABLE links(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    ended_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    created_at TIMESTAMPTZ DEFAULT NOW(),
);