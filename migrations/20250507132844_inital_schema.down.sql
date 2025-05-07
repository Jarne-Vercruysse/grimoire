-- Add down migration script here
ALTER TABLE files 
DROP CONSTRAINT IF EXISTS users_id_fkey;

DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS users;
