-- Add up migration script here
CREATE TABLE files (
    id UUID PRIMARY KEY,
    filename TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size INTEGER NOT NULL,
    storage_path TEXT NOT NULL,
    uploaded_at TIMESTAMP NOT NULL DEFAULT now()
);
