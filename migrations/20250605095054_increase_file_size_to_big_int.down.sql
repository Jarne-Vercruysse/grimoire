-- Add down migration script here
ALTER TABLE files ALTER COLUMN size TYPE INTEGER;
