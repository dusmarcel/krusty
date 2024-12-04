-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name TEXT,
    salt TEXT,
    hash TEXT
)