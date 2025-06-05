-- Add up migration script here
ALTER TABLE files ALTER COLUMN size TYPE BIGINT;
