-- This migration adds a default UUID generation to the 'id' column of the 'users' table.
CREATE EXTENSION IF NOT EXISTS pgcrypto;

ALTER TABLE users
ALTER COLUMN id
SET DEFAULT gen_random_uuid();