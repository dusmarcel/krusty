-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    uuid CHAR(32) PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    salt TEXT NOT NULL,
    hash TEXT NOT NULL
)