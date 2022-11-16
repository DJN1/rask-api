CREATE TABLE users (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    username VARCHAR(32) UNIQUE NOT NULL,
    email VARCHAR(320) UNIQUE NOT NULL,
    password TEXT NOT NULL,
    firstname VARCHAR(32) NULL,
    lastname VARCHAR(64) NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NULL
)