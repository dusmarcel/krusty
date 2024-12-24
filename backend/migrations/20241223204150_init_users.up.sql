-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    name TEXT,
    preferred_username TEXT UNIQUE NOT NULL,
    summary TEXT,
    private_key TEXT UNIQUE NOT NULL,
    public_key TEXT UNIQUE NOT NULL,
    salt TEXT UNIQUE NOT NULL,
    hash TEXT UNIQUE NOT NULL
)
