-- Add up migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid ()
);

CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    owner_id UUID REFERENCES users (id),
    s3_key VARCHAR(255) NOT NULL
);
