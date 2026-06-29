-- Add migration script here
ALTER TYPE role_users RENAME VALUE 'asesor' TO 'ASSESSOR';
ALTER TABLE users ALTER COLUMN role SET DEFAULT 'auditee';