-- Add migration script here
CREATE TABLE logs(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    activity TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);