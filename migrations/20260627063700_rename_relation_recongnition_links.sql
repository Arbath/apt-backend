-- Add migration script here
BEGIN;

ALTER TABLE lecturer_recognitions 
ALTER COLUMN link_id DROP NOT NULL;

ALTER TABLE lecturer_recognitions 
DROP CONSTRAINT lecturer_recognitions_link_id_fkey;

ALTER TABLE lecturer_recognitions 
ADD CONSTRAINT lecturer_recognitions_link_id_fkey 
FOREIGN KEY (link_id) REFERENCES links(id) ON DELETE SET NULL;

COMMIT;