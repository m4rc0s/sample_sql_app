-- Your SQL goes here
CREATE TABLE IF NOT EXISTS project(
    id UUID primary key,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
)