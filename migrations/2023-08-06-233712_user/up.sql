CREATE TABLE IF NOT EXISTS users (
    id UUID primary key,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
)